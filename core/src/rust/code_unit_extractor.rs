// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::rust::normalizer::normalization_context::NormalizationContext;
use std::path::PathBuf;

use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::code_unit::CodeUnit;
use crate::code_unit_kind::CodeUnitKind;
use crate::fingerprint::Fingerprint;
use crate::node::NormalizedNode;
use crate::rust::parser::has_cfg_test_attr;
use crate::rust::parser::has_test_attr;
use crate::rust::plain_constructor::PlainConstructor;
use syn::ExprClosure;
use syn::ItemFn;
use syn::ItemImpl;
use syn::ItemMod;
use syn::Type;
use syn::visit;

/// Extracts code units from a syn file by visiting the AST.
pub struct CodeUnitExtractor {
    file: PathBuf,
    min_node_count: usize,
    min_line_count: usize,
    units: Vec<CodeUnit>,
    /// Track current impl block context for method naming.
    current_impl: Option<String>,
    /// Track if we're in a trait impl
    in_trait_impl: bool,
    /// Track if we're inside test code (`#[cfg(test)]` module/impl).
    in_test_context: bool,
}

impl CodeUnitExtractor {
    #[must_use]
    pub const fn new(file: PathBuf, min_node_count: usize, min_line_count: usize) -> Self {
        Self {
            file,
            min_node_count,
            min_line_count,
            units: Vec::new(),
            current_impl: None,
            in_trait_impl: false,
            in_test_context: false,
        }
    }

    /// The code units visited so far, in the order they were found.
    #[must_use]
    pub fn into_units(self) -> Vec<CodeUnit> {
        self.units
    }

    #[allow(clippy::too_many_arguments)]
    fn add_unit(
        &mut self,
        kind: CodeUnitKind,
        name: String,
        line_start: usize,
        line_end: usize,
        sig: NormalizedNode,
        body: NormalizedNode,
        is_test: bool,
    ) {
        let node_count = NormalizedNode::count_nodes(&sig) + NormalizedNode::count_nodes(&body);
        if node_count < self.min_node_count {
            return;
        }
        let line_count = line_end.saturating_sub(line_start) + 1;
        if self.min_line_count > 0 && line_count < self.min_line_count {
            return;
        }
        let fingerprint = Fingerprint::from_sig_and_body(&sig, &body);
        self.units.push(CodeUnit {
            kind,
            name,
            file: self.file.clone(),
            line_start,
            line_end,
            signature: sig,
            body,
            fingerprint,
            node_count,
            parent_name: None,
            is_test,
        });
    }

    /// Get a simple string representation of a type for naming.
    fn quote_type(ty: &Type) -> String {
        match ty {
            Type::Path(tp) => tp
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
            _ => "Unknown".to_string(),
        }
    }
}

impl<'ast> Visit<'ast> for CodeUnitExtractor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let is_test =
            self.in_test_context || has_test_attr(&node.attrs) || has_cfg_test_attr(&node.attrs);

        let name = node.sig.ident.to_string();
        let line_start = node.sig.ident.span().start().line;
        let line_end = node.block.brace_token.span.close().end().line;
        let normalized = NormalizationContext::normalize_item_fn(node);
        let sig = normalized.signature;
        let body = normalized.body;
        self.add_unit(
            CodeUnitKind::Function,
            name,
            line_start,
            line_end,
            sig,
            body,
            is_test,
        );

        // Continue visiting nested items (propagate test context)
        let prev = self.in_test_context;
        self.in_test_context = is_test;
        visit::visit_item_fn(self, node);
        self.in_test_context = prev;
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        let prev = self.in_test_context;
        if has_cfg_test_attr(&node.attrs) {
            self.in_test_context = true;
        }
        visit::visit_item_mod(self, node);
        self.in_test_context = prev;
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let prev_test = self.in_test_context;
        if has_cfg_test_attr(&node.attrs) {
            self.in_test_context = true;
        }

        let type_name = Self::quote_type(&node.self_ty);
        let is_trait_impl = node.trait_.is_some();
        let trait_name = node
            .trait_
            .as_ref()
            .map(|(_, path, _)| {
                path.segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::")
            })
            .unwrap_or_default();

        let prev_impl = self.current_impl.take();
        let prev_trait = self.in_trait_impl;

        self.current_impl = Some(type_name.clone());
        self.in_trait_impl = is_trait_impl;

        // Visit each method in the impl block
        for item in &node.items {
            if let syn::ImplItem::Fn(method) = item {
                if !is_trait_impl && PlainConstructor::is_plain(method, &type_name) {
                    continue;
                }
                let method_name = method.sig.ident.to_string();
                let full_name = if is_trait_impl {
                    format!("<{type_name} as {trait_name}>::{method_name}")
                } else {
                    format!("{type_name}::{method_name}")
                };

                let line_start = method.sig.ident.span().start().line;
                let line_end = method.block.brace_token.span.close().end().line;

                let normalized = NormalizationContext::normalize_impl_item_fn(method);

                let sig = normalized.signature;

                let body = normalized.body;
                let kind = if is_trait_impl {
                    CodeUnitKind::TraitImplBlock
                } else {
                    CodeUnitKind::Method
                };

                self.add_unit(
                    kind,
                    full_name,
                    line_start,
                    line_end,
                    sig,
                    body,
                    self.in_test_context,
                );
            }
        }

        self.current_impl = prev_impl;
        self.in_trait_impl = prev_trait;
        self.in_test_context = prev_test;
    }

    fn visit_expr_closure(&mut self, node: &'ast ExprClosure) {
        let line_start = node.or1_token.span.start().line;
        let line_end = match &*node.body {
            syn::Expr::Block(eb) => eb.block.brace_token.span.close().end().line,
            other => {
                let end = other.span().end().line;
                if end > 0 { end } else { line_start }
            }
        };

        let normalized = NormalizationContext::normalize_closure_expr(node);
        let node_count = NormalizedNode::count_nodes(&normalized);
        let line_count = line_end.saturating_sub(line_start) + 1;
        if node_count >= self.min_node_count
            && (self.min_line_count == 0 || line_count >= self.min_line_count)
        {
            let name = format!("closure at {}:{}", self.file.display(), line_start);
            let fingerprint = Fingerprint::from_node(&normalized);
            self.units.push(CodeUnit {
                kind: CodeUnitKind::Closure,
                name,
                file: self.file.clone(),
                line_start,
                line_end,
                signature: NormalizedNode::leaf(crate::node::NodeKind::Opaque),
                body: normalized,
                fingerprint,
                node_count,
                parent_name: None,
                is_test: self.in_test_context,
            });
        }

        // Continue visiting nested closures
        visit::visit_expr_closure(self, node);
    }
}

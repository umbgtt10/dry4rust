// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node::NodeKind;
use crate::node::NormalizedNode;
use crate::node::PlaceholderKind;
use crate::rust::normalizer::normalization_context::NormalizationContext;
use crate::rust::normalizer::normalized_fn::NormalizedFn;

use syn::Block;
use syn::ExprClosure;
use syn::ImplItemFn;
use syn::ItemFn;
use syn::ItemImpl;
use syn::Signature;

impl NormalizationContext {
    pub fn normalize_signature(&mut self, sig: &Signature) -> NormalizedNode {
        // FnSignature -> [return_type_or_None, param0, param1, ...]
        let return_type = match &sig.output {
            syn::ReturnType::Default => NormalizedNode::none(),
            syn::ReturnType::Type(_, ty) => self.normalize_type(ty),
        };
        let mut children = vec![return_type];
        children.extend(sig.inputs.iter().map(|arg| match arg {
            syn::FnArg::Receiver(r) => {
                let idx = self.placeholder("self", PlaceholderKind::Variable);
                let self_node =
                    NormalizedNode::leaf(NodeKind::Placeholder(PlaceholderKind::Variable, idx));
                if r.reference.is_some() {
                    NormalizedNode::with_children(
                        NodeKind::Reference {
                            mutable: r.mutability.is_some(),
                        },
                        vec![self_node],
                    )
                } else {
                    self_node
                }
            }
            syn::FnArg::Typed(pt) => {
                let pat = self.normalize_pat(&pt.pat);
                let ty = self.normalize_type(&pt.ty);
                NormalizedNode::with_children(NodeKind::FieldValue, vec![pat, ty])
            }
        }));
        NormalizedNode::with_children(NodeKind::FnSignature, children)
    }

    fn normalize_fn_parts(sig: &Signature, block: &Block) -> NormalizedFn {
        let mut ctx = Self::new();
        let signature = ctx.normalize_signature(sig);
        let body = ctx.normalize_block(block);
        NormalizedFn::new(signature, body)
    }

    /// Normalize a top-level function.
    #[must_use]
    pub fn normalize_item_fn(func: &ItemFn) -> NormalizedFn {
        Self::normalize_fn_parts(&func.sig, &func.block)
    }

    /// Normalize a method within an impl block.
    #[must_use]
    pub fn normalize_impl_item_fn(method: &ImplItemFn) -> NormalizedFn {
        Self::normalize_fn_parts(&method.sig, &method.block)
    }

    /// Normalize a closure expression.
    #[must_use]
    pub fn normalize_closure_expr(closure: &ExprClosure) -> NormalizedNode {
        let mut ctx = Self::new();
        let mut children = vec![ctx.normalize_expr(&closure.body)];
        children.extend(closure.inputs.iter().map(|p| ctx.normalize_pat(p)));
        NormalizedNode::with_children(NodeKind::Closure, children)
    }

    /// Normalize an impl block -- normalizes each method body.
    #[must_use]
    pub fn normalize_impl_block(imp: &ItemImpl) -> Vec<(String, NormalizedFn)> {
        imp.items
            .iter()
            .filter_map(|item| {
                if let syn::ImplItem::Fn(method) = item {
                    let name = method.sig.ident.to_string();
                    Some((name, Self::normalize_impl_item_fn(method)))
                } else {
                    None
                }
            })
            .collect()
    }
}

// -- Public entry points ------------------------------------------------------

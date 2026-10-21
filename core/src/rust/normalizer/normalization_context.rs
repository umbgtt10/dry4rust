// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node::BinOpKind;
use crate::node::LiteralKind;
use crate::node::NodeKind;
use crate::node::NormalizedNode;
use crate::node::PlaceholderKind;
use crate::node::UnOpKind;
use crate::rust::normalizer::normalized_fn::NormalizedFn;
use std::collections::HashMap;
use syn::BinOp;
use syn::Block;
use syn::Expr;
use syn::ExprClosure;
use syn::ImplItemFn;
use syn::ItemFn;
use syn::ItemImpl;
use syn::Lit;
use syn::Macro;
use syn::Member;
use syn::Pat;
use syn::Signature;
use syn::Stmt;
use syn::Type;
use syn::UnOp;
use syn::punctuated::Punctuated;

/// Tracks identifier-to-placeholder mappings during normalization.
pub struct NormalizationContext {
    /// Maps (identifier_string, kind) -> placeholder index
    mappings: HashMap<(String, PlaceholderKind), usize>,
    /// Per-kind counters
    counters: HashMap<PlaceholderKind, usize>,
}

impl NormalizationContext {
    #[must_use]
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    #[must_use]
    pub fn member_to_string(member: &Member) -> String {
        match member {
            Member::Named(ident) => ident.to_string(),
            Member::Unnamed(idx) => idx.index.to_string(),
        }
    }

    #[must_use]
    pub const fn normalize_bin_op(op: &BinOp) -> BinOpKind {
        match op {
            BinOp::Add(_) => BinOpKind::Add,
            BinOp::Sub(_) => BinOpKind::Sub,
            BinOp::Mul(_) => BinOpKind::Mul,
            BinOp::Div(_) => BinOpKind::Div,
            BinOp::Rem(_) => BinOpKind::Rem,
            BinOp::And(_) => BinOpKind::And,
            BinOp::Or(_) => BinOpKind::Or,
            BinOp::BitXor(_) => BinOpKind::BitXor,
            BinOp::BitAnd(_) => BinOpKind::BitAnd,
            BinOp::BitOr(_) => BinOpKind::BitOr,
            BinOp::Shl(_) => BinOpKind::Shl,
            BinOp::Shr(_) => BinOpKind::Shr,
            BinOp::Eq(_) => BinOpKind::Eq,
            BinOp::Lt(_) => BinOpKind::Lt,
            BinOp::Le(_) => BinOpKind::Le,
            BinOp::Ne(_) => BinOpKind::Ne,
            BinOp::Ge(_) => BinOpKind::Ge,
            BinOp::Gt(_) => BinOpKind::Gt,
            BinOp::AddAssign(_) => BinOpKind::AddAssign,
            BinOp::SubAssign(_) => BinOpKind::SubAssign,
            BinOp::MulAssign(_) => BinOpKind::MulAssign,
            BinOp::DivAssign(_) => BinOpKind::DivAssign,
            BinOp::RemAssign(_) => BinOpKind::RemAssign,
            BinOp::BitXorAssign(_) => BinOpKind::BitXorAssign,
            BinOp::BitAndAssign(_) => BinOpKind::BitAndAssign,
            BinOp::BitOrAssign(_) => BinOpKind::BitOrAssign,
            BinOp::ShlAssign(_) => BinOpKind::ShlAssign,
            BinOp::ShrAssign(_) => BinOpKind::ShrAssign,
            _ => BinOpKind::Other,
        }
    }

    pub fn normalize_block(&mut self, block: &Block) -> NormalizedNode {
        NormalizedNode::with_children(
            NodeKind::Block,
            block.stmts.iter().map(|s| self.normalize_stmt(s)).collect(),
        )
    }

    /// Normalize a closure expression.
    #[must_use]
    pub fn normalize_closure_expr(closure: &ExprClosure) -> NormalizedNode {
        let mut ctx = Self::new();
        let mut children = vec![ctx.normalize_expr(&closure.body)];
        children.extend(closure.inputs.iter().map(|p| ctx.normalize_pat(p)));
        NormalizedNode::with_children(NodeKind::Closure, children)
    }

    #[allow(clippy::too_many_lines)]
    pub fn normalize_expr(&mut self, expr: &Expr) -> NormalizedNode {
        match expr {
            Expr::Lit(el) => Self::normalize_lit(&el.lit),
            Expr::Path(ep) => {
                if ep.path.segments.len() == 1 {
                    let seg = &ep.path.segments[0];
                    let ident = seg.ident.to_string();
                    let kind = if ident.chars().next().is_some_and(char::is_uppercase) {
                        PlaceholderKind::Type
                    } else {
                        PlaceholderKind::Variable
                    };
                    let idx = self.placeholder(&ident, kind);
                    NormalizedNode::leaf(NodeKind::Placeholder(kind, idx))
                } else {
                    let segments: Vec<NormalizedNode> = ep
                        .path
                        .segments
                        .iter()
                        .map(|seg| {
                            let ident = seg.ident.to_string();
                            let kind = if ident.chars().next().is_some_and(char::is_uppercase) {
                                PlaceholderKind::Type
                            } else {
                                PlaceholderKind::Variable
                            };
                            let idx = self.placeholder(&ident, kind);
                            NormalizedNode::leaf(NodeKind::Placeholder(kind, idx))
                        })
                        .collect();
                    NormalizedNode::with_children(NodeKind::Path, segments)
                }
            }
            // BinaryOp -> [left, right]
            Expr::Binary(eb) => NormalizedNode::with_children(
                NodeKind::BinaryOp(Self::normalize_bin_op(&eb.op)),
                vec![
                    self.normalize_expr(&eb.left),
                    self.normalize_expr(&eb.right),
                ],
            ),
            // UnaryOp -> [operand]
            Expr::Unary(eu) => NormalizedNode::with_children(
                NodeKind::UnaryOp(Self::normalize_un_op(&eu.op)),
                vec![self.normalize_expr(&eu.expr)],
            ),
            // Call -> [func, arg0, arg1, ...]
            Expr::Call(ec) => {
                let mut children = vec![self.normalize_expr(&ec.func)];
                children.extend(ec.args.iter().map(|a| self.normalize_expr(a)));
                NormalizedNode::with_children(NodeKind::Call, children)
            }
            // MethodCall -> [receiver, method, arg0, ...]
            Expr::MethodCall(emc) => {
                let method_idx =
                    self.placeholder(&emc.method.to_string(), PlaceholderKind::Function);
                let mut children = vec![
                    self.normalize_expr(&emc.receiver),
                    NormalizedNode::leaf(NodeKind::Placeholder(
                        PlaceholderKind::Function,
                        method_idx,
                    )),
                ];
                children.extend(emc.args.iter().map(|a| self.normalize_expr(a)));
                NormalizedNode::with_children(NodeKind::MethodCall, children)
            }
            // FieldAccess -> [base, field]
            Expr::Field(ef) => {
                let field_idx = self.placeholder(
                    &Self::member_to_string(&ef.member),
                    PlaceholderKind::Variable,
                );
                NormalizedNode::with_children(
                    NodeKind::FieldAccess,
                    vec![
                        self.normalize_expr(&ef.base),
                        NormalizedNode::leaf(NodeKind::Placeholder(
                            PlaceholderKind::Variable,
                            field_idx,
                        )),
                    ],
                )
            }
            // Index -> [base, index]
            Expr::Index(ei) => NormalizedNode::with_children(
                NodeKind::Index,
                vec![
                    self.normalize_expr(&ei.expr),
                    self.normalize_expr(&ei.index),
                ],
            ),
            // Closure -> [body, param0, param1, ...]
            Expr::Closure(ec) => {
                let mut children = vec![self.normalize_expr(&ec.body)];
                children.extend(ec.inputs.iter().map(|p| self.normalize_pat(p)));
                NormalizedNode::with_children(NodeKind::Closure, children)
            }
            // Return -> [] or [value]
            Expr::Return(er) => {
                let children: Vec<_> = er
                    .expr
                    .as_ref()
                    .map(|e| vec![self.normalize_expr(e)])
                    .unwrap_or_default();
                NormalizedNode::with_children(NodeKind::Return, children)
            }
            // Break -> [] or [value]
            Expr::Break(eb) => {
                let children: Vec<_> = eb
                    .expr
                    .as_ref()
                    .map(|e| vec![self.normalize_expr(e)])
                    .unwrap_or_default();
                NormalizedNode::with_children(NodeKind::Break, children)
            }
            Expr::Continue(_) => NormalizedNode::leaf(NodeKind::Continue),
            // Assign -> [left, right]
            Expr::Assign(ea) => NormalizedNode::with_children(
                NodeKind::Assign,
                vec![
                    self.normalize_expr(&ea.left),
                    self.normalize_expr(&ea.right),
                ],
            ),
            // Reference -> [expr]
            Expr::Reference(er) => NormalizedNode::with_children(
                NodeKind::Reference {
                    mutable: er.mutability.is_some(),
                },
                vec![self.normalize_expr(&er.expr)],
            ),
            Expr::Tuple(et) => NormalizedNode::with_children(
                NodeKind::Tuple,
                et.elems.iter().map(|e| self.normalize_expr(e)).collect(),
            ),
            Expr::Array(ea) => NormalizedNode::with_children(
                NodeKind::Array,
                ea.elems.iter().map(|e| self.normalize_expr(e)).collect(),
            ),
            // Repeat -> [elem, len]
            Expr::Repeat(er) => NormalizedNode::with_children(
                NodeKind::Repeat,
                vec![self.normalize_expr(&er.expr), self.normalize_expr(&er.len)],
            ),
            // Cast -> [expr, ty]
            Expr::Cast(ec) => NormalizedNode::with_children(
                NodeKind::Cast,
                vec![self.normalize_expr(&ec.expr), self.normalize_type(&ec.ty)],
            ),
            // StructInit -> [rest_or_None, field0, field1, ...]
            Expr::Struct(es) => {
                let mut children = vec![NormalizedNode::opt(
                    es.rest.as_ref().map(|e| self.normalize_expr(e)),
                )];
                children.extend(es.fields.iter().map(|f| {
                    let field_idx = self.placeholder(
                        &Self::member_to_string(&f.member),
                        PlaceholderKind::Variable,
                    );
                    NormalizedNode::with_children(
                        NodeKind::FieldValue,
                        vec![
                            NormalizedNode::leaf(NodeKind::Placeholder(
                                PlaceholderKind::Variable,
                                field_idx,
                            )),
                            self.normalize_expr(&f.expr),
                        ],
                    )
                }));
                NormalizedNode::with_children(NodeKind::StructInit, children)
            }
            // Await -> [expr]
            Expr::Await(ea) => {
                NormalizedNode::with_children(NodeKind::Await, vec![self.normalize_expr(&ea.base)])
            }
            // Try -> [expr]
            Expr::Try(et) => {
                NormalizedNode::with_children(NodeKind::Try, vec![self.normalize_expr(&et.expr)])
            }
            // If -> [condition, then_branch, else_or_None]
            Expr::If(ei) => NormalizedNode::with_children(
                NodeKind::If,
                vec![
                    self.normalize_expr(&ei.cond),
                    self.normalize_block(&ei.then_branch),
                    NormalizedNode::opt(
                        ei.else_branch.as_ref().map(|(_, e)| self.normalize_expr(e)),
                    ),
                ],
            ),
            // Match -> [expr, arm0, arm1, ...]
            // Each arm is MatchArm -> [pattern, guard_or_None, body]
            Expr::Match(em) => {
                let mut children = vec![self.normalize_expr(&em.expr)];
                children.extend(em.arms.iter().map(|arm| {
                    NormalizedNode::with_children(
                        NodeKind::MatchArm,
                        vec![
                            self.normalize_pat(&arm.pat),
                            NormalizedNode::opt(
                                arm.guard.as_ref().map(|(_, g)| self.normalize_expr(g)),
                            ),
                            self.normalize_expr(&arm.body),
                        ],
                    )
                }));
                NormalizedNode::with_children(NodeKind::Match, children)
            }
            // Loop -> [body]
            Expr::Loop(el) => {
                NormalizedNode::with_children(NodeKind::Loop, vec![self.normalize_block(&el.body)])
            }
            // While -> [condition, body]
            Expr::While(ew) => NormalizedNode::with_children(
                NodeKind::While,
                vec![
                    self.normalize_expr(&ew.cond),
                    self.normalize_block(&ew.body),
                ],
            ),
            // ForLoop -> [pat, iter, body]
            Expr::ForLoop(ef) => NormalizedNode::with_children(
                NodeKind::ForLoop,
                vec![
                    self.normalize_pat(&ef.pat),
                    self.normalize_expr(&ef.expr),
                    self.normalize_block(&ef.body),
                ],
            ),
            Expr::Block(eb) => self.normalize_block(&eb.block),
            // Paren -> [expr]
            Expr::Paren(ep) => {
                NormalizedNode::with_children(NodeKind::Paren, vec![self.normalize_expr(&ep.expr)])
            }
            // Range -> [from_or_None, to_or_None]
            Expr::Range(er) => NormalizedNode::with_children(
                NodeKind::Range,
                vec![
                    NormalizedNode::opt(er.start.as_ref().map(|e| self.normalize_expr(e))),
                    NormalizedNode::opt(er.end.as_ref().map(|e| self.normalize_expr(e))),
                ],
            ),
            // LetExpr -> [pat, expr]
            Expr::Let(el) => NormalizedNode::with_children(
                NodeKind::LetExpr,
                vec![self.normalize_pat(&el.pat), self.normalize_expr(&el.expr)],
            ),
            Expr::Macro(em) => self.normalize_macro(&em.mac),
            Expr::Group(eg) => self.normalize_expr(&eg.expr),
            Expr::Unsafe(eu) => self.normalize_block(&eu.block),
            Expr::Const(ec) => self.normalize_block(&ec.block),
            _ => NormalizedNode::leaf(NodeKind::Opaque),
        }
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

    /// Normalize a method within an impl block.
    #[must_use]
    pub fn normalize_impl_item_fn(method: &ImplItemFn) -> NormalizedFn {
        Self::normalize_fn_parts(&method.sig, &method.block)
    }

    /// Normalize a top-level function.
    #[must_use]
    pub fn normalize_item_fn(func: &ItemFn) -> NormalizedFn {
        Self::normalize_fn_parts(&func.sig, &func.block)
    }

    #[must_use]
    pub const fn normalize_lit(lit: &Lit) -> NormalizedNode {
        match lit {
            Lit::Str(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Str)),
            Lit::ByteStr(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::ByteStr)),
            Lit::CStr(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::CStr)),
            Lit::Byte(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Byte)),
            Lit::Char(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Char)),
            Lit::Int(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Int)),
            Lit::Float(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Float)),
            Lit::Bool(_) => NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Bool)),
            _ => NormalizedNode::leaf(NodeKind::Opaque),
        }
    }

    pub fn normalize_macro(&mut self, mac: &Macro) -> NormalizedNode {
        let name = mac
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        let args = if mac.tokens.is_empty() {
            Vec::new()
        } else {
            match mac.parse_body_with(Punctuated::<Expr, syn::Token![,]>::parse_terminated) {
                Ok(punct) => punct.into_iter().map(|e| self.normalize_expr(&e)).collect(),
                Err(_) => vec![NormalizedNode::leaf(NodeKind::Opaque)],
            }
        };
        NormalizedNode::with_children(NodeKind::MacroCall { name }, args)
    }

    pub fn normalize_pat(&mut self, pat: &Pat) -> NormalizedNode {
        match pat {
            Pat::Ident(pi) => {
                let idx = self.placeholder(&pi.ident.to_string(), PlaceholderKind::Variable);
                NormalizedNode::leaf(NodeKind::PatPlaceholder(PlaceholderKind::Variable, idx))
            }
            Pat::Wild(_) => NormalizedNode::leaf(NodeKind::PatWild),
            Pat::Tuple(pt) => NormalizedNode::with_children(
                NodeKind::PatTuple,
                pt.elems.iter().map(|p| self.normalize_pat(p)).collect(),
            ),
            Pat::TupleStruct(pts) => NormalizedNode::with_children(
                NodeKind::PatStruct,
                pts.elems.iter().map(|p| self.normalize_pat(p)).collect(),
            ),
            Pat::Struct(ps) => NormalizedNode::with_children(
                NodeKind::PatStruct,
                ps.fields
                    .iter()
                    .map(|f| {
                        let value = self.normalize_pat(&f.pat);
                        let name_idx = self.placeholder(
                            &Self::member_to_string(&f.member),
                            PlaceholderKind::Variable,
                        );
                        NormalizedNode::with_children(
                            NodeKind::FieldValue,
                            vec![
                                NormalizedNode::leaf(NodeKind::PatPlaceholder(
                                    PlaceholderKind::Variable,
                                    name_idx,
                                )),
                                value,
                            ],
                        )
                    })
                    .collect(),
            ),
            Pat::Or(po) => NormalizedNode::with_children(
                NodeKind::PatOr,
                po.cases.iter().map(|p| self.normalize_pat(p)).collect(),
            ),
            Pat::Lit(pl) => NormalizedNode::with_children(
                NodeKind::PatLiteral,
                vec![Self::normalize_lit(&pl.lit)],
            ),
            Pat::Reference(pr) => NormalizedNode::with_children(
                NodeKind::PatReference {
                    mutable: pr.mutability.is_some(),
                },
                vec![self.normalize_pat(&pr.pat)],
            ),
            Pat::Slice(ps) => NormalizedNode::with_children(
                NodeKind::PatSlice,
                ps.elems.iter().map(|p| self.normalize_pat(p)).collect(),
            ),
            Pat::Rest(_) => NormalizedNode::leaf(NodeKind::PatRest),
            // PatRange -> [from_or_None, to_or_None]
            Pat::Range(pr) => NormalizedNode::with_children(
                NodeKind::PatRange,
                vec![
                    NormalizedNode::opt(pr.start.as_ref().map(|e| self.normalize_expr(e))),
                    NormalizedNode::opt(pr.end.as_ref().map(|e| self.normalize_expr(e))),
                ],
            ),
            Pat::Path(pp) => {
                if pp.path.segments.len() == 1 {
                    let seg = &pp.path.segments[0];
                    let idx = self.placeholder(&seg.ident.to_string(), PlaceholderKind::Variable);
                    NormalizedNode::leaf(NodeKind::PatPlaceholder(PlaceholderKind::Variable, idx))
                } else {
                    NormalizedNode::with_children(
                        NodeKind::PatStruct,
                        pp.path
                            .segments
                            .iter()
                            .map(|seg| {
                                let idx = self
                                    .placeholder(&seg.ident.to_string(), PlaceholderKind::Variable);
                                NormalizedNode::leaf(NodeKind::PatPlaceholder(
                                    PlaceholderKind::Variable,
                                    idx,
                                ))
                            })
                            .collect(),
                    )
                }
            }
            Pat::Type(pt) => self.normalize_pat(&pt.pat),
            Pat::Macro(pm) => self.normalize_macro(&pm.mac),
            _ => NormalizedNode::leaf(NodeKind::Opaque),
        }
    }

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

    pub fn normalize_stmt(&mut self, stmt: &Stmt) -> NormalizedNode {
        match stmt {
            // LetBinding -> [pattern, type_or_None, init_or_None, diverge_or_None]
            Stmt::Local(local) => NormalizedNode::with_children(
                NodeKind::LetBinding,
                vec![
                    self.normalize_pat(&local.pat),
                    NormalizedNode::none(), // type annotations on let bindings are part of the pattern in syn
                    NormalizedNode::opt(
                        local
                            .init
                            .as_ref()
                            .map(|init| self.normalize_expr(&init.expr)),
                    ),
                    NormalizedNode::opt(
                        local
                            .init
                            .as_ref()
                            .and_then(|init| init.diverge.as_ref())
                            .map(|(_, expr)| self.normalize_expr(expr)),
                    ),
                ],
            ),
            Stmt::Expr(expr, semi) => {
                let normalized = self.normalize_expr(expr);
                if semi.is_some() {
                    NormalizedNode::with_children(NodeKind::Semi, vec![normalized])
                } else {
                    normalized
                }
            }
            Stmt::Item(_) => NormalizedNode::leaf(NodeKind::Opaque),
            Stmt::Macro(sm) => {
                let normalized = self.normalize_macro(&sm.mac);
                if sm.semi_token.is_some() {
                    NormalizedNode::with_children(NodeKind::Semi, vec![normalized])
                } else {
                    normalized
                }
            }
        }
    }

    pub fn normalize_type(&mut self, ty: &Type) -> NormalizedNode {
        match ty {
            Type::Path(tp) => {
                // Single-segment paths become type placeholders
                if tp.qself.is_none() && tp.path.segments.len() == 1 {
                    let seg = &tp.path.segments[0];
                    let idx = self.placeholder(&seg.ident.to_string(), PlaceholderKind::Type);
                    NormalizedNode::leaf(NodeKind::TypePlaceholder(PlaceholderKind::Type, idx))
                } else {
                    let segments: Vec<NormalizedNode> = tp
                        .path
                        .segments
                        .iter()
                        .map(|seg| {
                            let idx =
                                self.placeholder(&seg.ident.to_string(), PlaceholderKind::Type);
                            NormalizedNode::leaf(NodeKind::TypePlaceholder(
                                PlaceholderKind::Type,
                                idx,
                            ))
                        })
                        .collect();
                    NormalizedNode::with_children(NodeKind::TypePath, segments)
                }
            }
            Type::Reference(r) => NormalizedNode::with_children(
                NodeKind::TypeReference {
                    mutable: r.mutability.is_some(),
                },
                vec![self.normalize_type(&r.elem)],
            ),
            Type::Tuple(t) => {
                if t.elems.is_empty() {
                    NormalizedNode::leaf(NodeKind::TypeUnit)
                } else {
                    NormalizedNode::with_children(
                        NodeKind::TypeTuple,
                        t.elems.iter().map(|e| self.normalize_type(e)).collect(),
                    )
                }
            }
            Type::Slice(s) => NormalizedNode::with_children(
                NodeKind::TypeSlice,
                vec![self.normalize_type(&s.elem)],
            ),
            Type::Array(a) => NormalizedNode::with_children(
                NodeKind::TypeArray,
                vec![self.normalize_type(&a.elem), self.normalize_expr(&a.len)],
            ),
            Type::ImplTrait(i) => NormalizedNode::with_children(
                NodeKind::TypeImplTrait,
                i.bounds
                    .iter()
                    .filter_map(|b| {
                        if let syn::TypeParamBound::Trait(t) = b {
                            let segments: Vec<NormalizedNode> = t
                                .path
                                .segments
                                .iter()
                                .map(|seg| {
                                    let idx = self
                                        .placeholder(&seg.ident.to_string(), PlaceholderKind::Type);
                                    NormalizedNode::leaf(NodeKind::TypePlaceholder(
                                        PlaceholderKind::Type,
                                        idx,
                                    ))
                                })
                                .collect();
                            Some(if segments.len() == 1 {
                                segments.into_iter().next().unwrap()
                            } else {
                                NormalizedNode::with_children(NodeKind::TypePath, segments)
                            })
                        } else {
                            None
                        }
                    })
                    .collect(),
            ),
            Type::Infer(_) => NormalizedNode::leaf(NodeKind::TypeInfer),
            Type::Never(_) => NormalizedNode::leaf(NodeKind::TypeNever),
            Type::Paren(p) => self.normalize_type(&p.elem),
            Type::Macro(tm) => self.normalize_macro(&tm.mac),
            _ => NormalizedNode::leaf(NodeKind::Opaque),
        }
    }

    #[must_use]
    pub const fn normalize_un_op(op: &UnOp) -> UnOpKind {
        match op {
            UnOp::Deref(_) => UnOpKind::Deref,
            UnOp::Not(_) => UnOpKind::Not,
            UnOp::Neg(_) => UnOpKind::Neg,
            _ => UnOpKind::Other,
        }
    }

    /// Get or assign a placeholder index for the given identifier and kind.
    pub fn placeholder(&mut self, name: &str, kind: PlaceholderKind) -> usize {
        let key = (name.to_string(), kind);
        if let Some(&idx) = self.mappings.get(&key) {
            return idx;
        }
        let counter = self.counters.entry(kind).or_insert(0);
        let idx = *counter;
        *counter += 1;
        self.mappings.insert(key, idx);
        idx
    }

    fn normalize_fn_parts(sig: &Signature, block: &Block) -> NormalizedFn {
        let mut ctx = Self::new();
        let signature = ctx.normalize_signature(sig);
        let body = ctx.normalize_block(block);
        NormalizedFn::new(signature, body)
    }
}

impl Default for NormalizationContext {
    fn default() -> Self {
        Self::new()
    }
}

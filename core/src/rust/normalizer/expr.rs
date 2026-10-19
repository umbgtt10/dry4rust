// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node::NodeKind;
use crate::node::NormalizedNode;
use crate::node::PlaceholderKind;
use crate::rust::normalizer::normalization_context::NormalizationContext;

use syn::Block;
use syn::Expr;
use syn::Stmt;

impl NormalizationContext {
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

    pub fn normalize_block(&mut self, block: &Block) -> NormalizedNode {
        NormalizedNode::with_children(
            NodeKind::Block,
            block.stmts.iter().map(|s| self.normalize_stmt(s)).collect(),
        )
    }
}

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node::{NodeKind, NormalizedNode, PlaceholderKind};
use crate::normalization_context::NormalizationContext;
use syn::Pat;
use syn::Type;

use super::expr::normalize_expr;
use super::helpers::{member_to_string, normalize_lit, normalize_macro};

pub fn normalize_type(ty: &Type, ctx: &mut NormalizationContext) -> NormalizedNode {
    match ty {
        Type::Path(tp) => {
            // Single-segment paths become type placeholders
            if tp.qself.is_none() && tp.path.segments.len() == 1 {
                let seg = &tp.path.segments[0];
                let idx = ctx.placeholder(&seg.ident.to_string(), PlaceholderKind::Type);
                NormalizedNode::leaf(NodeKind::TypePlaceholder(PlaceholderKind::Type, idx))
            } else {
                let segments: Vec<NormalizedNode> = tp
                    .path
                    .segments
                    .iter()
                    .map(|seg| {
                        let idx = ctx.placeholder(&seg.ident.to_string(), PlaceholderKind::Type);
                        NormalizedNode::leaf(NodeKind::TypePlaceholder(PlaceholderKind::Type, idx))
                    })
                    .collect();
                NormalizedNode::with_children(NodeKind::TypePath, segments)
            }
        }
        Type::Reference(r) => NormalizedNode::with_children(
            NodeKind::TypeReference {
                mutable: r.mutability.is_some(),
            },
            vec![normalize_type(&r.elem, ctx)],
        ),
        Type::Tuple(t) => {
            if t.elems.is_empty() {
                NormalizedNode::leaf(NodeKind::TypeUnit)
            } else {
                NormalizedNode::with_children(
                    NodeKind::TypeTuple,
                    t.elems.iter().map(|e| normalize_type(e, ctx)).collect(),
                )
            }
        }
        Type::Slice(s) => {
            NormalizedNode::with_children(NodeKind::TypeSlice, vec![normalize_type(&s.elem, ctx)])
        }
        Type::Array(a) => NormalizedNode::with_children(
            NodeKind::TypeArray,
            vec![normalize_type(&a.elem, ctx), normalize_expr(&a.len, ctx)],
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
                                let idx =
                                    ctx.placeholder(&seg.ident.to_string(), PlaceholderKind::Type);
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
        Type::Paren(p) => normalize_type(&p.elem, ctx),
        Type::Macro(tm) => normalize_macro(&tm.mac, ctx),
        _ => NormalizedNode::leaf(NodeKind::Opaque),
    }
}

pub fn normalize_pat(pat: &Pat, ctx: &mut NormalizationContext) -> NormalizedNode {
    match pat {
        Pat::Ident(pi) => {
            let idx = ctx.placeholder(&pi.ident.to_string(), PlaceholderKind::Variable);
            NormalizedNode::leaf(NodeKind::PatPlaceholder(PlaceholderKind::Variable, idx))
        }
        Pat::Wild(_) => NormalizedNode::leaf(NodeKind::PatWild),
        Pat::Tuple(pt) => NormalizedNode::with_children(
            NodeKind::PatTuple,
            pt.elems.iter().map(|p| normalize_pat(p, ctx)).collect(),
        ),
        Pat::TupleStruct(pts) => NormalizedNode::with_children(
            NodeKind::PatStruct,
            pts.elems.iter().map(|p| normalize_pat(p, ctx)).collect(),
        ),
        Pat::Struct(ps) => NormalizedNode::with_children(
            NodeKind::PatStruct,
            ps.fields
                .iter()
                .map(|f| {
                    let value = normalize_pat(&f.pat, ctx);
                    let name_idx =
                        ctx.placeholder(&member_to_string(&f.member), PlaceholderKind::Variable);
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
            po.cases.iter().map(|p| normalize_pat(p, ctx)).collect(),
        ),
        Pat::Lit(pl) => {
            NormalizedNode::with_children(NodeKind::PatLiteral, vec![normalize_lit(&pl.lit)])
        }
        Pat::Reference(pr) => NormalizedNode::with_children(
            NodeKind::PatReference {
                mutable: pr.mutability.is_some(),
            },
            vec![normalize_pat(&pr.pat, ctx)],
        ),
        Pat::Slice(ps) => NormalizedNode::with_children(
            NodeKind::PatSlice,
            ps.elems.iter().map(|p| normalize_pat(p, ctx)).collect(),
        ),
        Pat::Rest(_) => NormalizedNode::leaf(NodeKind::PatRest),
        // PatRange -> [from_or_None, to_or_None]
        Pat::Range(pr) => NormalizedNode::with_children(
            NodeKind::PatRange,
            vec![
                NormalizedNode::opt(pr.start.as_ref().map(|e| normalize_expr(e, ctx))),
                NormalizedNode::opt(pr.end.as_ref().map(|e| normalize_expr(e, ctx))),
            ],
        ),
        Pat::Path(pp) => {
            if pp.path.segments.len() == 1 {
                let seg = &pp.path.segments[0];
                let idx = ctx.placeholder(&seg.ident.to_string(), PlaceholderKind::Variable);
                NormalizedNode::leaf(NodeKind::PatPlaceholder(PlaceholderKind::Variable, idx))
            } else {
                NormalizedNode::with_children(
                    NodeKind::PatStruct,
                    pp.path
                        .segments
                        .iter()
                        .map(|seg| {
                            let idx =
                                ctx.placeholder(&seg.ident.to_string(), PlaceholderKind::Variable);
                            NormalizedNode::leaf(NodeKind::PatPlaceholder(
                                PlaceholderKind::Variable,
                                idx,
                            ))
                        })
                        .collect(),
                )
            }
        }
        Pat::Type(pt) => normalize_pat(&pt.pat, ctx),
        Pat::Macro(pm) => normalize_macro(&pm.mac, ctx),
        _ => NormalizedNode::leaf(NodeKind::Opaque),
    }
}

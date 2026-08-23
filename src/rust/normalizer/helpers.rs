// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node::{BinOpKind, LiteralKind, NodeKind, NormalizedNode, UnOpKind};
use crate::normalization_context::NormalizationContext;
use syn::punctuated::Punctuated;

use super::expr::normalize_expr;
use syn::BinOp;
use syn::Expr;
use syn::Lit;
use syn::Macro;
use syn::Member;
use syn::UnOp;

#[must_use]
pub fn member_to_string(member: &Member) -> String {
    match member {
        Member::Named(ident) => ident.to_string(),
        Member::Unnamed(idx) => idx.index.to_string(),
    }
}

pub fn normalize_macro(mac: &Macro, ctx: &mut NormalizationContext) -> NormalizedNode {
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
            Ok(punct) => punct.into_iter().map(|e| normalize_expr(&e, ctx)).collect(),
            Err(_) => vec![NormalizedNode::leaf(NodeKind::Opaque)],
        }
    };
    NormalizedNode::with_children(NodeKind::MacroCall { name }, args)
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

#[must_use]
pub const fn normalize_un_op(op: &UnOp) -> UnOpKind {
    match op {
        UnOp::Deref(_) => UnOpKind::Deref,
        UnOp::Not(_) => UnOpKind::Not,
        UnOp::Neg(_) => UnOpKind::Neg,
        _ => UnOpKind::Other,
    }
}

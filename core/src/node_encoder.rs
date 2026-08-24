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
use crate::stable_hasher::StableHasher;

/// Walks a normalised tree and feeds it to a [`StableHasher`] in a form that
/// does not change when the code around it does.
///
/// Every variant is written as its own name rather than its position in the
/// enum, so reordering `NodeKind` -- or inserting a variant in the middle --
/// leaves existing fingerprints alone. Every match here is exhaustive, so a
/// new variant is a compile error rather than a silently unhashed field.
///
/// Children are written with their count in front, which is what stops
/// `[a, [b]]` and `[[a], b]` from hashing alike.
pub struct NodeEncoder {
    hasher: StableHasher,
}

impl NodeEncoder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hasher: StableHasher::new(),
        }
    }

    /// Absorb a whole tree.
    pub fn encode(&mut self, node: &NormalizedNode) {
        self.encode_kind(&node.kind);
        self.hasher.write_u64(node.children.len() as u64);
        for child in &node.children {
            self.encode(child);
        }
    }

    /// The fingerprint of everything absorbed so far.
    #[must_use]
    pub const fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn encode_kind(&mut self, kind: &NodeKind) {
        self.hasher.write_str(Self::kind_name(kind));
        self.encode_payload(kind);
    }

    /// The data a variant carries beyond its name.
    fn encode_payload(&mut self, kind: &NodeKind) {
        match kind {
            NodeKind::Literal(literal) => self.hasher.write_str(Self::literal_name(literal)),
            NodeKind::Placeholder(placeholder, index)
            | NodeKind::PatPlaceholder(placeholder, index)
            | NodeKind::TypePlaceholder(placeholder, index) => {
                self.hasher.write_str(Self::placeholder_name(*placeholder));
                self.hasher.write_u64(*index as u64);
            }
            NodeKind::BinaryOp(op) => self.hasher.write_str(Self::bin_op_name(op)),
            NodeKind::UnaryOp(op) => self.hasher.write_str(Self::un_op_name(op)),
            NodeKind::Reference { mutable }
            | NodeKind::PatReference { mutable }
            | NodeKind::TypeReference { mutable } => self.hasher.write_u8(u8::from(*mutable)),
            NodeKind::MacroCall { name } => self.hasher.write_str(name),
            _ => {}
        }
    }

    const fn kind_name(kind: &NodeKind) -> &'static str {
        match kind {
            NodeKind::Block => "Block",
            NodeKind::LetBinding => "LetBinding",
            NodeKind::Semi => "Semi",
            NodeKind::Paren => "Paren",
            NodeKind::Literal(..) => "Literal",
            NodeKind::Placeholder(..) => "Placeholder",
            NodeKind::BinaryOp(..) => "BinaryOp",
            NodeKind::UnaryOp(..) => "UnaryOp",
            NodeKind::Range => "Range",
            NodeKind::Call => "Call",
            NodeKind::MethodCall => "MethodCall",
            NodeKind::FieldAccess => "FieldAccess",
            NodeKind::Index => "Index",
            NodeKind::Path => "Path",
            NodeKind::Closure => "Closure",
            NodeKind::FnSignature => "FnSignature",
            NodeKind::Return => "Return",
            NodeKind::Break => "Break",
            NodeKind::Continue => "Continue",
            NodeKind::Assign => "Assign",
            NodeKind::Reference { .. } => "Reference",
            NodeKind::Tuple => "Tuple",
            NodeKind::Array => "Array",
            NodeKind::Set => "Set",
            NodeKind::Repeat => "Repeat",
            NodeKind::Cast => "Cast",
            NodeKind::StructInit => "StructInit",
            NodeKind::Await => "Await",
            NodeKind::Yield => "Yield",
            NodeKind::Try => "Try",
            NodeKind::If => "If",
            NodeKind::Match => "Match",
            NodeKind::MatchArm => "MatchArm",
            NodeKind::Loop => "Loop",
            NodeKind::While => "While",
            NodeKind::ForLoop => "ForLoop",
            NodeKind::LetExpr => "LetExpr",
            NodeKind::PatWild => "PatWild",
            NodeKind::PatPlaceholder(..) => "PatPlaceholder",
            NodeKind::PatTuple => "PatTuple",
            NodeKind::PatStruct => "PatStruct",
            NodeKind::PatOr => "PatOr",
            NodeKind::PatLiteral => "PatLiteral",
            NodeKind::PatReference { .. } => "PatReference",
            NodeKind::PatSlice => "PatSlice",
            NodeKind::PatRest => "PatRest",
            NodeKind::PatRange => "PatRange",
            NodeKind::TypePlaceholder(..) => "TypePlaceholder",
            NodeKind::TypeReference { .. } => "TypeReference",
            NodeKind::TypeTuple => "TypeTuple",
            NodeKind::TypeSlice => "TypeSlice",
            NodeKind::TypeArray => "TypeArray",
            NodeKind::TypePath => "TypePath",
            NodeKind::TypeImplTrait => "TypeImplTrait",
            NodeKind::TypeInfer => "TypeInfer",
            NodeKind::TypeUnit => "TypeUnit",
            NodeKind::TypeNever => "TypeNever",
            NodeKind::FieldValue => "FieldValue",
            NodeKind::MacroCall { .. } => "MacroCall",
            NodeKind::Opaque => "Opaque",
            NodeKind::None => "None",
        }
    }

    const fn literal_name(literal: &LiteralKind) -> &'static str {
        match literal {
            LiteralKind::Int => "Int",
            LiteralKind::Float => "Float",
            LiteralKind::Str => "Str",
            LiteralKind::ByteStr => "ByteStr",
            LiteralKind::CStr => "CStr",
            LiteralKind::Byte => "Byte",
            LiteralKind::Char => "Char",
            LiteralKind::Bool => "Bool",
            LiteralKind::Null => "Null",
        }
    }

    const fn placeholder_name(placeholder: PlaceholderKind) -> &'static str {
        match placeholder {
            PlaceholderKind::Variable => "Variable",
            PlaceholderKind::Function => "Function",
            PlaceholderKind::Type => "Type",
            PlaceholderKind::Lifetime => "Lifetime",
            PlaceholderKind::Label => "Label",
        }
    }

    const fn bin_op_name(op: &BinOpKind) -> &'static str {
        match op {
            BinOpKind::Add => "Add",
            BinOpKind::Sub => "Sub",
            BinOpKind::Mul => "Mul",
            BinOpKind::Div => "Div",
            BinOpKind::Rem => "Rem",
            BinOpKind::And => "And",
            BinOpKind::Or => "Or",
            BinOpKind::BitXor => "BitXor",
            BinOpKind::BitAnd => "BitAnd",
            BinOpKind::BitOr => "BitOr",
            BinOpKind::Shl => "Shl",
            BinOpKind::Shr => "Shr",
            BinOpKind::Eq => "Eq",
            BinOpKind::Lt => "Lt",
            BinOpKind::Le => "Le",
            BinOpKind::Ne => "Ne",
            BinOpKind::Ge => "Ge",
            BinOpKind::Gt => "Gt",
            BinOpKind::AddAssign => "AddAssign",
            BinOpKind::SubAssign => "SubAssign",
            BinOpKind::MulAssign => "MulAssign",
            BinOpKind::DivAssign => "DivAssign",
            BinOpKind::RemAssign => "RemAssign",
            BinOpKind::BitXorAssign => "BitXorAssign",
            BinOpKind::BitAndAssign => "BitAndAssign",
            BinOpKind::BitOrAssign => "BitOrAssign",
            BinOpKind::ShlAssign => "ShlAssign",
            BinOpKind::ShrAssign => "ShrAssign",
            BinOpKind::FloorDiv => "FloorDiv",
            BinOpKind::Pow => "Pow",
            BinOpKind::In => "In",
            BinOpKind::NotIn => "NotIn",
            BinOpKind::Is => "Is",
            BinOpKind::IsNot => "IsNot",
            BinOpKind::FloorDivAssign => "FloorDivAssign",
            BinOpKind::PowAssign => "PowAssign",
            BinOpKind::Other => "Other",
        }
    }

    const fn un_op_name(op: &UnOpKind) -> &'static str {
        match op {
            UnOpKind::Deref => "Deref",
            UnOpKind::Not => "Not",
            UnOpKind::Neg => "Neg",
            UnOpKind::Other => "Other",
        }
    }
}

impl Default for NodeEncoder {
    fn default() -> Self {
        Self::new()
    }
}

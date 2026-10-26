// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

use crate::node::bin_op_kind::BinOpKind;
use crate::node::literal_kind::LiteralKind;
use crate::node::placeholder_kind::PlaceholderKind;
use crate::node::un_op_kind::UnOpKind;

/// The kind of a normalized AST node. Carries only non-child data
/// (operator kinds, literal kinds, placeholder indices, mutability flags, macro names).
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum NodeKind {
    // Blocks and statements
    Block,
    LetBinding,
    Semi,
    Paren,

    // Literals and identifiers
    Literal(LiteralKind),
    Placeholder(PlaceholderKind, usize),

    // Operations
    BinaryOp(BinOpKind),
    UnaryOp(UnOpKind),
    Range,

    // Calls and access
    Call,
    MethodCall,
    FieldAccess,
    Index,
    Path,

    // Closures and functions
    Closure,
    FnSignature,

    // Control flow
    Return,
    Break,
    Continue,
    Assign,

    // References and pointers
    Reference {
        mutable: bool,
    },

    // Compound types
    Tuple,
    Array,
    Set,
    Repeat,

    // Type operations
    Cast,
    StructInit,

    // Async/error
    Await,
    Yield,
    Try,

    // Control flow structures
    If,
    Match,
    MatchArm,
    Loop,
    While,
    ForLoop,
    LetExpr,

    // Patterns
    PatWild,
    PatPlaceholder(PlaceholderKind, usize),
    PatTuple,
    PatStruct,
    PatOr,
    PatLiteral,
    PatReference {
        mutable: bool,
    },
    PatSlice,
    PatRest,
    PatRange,

    // Types
    TypePlaceholder(PlaceholderKind, usize),
    TypeReference {
        mutable: bool,
    },
    TypeTuple,
    TypeSlice,
    TypeArray,
    TypePath,
    TypeImplTrait,
    TypeInfer,
    TypeUnit,
    TypeNever,

    // Field initializer (name = value)
    FieldValue,

    // Macro invocations
    MacroCall {
        name: String,
    },

    // Opaque — unsupported constructs
    Opaque,

    /// Sentinel for absent optional children, ensuring fixed child positions
    /// for correct zip alignment in similarity comparison.
    #[default]
    None,
}

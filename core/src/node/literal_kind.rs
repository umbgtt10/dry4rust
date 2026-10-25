// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// Kinds of literals — preserves type but erases value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Int,
    Float,
    Str,
    ByteStr,
    CStr,
    Byte,
    Char,
    Bool,
    Null,
}

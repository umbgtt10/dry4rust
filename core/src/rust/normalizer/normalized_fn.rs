// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::node::NormalizedNode;

/// A function normalized: its signature and its body, kept apart because the
/// analysis compares them apart -- and named, because both are a
/// `NormalizedNode` and only position told them apart in a pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedFn {
    pub signature: NormalizedNode,
    pub body: NormalizedNode,
}

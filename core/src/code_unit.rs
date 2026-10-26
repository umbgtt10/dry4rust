// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::code_unit_kind::CodeUnitKind;
use crate::fingerprint::Fingerprint;
use crate::node::normalized_node::NormalizedNode;
use crate::source_path::SourcePath;

/// A unit of code extracted and normalized for duplication analysis.
#[derive(Debug, Clone, Default)]
pub struct CodeUnit {
    pub kind: CodeUnitKind,
    pub name: String,
    pub file: SourcePath,
    pub line_start: usize,
    pub line_end: usize,
    pub signature: NormalizedNode,
    pub body: NormalizedNode,
    pub fingerprint: Fingerprint,
    pub node_count: usize,
    /// For sub-function units, the name of the parent function.
    pub parent_name: Option<String>,
    /// Whether this code unit was identified as test code by the language analyzer.
    pub is_test: bool,
}

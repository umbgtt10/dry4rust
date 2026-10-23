// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;
use std::path::PathBuf;

use crate::code_unit_kind::CodeUnitKind;
use crate::fingerprint::Fingerprint;
use crate::node::NormalizedNode;

/// A unit of code extracted and normalized for duplication analysis.
#[derive(Debug, Clone, Default)]
pub struct CodeUnit {
    kind: CodeUnitKind,
    name: String,
    file: PathBuf,
    line_start: usize,
    line_end: usize,
    signature: NormalizedNode,
    body: NormalizedNode,
    fingerprint: Fingerprint,
    node_count: usize,
    /// For sub-function units, the name of the parent function.
    parent_name: Option<String>,
    /// Whether this code unit was identified as test code by the language analyzer.
    is_test: bool,
}

impl CodeUnit {
    #[must_use]
    pub const fn body(&self) -> &NormalizedNode {
        &self.body
    }

    #[must_use]
    pub fn file(&self) -> &Path {
        &self.file
    }

    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    #[must_use]
    pub const fn is_test(&self) -> bool {
        self.is_test
    }

    #[must_use]
    pub const fn kind(&self) -> &CodeUnitKind {
        &self.kind
    }

    #[must_use]
    pub const fn line_end(&self) -> usize {
        self.line_end
    }

    #[must_use]
    pub const fn line_start(&self) -> usize {
        self.line_start
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    #[must_use]
    pub fn parent_name(&self) -> Option<&str> {
        self.parent_name.as_deref()
    }

    #[must_use]
    pub const fn signature(&self) -> &NormalizedNode {
        &self.signature
    }

    #[must_use]
    pub fn with_body(self, body: NormalizedNode) -> Self {
        Self { body, ..self }
    }

    #[must_use]
    pub fn with_file(self, file: PathBuf) -> Self {
        Self { file, ..self }
    }

    #[must_use]
    pub fn with_fingerprint(self, fingerprint: Fingerprint) -> Self {
        Self {
            fingerprint,
            ..self
        }
    }

    #[must_use]
    pub fn with_is_test(self, is_test: bool) -> Self {
        Self { is_test, ..self }
    }

    #[must_use]
    pub fn with_kind(self, kind: CodeUnitKind) -> Self {
        Self { kind, ..self }
    }

    #[must_use]
    pub fn with_line_end(self, line_end: usize) -> Self {
        Self { line_end, ..self }
    }

    #[must_use]
    pub fn with_line_start(self, line_start: usize) -> Self {
        Self { line_start, ..self }
    }

    #[must_use]
    pub fn with_name(self, name: String) -> Self {
        Self { name, ..self }
    }

    #[must_use]
    pub fn with_node_count(self, node_count: usize) -> Self {
        Self { node_count, ..self }
    }

    #[must_use]
    pub fn with_parent_name(self, parent_name: Option<String>) -> Self {
        Self {
            parent_name,
            ..self
        }
    }

    #[must_use]
    pub fn with_signature(self, signature: NormalizedNode) -> Self {
        Self { signature, ..self }
    }
}

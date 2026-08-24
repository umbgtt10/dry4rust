// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::analysis::AnalysisResult;
use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit::CodeUnitKind;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::grouper::compute_stats_with_sub;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use std::collections::HashSet;
use std::path::PathBuf;

/// A duplicate group with a stated fingerprint and members, for tests that
/// care about grouping rather than about the code behind it.
pub fn group(fingerprint: u64, names: &[&str]) -> DuplicateGroup {
    let fingerprint = Fingerprint::new(fingerprint);
    DuplicateGroup {
        fingerprint,
        members: names
            .iter()
            .map(|name| CodeUnit {
                kind: CodeUnitKind::Function,
                name: (*name).to_owned(),
                file: PathBuf::from("src/lib.rs"),
                line_start: 1,
                line_end: 9,
                signature: NormalizedNode::leaf(NodeKind::Opaque),
                body: NormalizedNode::leaf(NodeKind::Opaque),
                fingerprint,
                node_count: 9,
                parent_name: None,
                is_test: false,
            })
            .collect(),
        similarity: 1.0,
    }
}

/// An analysis result holding exactly these groups, for tests about what is
/// done with groups rather than about how they were found.
pub fn result_with(
    exact: Vec<DuplicateGroup>,
    near: Vec<DuplicateGroup>,
    sub_exact: Vec<DuplicateGroup>,
    sub_near: Vec<DuplicateGroup>,
) -> AnalysisResult {
    AnalysisResult {
        stats: compute_stats_with_sub(&[], &exact, &near, &sub_exact, &sub_near),
        exact_groups: exact,
        near_groups: near,
        sub_exact_groups: sub_exact,
        sub_near_groups: sub_near,
        warnings: Vec::new(),
        all_fingerprints: HashSet::new(),
    }
}

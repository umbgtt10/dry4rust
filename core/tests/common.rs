// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_baseline_store::InMemoryBaselineStore;
use dry4rust::analysis::Analysis;
use dry4rust::cli::suppression::baseline_constants::FORMAT_VERSION;
use dry4rust::cli::suppression::baseline_entry::BaselineEntry;
use dry4rust::cli::suppression::baseline_file::BaselineFile;
use dry4rust::cli::suppression::baseline_filter::BaselineFilter;
use dry4rust::cli::suppression::baseline_kind::BaselineKind;
use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit_kind::CodeUnitKind;
use dry4rust::config::Config;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::groups::duplicate_group::DuplicateGroup;
use dry4rust::node::node_kind::NodeKind;
use dry4rust::node::normalized_node::NormalizedNode;
use dry4rust::source_path::SourcePath;
use std::collections::BTreeSet;

/// A baseline in effect, recording each group under the kind it is paired
/// with.
#[must_use]
pub fn baseline_recording(recorded: &[(BaselineKind, &DuplicateGroup)]) -> BaselineFilter {
    let store = InMemoryBaselineStore::new();
    BaselineFile::new(
        FORMAT_VERSION,
        recorded
            .iter()
            .map(|(kind, group)| BaselineEntry::of(*kind, group))
            .collect(),
    )
    .save(
        &store,
        &SourcePath::new(String::from("/p/dry4rust-baseline.json")),
    )
    .expect("the baseline is written");
    let config = Config::default()
        .with_root(SourcePath::new(String::from("/p")))
        .with_baseline(Some(SourcePath::new(String::from(
            "dry4rust-baseline.json",
        ))));
    BaselineFilter::load(&store, &config).expect("the baseline loads")
}

/// A duplicate group with a stated fingerprint and members, for tests that
/// care about grouping rather than about the code behind it.
#[must_use]
pub fn group(fingerprint: u64, names: &[&str]) -> DuplicateGroup {
    let fingerprint = Fingerprint::new(fingerprint);
    DuplicateGroup::new(
        fingerprint,
        names
            .iter()
            .map(|name| CodeUnit {
                kind: CodeUnitKind::Function,
                name: (*name).to_owned(),
                file: SourcePath::new(String::from("src/lib.rs")),
                line_start: 1,
                line_end: 9,
                signature: NormalizedNode::leaf(NodeKind::Opaque),
                body: NormalizedNode::leaf(NodeKind::Opaque),
                fingerprint,
                node_count: 9,
                ..Default::default()
            })
            .collect(),
        1.0,
    )
}

/// An analysis result holding exactly these groups, for tests about what is
/// done with groups rather than about how they were found.
#[must_use]
pub fn result_with(
    exact: Vec<DuplicateGroup>,
    near: Vec<DuplicateGroup>,
    sub_exact: Vec<DuplicateGroup>,
    sub_near: Vec<DuplicateGroup>,
) -> Analysis {
    Analysis::new(
        DuplicateGroup::compute_stats_with_sub(&[], &exact, &near, &sub_exact, &sub_near),
        exact,
        near,
        sub_exact,
        sub_near,
        Vec::new(),
        BTreeSet::new(),
    )
}

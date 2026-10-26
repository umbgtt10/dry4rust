// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::analysis::Analysis;
use dry4rust::cli::ports::Ports;
use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit_kind::CodeUnitKind;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::node::node_kind::NodeKind;
use dry4rust::node::normalized_node::NormalizedNode;
use dry4rust::source_path::SourcePath;
use dry4rust_cli::system::json_baseline_store::JsonBaselineStore;
use dry4rust_cli::system::toml_config_source::TomlConfigSource;
use dry4rust_cli::system::toml_ignore_store::TomlIgnoreStore;
use dry4rust_cli::system::walkdir_source_tree::WalkdirSourceTree;
use std::collections::HashSet;
use std::path::Path;

/// The adapters `EntryPoint` composes, for tests that run the core against the
/// real file system.
#[must_use]
pub const fn disk_ports() -> Ports<'static> {
    Ports {
        sources: &WalkdirSourceTree,
        config: &TomlConfigSource,
        ignore: &TomlIgnoreStore,
        baseline: &JsonBaselineStore,
    }
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
        HashSet::new(),
    )
}

/// A path on disk as the core holds it.
#[must_use]
pub fn source_path(path: &Path) -> SourcePath {
    SourcePath::new(
        path.to_str()
            .expect("a test path is UTF-8")
            .replace('\\', "/"),
    )
}

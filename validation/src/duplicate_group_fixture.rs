// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit_kind::CodeUnitKind;
use dry4rust::duplicate_group::DuplicateGroup;
use dry4rust::fingerprint::Fingerprint;
use dry4rust::node::node_kind::NodeKind;
use dry4rust::node::normalized_node::NormalizedNode;
use std::path::PathBuf;

pub struct DuplicateGroupFixture;

impl DuplicateGroupFixture {
    /// A duplicate group with a stated fingerprint and members, for tests that
    /// care about grouping rather than about the code behind it.
    #[must_use]
    pub fn of(fingerprint: u64, names: &[&str]) -> DuplicateGroup {
        let fingerprint = Fingerprint::new(fingerprint);
        DuplicateGroup::new(
            fingerprint,
            names
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
                    ..Default::default()
                })
                .collect(),
            1.0,
        )
    }
}

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::baseline_kind::BaselineKind;
use dry4rust_cli::system::baseline_kind_record::BaselineKindRecord;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::to_string;

/// A kind as a field spells it, which is how the mirror is reached.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Spelled(#[serde(with = "BaselineKindRecord")] BaselineKind);

#[test]
fn deserialize_reads_back_every_kind_it_writes() {
    // Arrange
    let kinds = [
        BaselineKind::Exact,
        BaselineKind::Near,
        BaselineKind::SubExact,
        BaselineKind::SubNear,
    ];

    // Act & Assert
    for kind in kinds {
        let written = to_string(&Spelled(kind)).expect("a kind serializes");
        let read: Spelled = from_str(&written).expect("and reads back");
        assert_eq!(read, Spelled(kind), "round trip through {written}");
    }
}

#[test]
fn serialize_writes_a_name_rather_than_a_position() {
    // Arrange & Act
    let written = to_string(&Spelled(BaselineKind::SubExact)).expect("a kind serializes");

    // Assert
    assert_eq!(
        written, "\"sub_exact\"",
        "a persisted file cannot rest on the order the variants happen to be \
         declared in; reordering them would silently repoint every entry"
    );
}

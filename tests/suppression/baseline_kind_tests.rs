// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::suppression::baseline_kind::BaselineKind;
use serde_json::from_str;
use serde_json::to_string;

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
        let written = to_string(&kind).expect("a kind serializes");
        let read: BaselineKind = from_str(&written).expect("and reads back");
        assert_eq!(read, kind, "round trip through {written}");
    }
}

#[test]
fn fmt_names_each_kind_as_the_report_does() {
    // Arrange & Act & Assert
    assert_eq!(BaselineKind::Exact.to_string(), "exact");
    assert_eq!(BaselineKind::Near.to_string(), "near");
    assert_eq!(BaselineKind::SubExact.to_string(), "sub-function exact");
    assert_eq!(BaselineKind::SubNear.to_string(), "sub-function near");
}

#[test]
fn serialize_writes_a_name_rather_than_a_position() {
    // Arrange & Act
    let written = to_string(&BaselineKind::SubExact).expect("a kind serializes");

    // Assert
    assert_eq!(
        written, "\"sub_exact\"",
        "a persisted file cannot rest on the order the variants happen to be \
         declared in; reordering them would silently repoint every entry"
    );
}

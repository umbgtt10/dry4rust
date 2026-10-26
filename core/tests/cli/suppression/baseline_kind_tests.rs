// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::baseline_kind::BaselineKind;

#[test]
fn fmt_names_each_kind_as_the_report_does() {
    // Arrange & Act & Assert
    assert_eq!(BaselineKind::Exact.to_string(), "exact");
    assert_eq!(BaselineKind::Near.to_string(), "near");
    assert_eq!(BaselineKind::SubExact.to_string(), "sub-function exact");
    assert_eq!(BaselineKind::SubNear.to_string(), "sub-function near");
}

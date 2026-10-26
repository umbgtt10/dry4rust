// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use crate::common::result_with;
use dry4rust::cli::suppression::baseline_file::BaselineFile;
use dry4rust_cli::system::baseline_document::BaselineDocument;

#[test]
fn from_a_recording_and_back_keeps_its_version_and_every_entry() {
    // Arrange
    let recorded = BaselineFile::record(&result_with(
        vec![group(0x11, &["a", "b"])],
        vec![group(0x22, &["c", "d"])],
        vec![],
        vec![],
    ));

    // Act
    let back = BaselineFile::from(BaselineDocument::from(&recorded));

    // Assert
    assert_eq!(back.version(), recorded.version());
    assert_eq!(back.entries(), recorded.entries());
}

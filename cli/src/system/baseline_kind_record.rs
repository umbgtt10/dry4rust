// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::suppression::baseline_kind::BaselineKind;
use serde::{Deserialize, Serialize};

/// How the file spells a baseline entry's kind: by name, never by position, so
/// reordering the variants cannot repoint a recorded entry. A mirror of the
/// core's enum for serde, which a field names with
/// `#[serde(with = "BaselineKindRecord")]`.
#[derive(Serialize, Deserialize)]
#[serde(remote = "BaselineKind", rename_all = "snake_case")]
pub enum BaselineKindRecord {
    Exact,
    Near,
    SubExact,
    SubNear,
}

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::dry_binary::DryBinary;
use assert_cmd::Command;
use std::path::Path;

pub struct BaselineScenario;

impl BaselineScenario {
    #[must_use]
    pub fn command_in(root: &str) -> Command {
        DryBinary::command_in(Path::new(root))
    }
}

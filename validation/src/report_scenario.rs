// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::dry_binary::DryBinary;
use crate::fixture::Fixture;
use assert_cmd::Command;

pub struct ReportScenario;

impl ReportScenario {
    #[must_use]
    pub fn command_on(fixture: &str) -> Command {
        DryBinary::command_in(&Fixture::path(fixture))
    }
}

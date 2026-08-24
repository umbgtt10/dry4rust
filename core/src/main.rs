// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::process::ExitCode;

use dry4rust::cli::entry_point::EntryPoint;

fn main() -> ExitCode {
    EntryPoint::run(args().collect())
}

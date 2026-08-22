// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::Cli;
use std::env::args;
use std::process::ExitCode;

// cargo runs `cargo dry4rust ...` as `cargo-dry4rust dry4rust ...`, so the name
// arrives twice, while running the binary directly does not repeat it. The
// strip is conditional and positional, so a package that happens to be named
// `dry4rust` survives.
fn main() -> ExitCode {
    let arguments = args().collect::<Vec<_>>();
    let forwarded_args = if arguments.get(1).map(String::as_str) == Some("dry4rust") {
        let mut forwarded = Vec::with_capacity(arguments.len().saturating_sub(1));
        if let Some(binary) = arguments.first() {
            forwarded.push(binary.clone());
        }
        forwarded.extend(arguments.into_iter().skip(2));
        forwarded
    } else {
        arguments
    };

    match Cli::run_from_args(forwarded_args) {
        Ok(code) => code,
        Err(error) => {
            println!("error: {error:#}");
            ExitCode::from(2)
        }
    }
}

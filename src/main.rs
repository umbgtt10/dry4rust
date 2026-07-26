// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().collect::<Vec<_>>();
    let forwarded_args = if args.get(1).map(String::as_str) == Some("dry4rust") {
        let mut forwarded = Vec::with_capacity(args.len().saturating_sub(1));
        if let Some(binary) = args.first() {
            forwarded.push(binary.clone());
        }
        forwarded.extend(args.into_iter().skip(2));
        forwarded
    } else {
        args
    };

    match dry4rust::run_from_args(forwarded_args) {
        Ok(code) => code,
        Err(error) => {
            println!("error: {error:#}");
            ExitCode::from(2)
        }
    }
}

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::process::ExitCode;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::gate::Gate;
use xtask::gates::iceberg_gate::IcebergGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::process::system_command_runner::SystemCommandRunner;

const CORE_PACKAGE: &str = "cargo-dry4rust";
const VALIDATION_PACKAGE: &str = "validation";
const XTASK_PACKAGE: &str = "xtask";
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "10";

// Reading the real process argv and wiring the concrete runner are the two
// things no test can reach, so they are all this binary does.
fn main() -> ExitCode {
    if args().nth(1).as_deref() == Some("stage2") {
        run_stage2()
    } else {
        eprintln!("usage: cargo xtask stage2");
        ExitCode::FAILURE
    }
}

fn run_stage2() -> ExitCode {
    let manifest_path = workspace_manifest_path();
    let runner = SystemCommandRunner::new();
    let parser = CrapReportParser::new();

    // No --package at all. One call judges every member against its own rules:
    // core and xtask take all twenty-one, validation stands down
    // paired-test-file because it has no src/ for a test file to be named
    // after. That split lives in stern4rust.toml, so a hand-run of
    // `cargo stern4rust` from the repository root sees exactly what this sees.
    let stern = SternGate::new(&runner, manifest_path.clone(), Vec::new());

    // core only, and this one genuinely cannot cover the others. CRAP scores
    // source functions against their coverage, and validation has no source --
    // only tests. Running it there also fails outright: coverage driven with
    // `-p validation` does not build core's binary, so the tests that spawn it
    // cannot find it.
    let crap = CrapGate::new(
        &runner,
        &parser,
        manifest_path.clone(),
        vec![String::from(CORE_PACKAGE)],
        String::from(CRAP_THRESHOLD),
    );

    // All three in one call rather than one call each. Verified equivalent:
    // scoping to validation alone scans nothing and passes vacuously, because
    // it has no source files to mirror or to score, and adding it to the core
    // invocation leaves the file count unchanged.
    let measured = vec![
        String::from(CORE_PACKAGE),
        String::from(VALIDATION_PACKAGE),
        String::from(XTASK_PACKAGE),
    ];

    let twin = TwinGate::new(&runner, manifest_path.clone(), measured.clone());
    let iceberg = IcebergGate::new(
        &runner,
        manifest_path,
        measured,
        String::from(ICEBERG_THRESHOLD),
    );

    let gates: Vec<&dyn Gate> = vec![&stern, &crap, &twin, &iceberg];

    match Stage2::new(gates).run() {
        Ok(()) => {
            println!("\ndry4rust Stage 2 passed!");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn workspace_manifest_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
        .join("Cargo.toml")
        .to_string_lossy()
        .into_owned()
}

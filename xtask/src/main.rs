// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;
use xtask::boundary::boundary_rule::BoundaryRule;
use xtask::boundary::crate_root_attributes::CrateRootAttributes;
use xtask::boundary::disk_source_files::DiskSourceFiles;
use xtask::boundary::package_dependencies::PackageDependencies;
use xtask::crap::crap_report_parser::CrapReportParser;
use xtask::gates::boundary_gate::BoundaryGate;
use xtask::gates::cargo_update_gate::CargoUpdateGate;
use xtask::gates::crap_gate::CrapGate;
use xtask::gates::deny_gate::DenyGate;
use xtask::gates::dry_self_gate::DrySelfGate;
use xtask::gates::gate::Gate;
use xtask::gates::iceberg_gate::IcebergGate;
use xtask::gates::shear_gate::ShearGate;
use xtask::gates::stage2::Stage2;
use xtask::gates::stern_gate::SternGate;
use xtask::gates::twin_gate::TwinGate;
use xtask::process::system_command_runner::SystemCommandRunner;

const CLI_PACKAGE: &str = "cargo-dry4rust";
const CORE_PACKAGE: &str = "dry4rust-core";
const VALIDATION_PACKAGE: &str = "validation";
const XTASK_PACKAGE: &str = "xtask";
const ALLOWED_CRATES: [&str; 3] = ["proc-macro2", "syn", "thiserror"];
const CORE_DIRS: [[&str; 2]; 2] = [["core", "src"], ["core", "tests"]];
const CORE_ROOT: [&str; 3] = ["core", "src", "lib.rs"];
const FORBIDDEN_MODULES: [&str; 6] = [
    "std::env",
    "std::fs",
    "std::io",
    "std::net",
    "std::os",
    "std::process",
];
const FORBIDDEN_MACROS: [&str; 5] = ["dbg", "eprint", "eprintln", "print", "println"];
const REQUIRED_ATTRIBUTES: [&str; 2] = ["no_std", "forbid(unsafe_code)"];
const FORBIDDEN_METHODS: [&str; 10] = [
    "canonicalize",
    "exists",
    "is_dir",
    "is_file",
    "is_symlink",
    "metadata",
    "read_dir",
    "read_link",
    "symlink_metadata",
    "try_exists",
];
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "10";
const DRY_BINARY: &str = "cargo-dry4rust";
const DRY_EXCLUDES: [&[&str]; 5] = [
    &["cli", "tests"],
    &["core", "tests"],
    &["fixture"],
    &["validation"],
    &["xtask"],
];
const DRY_BASELINE: &str = "dry4rust-baseline.json";
const DRY_MIN_NODES: &str = "25";

// Reading the real process argv and wiring the concrete runner are the two
// things no test can reach, so they are all this binary does.
fn main() -> ExitCode {
    match args().nth(1).as_deref() {
        Some("stage2") => run_stage2(),
        Some("sync") => run_sync(),
        _ => {
            eprintln!("usage: cargo xtask <stage2|sync>");
            ExitCode::FAILURE
        }
    }
}

// Stage 1's first step: the lock file to the newest compatible versions.
fn run_sync() -> ExitCode {
    let runner = SystemCommandRunner::new();
    let update = CargoUpdateGate::new(&runner, workspace_manifest_path());
    match Stage2::new(vec![&update]).run() {
        Ok(()) => {
            println!("\ndry4rust sync done");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn run_stage2() -> ExitCode {
    let manifest_path = workspace_manifest_path();
    let runner = SystemCommandRunner::new();
    let parser = CrapReportParser::new();
    let root = workspace_root();

    // First: core stays free of technology -- only its allowed crates in its
    // manifest, `no_std` and `forbid(unsafe_code)` on its root, and no file
    // system, process, environment or stdout in its tests -- so mutation testing
    // it needs nothing but its own tests. A breach is fixed by moving code into
    // cli, which changes what every later gate, stern4rust's included, measures;
    // and it costs a second.
    let sources = DiskSourceFiles::new();
    let dependencies = PackageDependencies::new();
    let attributes = CrateRootAttributes::new();
    let boundary = BoundaryGate::new(
        &runner,
        &sources,
        &dependencies,
        &attributes,
        manifest_path.clone(),
        BoundaryRule {
            package: String::from(CORE_PACKAGE),
            source_dirs: CORE_DIRS.iter().map(|dir| below(&root, dir)).collect(),
            allowed_crates: ALLOWED_CRATES.map(String::from).to_vec(),
            crate_root: below(&root, &CORE_ROOT),
            required_attributes: REQUIRED_ATTRIBUTES.map(String::from).to_vec(),
            forbidden_modules: FORBIDDEN_MODULES.map(String::from).to_vec(),
            forbidden_macros: FORBIDDEN_MACROS.map(String::from).to_vec(),
            forbidden_methods: FORBIDDEN_METHODS.map(String::from).to_vec(),
        },
    );

    // Second, and with no --package at all: one call judges every member
    // against every rule, and stern4rust.toml names the header rather than this
    // call, so a hand-run of `cargo stern4rust` from the repository root sees
    // exactly what this sees.
    let stern = SternGate::new(&runner, manifest_path.clone(), Vec::new());

    // Third: every declared dependency used, in the section that uses it. Its
    // corrections edit manifests only, so they move nothing the gates behind it
    // measure, and it costs a second.
    let shear = ShearGate::new(&runner, manifest_path.clone());
    // Last in stage 2: its advisories change without the code changing (see DenyGate).
    let deny = DenyGate::new(&runner, manifest_path.clone());

    // Fourth, for the same reason stern4rust runs early: removing a duplicate
    // moves code between files, which changes what the three behind it measure.
    // This checkout's own dry4rust, built from source -- a tool that enforces a
    // check it does not pass is not worth installing. One run over both
    // published crates' sources, so a copy from one into the other is seen:
    // tests repeat their arrangement by design, the fixture corpus is
    // duplicated on purpose, and validation and xtask are not published. Against a baseline of what was already
    // duplicated when the gate arrived, so it fails on what a change adds. The
    // floor leaves out units whose whole body is one call or one short struct
    // literal -- accessors, two-field constructors, the `Reporter` methods that
    // each hand one section to a shared writer -- whose sameness is their
    // signature's, not a copy.
    let dry = DrySelfGate::new(
        &runner,
        String::from(DRY_BINARY),
        root.to_string_lossy().into_owned(),
        DRY_EXCLUDES
            .iter()
            .map(|exclude| below(&root, exclude).to_string_lossy().into_owned())
            .collect(),
        root.join(DRY_BASELINE).to_string_lossy().into_owned(),
        String::from(DRY_MIN_NODES),
    );

    // core and cli. validation has source -- the harness and the scenario types
    // its tests call -- but coverage driven with `-p validation` does not build
    // cli's binary. On a clean target the tests that spawn it cannot find it
    // and the run fails outright; on a used one they find whatever binary an
    // earlier run left behind, which is not a measurement of this checkout.
    let crap = CrapGate::new(
        &runner,
        &parser,
        manifest_path.clone(),
        vec![String::from(CORE_PACKAGE), String::from(CLI_PACKAGE)],
        String::from(CRAP_THRESHOLD),
    );

    // All four in one call rather than one call each, so one report covers
    // every member. validation's src/ -- the shared harness and one scenario
    // type per scenario file -- is mirrored and scored like core's and xtask's.
    let measured = vec![
        String::from(CORE_PACKAGE),
        String::from(CLI_PACKAGE),
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

    let gates: Vec<&dyn Gate> = vec![
        &boundary, &stern, &shear, &dry, &crap, &twin, &iceberg, &deny,
    ];

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
    workspace_root()
        .join("Cargo.toml")
        .to_string_lossy()
        .into_owned()
}

// Joined a component at a time, so the path carries the platform's separator
// throughout; dry4rust gives the exclusions the separator its paths have.
fn below(root: &Path, components: &[&str]) -> PathBuf {
    components
        .iter()
        .fold(root.to_path_buf(), |path, component| path.join(component))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
        .to_path_buf()
}

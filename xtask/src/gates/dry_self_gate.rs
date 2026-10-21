// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::gates::gate::Gate;
use crate::gates::gate_failure::GateFailure;
use crate::process::command_runner::CommandRunner;

const CEILING_EXCEEDED: i32 = 1;

pub struct DrySelfGate<'a> {
    runner: &'a dyn CommandRunner,
    binary: String,
    path: String,
    baseline: String,
    min_nodes: String,
}

impl<'a> DrySelfGate<'a> {
    pub const fn new(
        runner: &'a dyn CommandRunner,
        binary: String,
        path: String,
        baseline: String,
        min_nodes: String,
    ) -> Self {
        Self {
            runner,
            binary,
            path,
            baseline,
            min_nodes,
        }
    }
}

impl Gate for DrySelfGate<'_> {
    fn label(&self) -> String {
        String::from("Duplication")
    }

    fn run(&self) -> Result<(), GateFailure> {
        // Built from source rather than run from whatever is installed: the gate
        // judges this tree with the dry4rust this tree is about to ship, so a
        // check it fails is a check it would publish failing. `--bin` before the
        // `--` chooses what cargo builds; everything after it is what
        // `cargo dry4rust` would have been handed, subcommand name first.
        //
        // Zero ceilings against the baseline: what the baseline records was here
        // before the gate was, and anything beyond it is what a change added. The
        // baseline was recorded at this floor and only matches at this floor.
        let args = vec![
            String::from("run"),
            String::from("--quiet"),
            String::from("--bin"),
            self.binary.clone(),
            String::from("--"),
            String::from("dry4rust"),
            String::from("--path"),
            self.path.clone(),
            String::from("--baseline"),
            self.baseline.clone(),
            String::from("--min-nodes"),
            self.min_nodes.clone(),
            String::from("check"),
            String::from("--max-exact"),
            String::from("0"),
            String::from("--max-near"),
            String::from("0"),
        ];

        // 1 is the tool's own "a ceiling was exceeded"; 2 is a configuration or a
        // baseline it refused, and a build that failed is cargo's 101 -- neither
        // measured anything, so neither may read as a verdict.
        match self.runner.run_streaming("cargo", &args)? {
            Some(0) => Ok(()),
            Some(CEILING_EXCEEDED) => Err(GateFailure::Found(format!(
                "duplication was added beyond {}",
                self.baseline
            ))),
            code => Err(GateFailure::Exited(code)),
        }
    }
}

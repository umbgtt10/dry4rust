// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::traits::baseline_store::BaselineStore;
use crate::traits::config_source::ConfigSource;
use crate::traits::ignore_store::IgnoreStore;
use crate::traits::source_tree::SourceTree;

// Everything a run reads or writes outside itself, handed over in one piece by
// whoever composes the run: the sources, the configuration, and the two
// suppression files.
#[derive(Clone, Copy)]
pub struct Ports<'a> {
    pub sources: &'a dyn SourceTree,
    pub config: &'a dyn ConfigSource,
    pub ignore: &'a dyn IgnoreStore,
    pub baseline: &'a dyn BaselineStore,
}

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::in_memory_baseline_store::InMemoryBaselineStore;
use crate::support::in_memory_config_source::InMemoryConfigSource;
use crate::support::in_memory_ignore_store::InMemoryIgnoreStore;
use crate::support::in_memory_source_tree::InMemorySourceTree;
use dry4rust::cli::ports::Ports;

// The four fakes a run is composed of, owned together so a test can hand out
// `Ports` and still look inside the stores afterwards.
#[derive(Debug, Default)]
pub struct FakePorts {
    pub sources: InMemorySourceTree,
    pub config: InMemoryConfigSource,
    pub ignore: InMemoryIgnoreStore,
    pub baseline: InMemoryBaselineStore,
}

impl FakePorts {
    #[must_use]
    pub fn with_sources(sources: InMemorySourceTree) -> Self {
        Self {
            sources,
            ..Self::default()
        }
    }

    pub fn ports(&self) -> Ports<'_> {
        Ports {
            sources: &self.sources,
            config: &self.config,
            ignore: &self.ignore,
            baseline: &self.baseline,
        }
    }
}

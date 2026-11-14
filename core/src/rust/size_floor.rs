// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// The floors a unit must reach to be compared at all: its nodes and its
/// lines. A floor of 0 lines admits any length.
pub struct SizeFloor {
    min_nodes: usize,
    min_lines: usize,
}

impl SizeFloor {
    #[must_use]
    pub const fn new(min_nodes: usize, min_lines: usize) -> Self {
        Self {
            min_nodes,
            min_lines,
        }
    }

    #[must_use]
    pub const fn admits(&self, nodes: usize, lines: usize) -> bool {
        nodes >= self.min_nodes && lines >= self.min_lines
    }
}

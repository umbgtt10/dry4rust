// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// Branches that are near-duplicates of each other rather than exact ones.
// Each pair differs by a single operator inside the branch, so the branches
// are structurally similar without normalising to the same tree.

pub fn total_rising(items: &[i32]) -> i32 {
    let mut total = 0;
    for item in items.iter() {
        let scaled = item * 2;
        let shifted = scaled + 1;
        let squared = shifted * shifted;
        total += squared;
    }
    total
}

pub fn total_falling(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values.iter() {
        let scaled = value * 2;
        let shifted = scaled - 1;
        let squared = shifted * shifted;
        total += squared;
    }
    total
}

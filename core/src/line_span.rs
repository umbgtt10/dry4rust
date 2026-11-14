// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// The lines a code unit spans, its first and its last counted.
pub struct LineSpan {
    start: usize,
    end: usize,
}

impl LineSpan {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn lines(&self) -> usize {
        self.end.saturating_sub(self.start) + 1
    }
}

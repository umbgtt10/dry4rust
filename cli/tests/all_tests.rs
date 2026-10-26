// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT
// `clippy::assert_is_empty` landed in the 1.99 toolchain and fires on
// `assert!(x.is_empty())` assertions that predate this work across the suite;
// the tests that moved here from core carry them. Allowing it here keeps every
// existing assertion's exact value rather than rewriting unrelated tests.
#![allow(clippy::assert_is_empty)]

pub mod common;
pub mod invocation;

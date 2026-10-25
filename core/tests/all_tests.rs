// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// `clippy::assert_is_empty` landed in the 1.99 toolchain and fires on
// `assert!(x.is_empty())` assertions that predate this work across the suite.
// Allowing it here keeps every existing assertion's exact value rather than
// rewriting unrelated tests.
#![allow(clippy::assert_is_empty)]

pub mod analysis_config_tests;
pub mod analysis_tests;
pub mod cli;
pub mod code_unit_kind_tests;
pub mod command_dispatcher_tests;
pub mod common;
pub mod config_tests;
pub mod duplicate_group_tests;
pub mod error_tests;
pub mod file_config_tests;
pub mod fingerprint_tests;
pub mod near_duplicate;
pub mod node_encoder_tests;
pub mod node_tests;
pub mod output;
pub mod placeholder_order_collector_tests;
pub mod rust;
pub mod scanner_tests;
pub mod stable_hasher_tests;
pub mod sub_unit_extractor_tests;
pub mod sub_unit_tests;
pub mod suppression;
pub mod threshold_tests;

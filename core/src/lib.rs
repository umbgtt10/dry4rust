// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod analysis;
pub mod analysis_config;
pub mod analyzer;
pub mod cli;
pub mod code_unit;
pub mod code_unit_kind;
pub mod command_dispatcher;
pub mod config;
pub mod duplicate_group;
pub mod error;
pub mod file_config;
pub mod fingerprint;
pub mod near_duplicate;
pub mod node;
pub mod node_encoder;
pub mod output;
pub mod placeholder_order_collector;
pub mod rust;
pub mod scanner;
pub mod stable_hasher;
pub mod sub_unit;
pub mod sub_unit_extractor;
pub mod suppression;
pub mod threshold;

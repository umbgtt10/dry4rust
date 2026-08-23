// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub mod analysis;
pub mod analyzer;
pub mod cli;
pub mod code_unit;
pub mod command_dispatcher;
pub mod config;
pub mod error;
pub mod extractor;
pub mod fingerprint;
pub mod grouper;
pub mod ignore;
pub mod near_duplicate_finder;
pub mod node;
pub mod normalization_context;
pub mod output;
pub mod rust;
pub mod scanner;
pub mod similarity;
pub mod similarity_pair;
pub mod sub_unit_extractor;
pub mod union_find;

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

use syn::visit::Visit;

use crate::error::Error;
use crate::rust::code_unit_extractor::CodeUnitExtractor;
use crate::source_path::SourcePath;
use crate::traits::source_tree::SourceTree;
use syn::Attribute;
use syn::Ident;
use syn::parse_file as parse_syn_file;

pub use crate::code_unit::CodeUnit;
pub use crate::code_unit_kind::CodeUnitKind;

/// Check if attributes contain `#[test]`.
#[must_use]
pub fn has_test_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("test"))
}

/// Check if attributes contain `#[cfg(test)]`.
#[must_use]
pub fn has_cfg_test_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && attr
                .parse_args::<Ident>()
                .is_ok_and(|ident| ident == "test")
    })
}

/// Parse Rust source code and extract code units.
///
/// This is the core parsing entry point used by `RustAnalyzer`.
/// `path` is used for diagnostics and naming only.
/// Test code is always included but tagged with `is_test: true`;
/// filtering is handled by the caller.
pub fn parse_source(
    path: &SourcePath,
    source: &str,
    min_node_count: usize,
    min_line_count: usize,
) -> Result<Vec<CodeUnit>, Error> {
    let file = parse_syn_file(source).map_err(|e| Error::Parse {
        path: path.clone(),
        reason: e.to_string(),
    })?;

    let mut extractor = CodeUnitExtractor::new(path.clone(), min_node_count, min_line_count);
    extractor.visit_file(&file);

    Ok(extractor.into_units())
}

/// Read a single Rust file from `tree` and extract code units.
///
/// This is a lower-level convenience function. Prefer using [`crate::rust::rust_analyzer::RustAnalyzer`]
/// with [`crate::analysis::Analysis::analyze`] for the full pipeline.
pub fn parse_file(
    tree: &dyn SourceTree,
    path: &SourcePath,
    min_node_count: usize,
    min_line_count: usize,
) -> Result<Vec<CodeUnit>, Error> {
    let content = tree.read_source(path).map_err(|e| Error::Read {
        path: path.clone(),
        reason: e.to_string(),
    })?;

    parse_source(path, &content, min_node_count, min_line_count)
}

/// Parse multiple files from `tree` and collect all code units, skipping files
/// that fail to parse.
///
/// This is a lower-level convenience function. Prefer using [`crate::rust::rust_analyzer::RustAnalyzer`]
/// with [`crate::analysis::Analysis::analyze`] for the full pipeline.
#[must_use]
pub fn parse_files(
    tree: &dyn SourceTree,
    paths: &[SourcePath],
    min_node_count: usize,
    min_line_count: usize,
) -> (Vec<CodeUnit>, Vec<String>) {
    let mut units = Vec::new();
    let mut warnings = Vec::new();

    for path in paths {
        match parse_file(tree, path, min_node_count, min_line_count) {
            Ok(file_units) => units.extend(file_units),
            Err(warning) => warnings.push(warning.to_string()),
        }
    }

    (units, warnings)
}

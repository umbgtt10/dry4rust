// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnit;
use dry4rust::code_unit_kind::CodeUnitKind;
use dry4rust::rust::code_unit_extractor::CodeUnitExtractor;
use std::path::PathBuf;
use syn::parse_file;
use syn::visit::Visit;

const SOURCE: &str = "fn alpha() -> i32 {\n    let x = 1;\n    x + 1\n}\n";

fn visited(source: &str, min_node_count: usize) -> Vec<CodeUnit> {
    let file = parse_file(source).expect("parses");
    let mut extractor = CodeUnitExtractor::new(PathBuf::from("src/alpha.rs"), min_node_count, 0);
    extractor.visit_file(&file);
    extractor.into_units()
}

#[test]
fn into_units_after_visiting_a_function_holds_it_with_its_file() {
    // Arrange & Act
    let units = visited(SOURCE, 1);

    // Assert
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "alpha");
    assert_eq!(units[0].kind, CodeUnitKind::Function);
    assert_eq!(units[0].file, PathBuf::from("src/alpha.rs"));
    assert_eq!((units[0].line_start, units[0].line_end), (1, 4));
}

#[test]
fn into_units_leaves_out_a_function_below_the_node_floor() {
    // Arrange & Act
    let units = visited(SOURCE, 10_000);

    // Assert
    assert!(units.is_empty());
}

#[test]
fn into_units_of_an_extractor_that_visited_nothing_is_empty() {
    // Arrange
    let extractor = CodeUnitExtractor::new(PathBuf::from("src/alpha.rs"), 1, 0);

    // Act
    let units = extractor.into_units();

    // Assert
    assert!(units.is_empty());
}

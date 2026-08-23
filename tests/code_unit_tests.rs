// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnitKind;
use std::collections::BTreeSet;

#[test]
fn code_unit_kind_debug_names_the_variant() {
    // Arrange & Act & Assert
    assert_eq!(format!("{:?}", CodeUnitKind::Method), "Method");
    assert_eq!(format!("{:?}", CodeUnitKind::ImplBlock), "ImplBlock");
}

#[test]
fn code_unit_kind_distinguishes_functions_from_closures() {
    // Arrange & Act & Assert
    assert_ne!(CodeUnitKind::Function, CodeUnitKind::Closure);
    assert_eq!(CodeUnitKind::Function, CodeUnitKind::Function);
}

#[test]
fn display_gives_no_two_kinds_the_same_name() {
    // Arrange
    let every_kind = [
        CodeUnitKind::Function,
        CodeUnitKind::Method,
        CodeUnitKind::Closure,
        CodeUnitKind::Class,
        CodeUnitKind::ImplBlock,
        CodeUnitKind::TraitImplBlock,
        CodeUnitKind::IfBranch,
        CodeUnitKind::MatchArm,
        CodeUnitKind::LoopBody,
        CodeUnitKind::Block,
    ];

    // Act
    let names: BTreeSet<String> = every_kind.iter().map(ToString::to_string).collect();

    // Assert
    assert_eq!(
        names.len(),
        every_kind.len(),
        "a report that calls two kinds the same thing cannot be acted on"
    );
}

#[test]
fn display_names_every_kind_the_way_a_report_should_read() {
    // Arrange
    let every_kind = [
        (CodeUnitKind::Function, "function"),
        (CodeUnitKind::Method, "method"),
        (CodeUnitKind::Closure, "closure"),
        (CodeUnitKind::Class, "class"),
        (CodeUnitKind::ImplBlock, "impl block"),
        (CodeUnitKind::TraitImplBlock, "trait impl block"),
        (CodeUnitKind::IfBranch, "if branch"),
        (CodeUnitKind::MatchArm, "match arm"),
        (CodeUnitKind::LoopBody, "loop body"),
        (CodeUnitKind::Block, "block"),
    ];

    // Act
    let rendered: Vec<String> = every_kind.iter().map(|(k, _)| k.to_string()).collect();

    // Assert
    let expected: Vec<&str> = every_kind.iter().map(|(_, name)| *name).collect();
    assert_eq!(
        rendered, expected,
        "these strings are what a user reads in a report, so they are a \
         contract rather than a detail"
    );
}

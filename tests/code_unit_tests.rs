// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::code_unit::CodeUnitKind;

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

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs;
use tempfile::TempDir;
use validation::fixture::Fixture;

#[test]
fn analysed_leaves_sub_function_analysis_off() {
    // Arrange & Act
    let (config, result) = Fixture::analysed("sub_function_near_dupes");

    // Assert
    assert!(!config.sub_function());
    assert!(result.sub_near_groups().is_empty());
}

#[test]
fn analysed_over_a_duplicated_fixture_finds_its_one_exact_group() {
    // Arrange & Act
    let (_, result) = Fixture::analysed("exact_dupes");

    // Assert
    assert_eq!(result.exact_groups().len(), 1);
    assert_eq!(result.exact_groups()[0].members().len(), 3);
}

#[test]
fn analysed_roots_its_configuration_at_the_fixture() {
    // Arrange & Act
    let (config, _) = Fixture::analysed("no_dupes");

    // Assert
    assert_eq!(config.root(), Fixture::path("no_dupes"));
}

#[test]
fn analysed_with_sub_function_turns_sub_function_analysis_on() {
    // Arrange & Act
    let (config, result) = Fixture::analysed_with_sub_function("sub_function_near_dupes");

    // Assert
    assert!(config.sub_function());
    assert_eq!(result.sub_near_groups().len(), 1);
}

#[test]
fn duplicated_crate_in_a_temp_dir_makes_the_duplicated_fixture_its_crate_root() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");

    // Act
    Fixture::duplicated_crate_in(&tmp);

    // Assert
    let written = fs::read_to_string(tmp.path().join("src/lib.rs")).expect("the root is written");
    let original = fs::read_to_string(Fixture::path("exact_dupes").join("src/target.rs"))
        .expect("the fixture is readable");
    assert_eq!(written, original);
}

#[test]
fn path_of_a_corpus_crate_names_a_directory_under_fixture() {
    // Arrange & Act
    let path = Fixture::path("exact_dupes");

    // Assert
    assert!(path.ends_with("fixture/exact_dupes"));
    assert!(path.join("Cargo.toml").is_file());
}

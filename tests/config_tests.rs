// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::config::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn config_with_exclude_tests() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dupes.toml"),
        r#"
        exclude_tests = true
        "#,
    )
    .unwrap();
    let config = Config::load(tmp.path());
    assert!(config.exclude_tests);
}

#[test]
fn config_with_min_lines() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dupes.toml"),
        r#"
        min_lines = 5
        "#,
    )
    .unwrap();
    let config = Config::load(tmp.path());
    assert_eq!(config.min_lines, 5);
}

#[test]
fn config_with_percentage_thresholds() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dupes.toml"),
        r#"
        max_exact_percent = 5.0
        max_near_percent = 10.5
        "#,
    )
    .unwrap();
    let config = Config::load(tmp.path());
    assert_eq!(config.max_exact_percent, Some(5.0));
    assert_eq!(config.max_near_percent, Some(10.5));
}

#[test]
fn config_with_thresholds() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dupes.toml"),
        r#"
        max_exact_duplicates = 0
        max_near_duplicates = 5
        "#,
    )
    .unwrap();
    let config = Config::load(tmp.path());
    assert_eq!(config.max_exact_duplicates, Some(0));
    assert_eq!(config.max_near_duplicates, Some(5));
}

#[test]
fn default_config() {
    let config = Config::default();
    assert_eq!(config.min_nodes, 10);
    assert!((config.similarity_threshold - 0.9).abs() < f64::EPSILON);
    assert!(config.exclude.is_empty());
}

#[test]
fn dupes_toml_overrides_cargo_toml() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("Cargo.toml"),
        r#"
        [package]
        name = "test"
        version = "0.1.0"
        edition = "2021"

        [package.metadata.dupes]
        min_nodes = 15
        "#,
    )
    .unwrap();
    fs::write(
        tmp.path().join("dupes.toml"),
        r#"
        min_nodes = 25
        "#,
    )
    .unwrap();
    let config = Config::load(tmp.path());
    assert_eq!(config.min_nodes, 25);
}

#[test]
fn load_from_cargo_toml_metadata() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("Cargo.toml"),
        r#"
        [package]
        name = "test"
        version = "0.1.0"
        edition = "2021"

        [package.metadata.dupes]
        min_nodes = 15
        similarity_threshold = 0.75
        "#,
    )
    .unwrap();
    let config = Config::load(tmp.path());
    assert_eq!(config.min_nodes, 15);
    assert!((config.similarity_threshold - 0.75).abs() < f64::EPSILON);
}

#[test]
fn load_from_dupes_toml() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("dupes.toml"),
        r#"
        min_nodes = 20
        similarity_threshold = 0.9
        exclude = ["tests"]
        "#,
    )
    .unwrap();
    let config = Config::load(tmp.path());
    assert_eq!(config.min_nodes, 20);
    assert!((config.similarity_threshold - 0.9).abs() < f64::EPSILON);
    assert_eq!(config.exclude, vec!["tests".to_string()]);
}

#[test]
fn load_no_config_files() {
    let tmp = TempDir::new().unwrap();
    let config = Config::load(tmp.path());
    assert_eq!(config.min_nodes, 10); // default
}

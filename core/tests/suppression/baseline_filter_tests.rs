// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::config::Config;
use dry4rust::suppression::baseline_entry::BaselineEntry;
use dry4rust::suppression::baseline_file::BaselineFile;
use dry4rust::suppression::baseline_file::FORMAT_VERSION;
use dry4rust::suppression::baseline_filter::BaselineFilter;
use dry4rust::suppression::baseline_kind::BaselineKind;
use std::path::PathBuf;
use tempfile::TempDir;

fn recorded_in(root: &TempDir, entries: Vec<BaselineEntry>) -> Config {
    let file = BaselineFile {
        version: FORMAT_VERSION,
        entries,
    };
    file.save(&root.path().join("dry4rust-baseline.json"))
        .expect("the baseline is written");
    Config {
        root: root.path().to_path_buf(),
        baseline: Some(PathBuf::from("dry4rust-baseline.json")),
        ..Config::default()
    }
}

#[test]
fn is_in_effect_is_false_when_no_baseline_is_configured() {
    // Arrange
    let config = Config::default();

    // Act
    let filter = BaselineFilter::load(&config).expect("no baseline is not a failure");

    // Assert
    assert!(
        !filter.is_in_effect(),
        "adding the feature must not change a run that never asked for it"
    );
}

#[test]
fn is_in_effect_is_true_once_one_is_loaded() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let config = recorded_in(&tmp, Vec::new());

    // Act
    let filter = BaselineFilter::load(&config).expect("the baseline loads");

    // Assert
    assert!(filter.is_in_effect());
}

#[test]
fn load_of_a_baseline_that_is_not_there_fails_rather_than_suppressing_nothing() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let config = Config {
        root: tmp.path().to_path_buf(),
        baseline: Some(PathBuf::from("dry4rust-baseline.json")),
        ..Config::default()
    };

    // Act
    let outcome = BaselineFilter::load(&config);

    // Assert
    assert!(
        outcome.is_err(),
        "a typo in a CI flag would otherwise read as a codebase with nothing \
         inherited"
    );
}

#[test]
fn none_suppresses_nothing_and_says_it_is_not_in_effect() {
    // Arrange
    let filter = BaselineFilter::none();
    let groups = vec![group(0x1234, &["one", "two"])];

    // Act
    let kept = filter.retain_new(BaselineKind::Exact, groups);

    // Assert
    assert!(!filter.is_in_effect());
    assert_eq!(kept.len(), 1);
}

#[test]
fn retain_new_drops_a_group_the_baseline_recorded() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let inherited = group(0x1234, &["one", "two"]);
    let config = recorded_in(
        &tmp,
        vec![BaselineEntry::of(BaselineKind::Exact, &inherited)],
    );
    let filter = BaselineFilter::load(&config).expect("the baseline loads");

    // Act
    let kept = filter.retain_new(BaselineKind::Exact, vec![inherited]);

    // Assert
    assert!(kept.is_empty());
}

#[test]
fn retain_new_keeps_a_group_recorded_under_another_kind() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let same = group(0x1234, &["one", "two"]);
    let config = recorded_in(&tmp, vec![BaselineEntry::of(BaselineKind::SubExact, &same)]);
    let filter = BaselineFilter::load(&config).expect("the baseline loads");

    // Act
    let kept = filter.retain_new(BaselineKind::Exact, vec![same]);

    // Assert
    assert_eq!(kept.len(), 1);
}

#[test]
fn retain_new_keeps_a_group_the_baseline_never_saw() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let config = recorded_in(
        &tmp,
        vec![BaselineEntry::of(
            BaselineKind::Exact,
            &group(0x1234, &["one", "two"]),
        )],
    );
    let filter = BaselineFilter::load(&config).expect("the baseline loads");
    let added = group(0x5678, &["three", "four"]);

    // Act
    let kept = filter.retain_new(BaselineKind::Exact, vec![added]);

    // Assert
    assert_eq!(kept.len(), 1, "new duplication is the whole point");
    assert_eq!(kept[0].fingerprint.value(), 0x5678);
}

#[test]
fn retain_new_keeps_a_recorded_group_that_has_since_grown() {
    // Arrange
    let tmp = TempDir::new().expect("temp dir");
    let config = recorded_in(
        &tmp,
        vec![BaselineEntry::of(
            BaselineKind::Exact,
            &group(0x1234, &["one", "two"]),
        )],
    );
    let filter = BaselineFilter::load(&config).expect("the baseline loads");

    // Act
    let kept = filter.retain_new(
        BaselineKind::Exact,
        vec![group(0x1234, &["one", "two", "three"])],
    );

    // Assert
    assert_eq!(
        kept.len(),
        1,
        "a third copy of an inherited duplicate is duplication that was added"
    );
}

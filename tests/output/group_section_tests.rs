// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::common::group;
use dry4rust::grouper::DuplicateGroup;
use dry4rust::output::group_section::GroupSection;
use std::path::Path;
use std::path::PathBuf;

fn parented(name: &str, parent: &str) -> DuplicateGroup {
    let mut group = group(0x11, &[name]);
    group.members[0].parent_name = Some(parent.to_owned());
    group
}

fn written(section: &GroupSection, groups: &[DuplicateGroup], base: Option<&Path>) -> String {
    let mut buf = Vec::new();
    section
        .write(groups, base, &mut buf)
        .expect("writing succeeds");
    String::from_utf8(buf).expect("utf-8")
}

#[test]
fn write_names_the_parent_only_where_the_section_is_sub_function() {
    // Arrange
    let groups = vec![parented("for body", "total_rising")];

    // Act
    let exact = written(&GroupSection::exact(), &groups, None);
    let sub_exact = written(&GroupSection::sub_exact(), &groups, None);

    // Assert
    assert!(
        !exact.contains("in total_rising"),
        "a top-level unit has no parent to name, got: {exact}"
    );
    assert!(sub_exact.contains("in total_rising"), "{sub_exact}");
}

#[test]
fn write_numbers_the_groups_from_one() {
    // Arrange
    let groups = vec![group(0x11, &["a"]), group(0x22, &["b"])];

    // Act
    let output = written(&GroupSection::exact(), &groups, None);

    // Assert
    assert!(
        output.contains("Group 1 (fingerprint: 0000000000000011"),
        "{output}"
    );
    assert!(
        output.contains("Group 2 (fingerprint: 0000000000000022"),
        "{output}"
    );
}

#[test]
fn write_over_an_empty_exact_section_says_none_were_found() {
    // Arrange & Act
    let output = written(&GroupSection::exact(), &[], None);

    // Assert
    assert_eq!(output, "No exact duplicates found.\n");
}

#[test]
fn write_over_an_empty_near_section_says_none_were_found() {
    // Arrange & Act
    let output = written(&GroupSection::near(), &[], None);

    // Assert
    assert_eq!(output, "No near duplicates found.\n");
}

#[test]
fn write_over_an_empty_sub_section_says_nothing_at_all() {
    // Arrange & Act
    let sub_exact = written(&GroupSection::sub_exact(), &[], None);
    let sub_near = written(&GroupSection::sub_near(), &[], None);

    // Assert
    assert!(
        sub_exact.is_empty() && sub_near.is_empty(),
        "sub-function analysis is opt-in, so 'none found' would suggest it ran"
    );
}

#[test]
fn write_shows_a_member_path_relative_to_the_base_it_is_given() {
    // Arrange
    let mut groups = vec![group(0x11, &["a"])];
    groups[0].members[0].file = PathBuf::from("/project/src/lib.rs");

    // Act
    let output = written(&GroupSection::exact(), &groups, Some(Path::new("/project")));

    // Assert
    assert!(output.contains("at src/lib.rs:1-9"), "{output}");
    assert!(!output.contains("/project/src"), "{output}");
}

#[test]
fn write_states_the_similarity_only_where_the_section_is_scored() {
    // Arrange
    let groups = vec![group(0x11, &["a"])];

    // Act
    let exact = written(&GroupSection::exact(), &groups, None);
    let near = written(&GroupSection::near(), &groups, None);

    // Assert
    assert!(!exact.contains("similarity:"), "{exact}");
    assert!(near.contains("similarity: 100%"), "{near}");
}

#[test]
fn write_underlines_the_title_to_its_own_length() {
    // Arrange
    let groups = vec![group(0x11, &["a"])];

    // Act
    let output = written(&GroupSection::exact(), &groups, None);

    // Assert
    assert!(
        output.starts_with("Exact Duplicates\n================\n\n"),
        "{output}"
    );
}

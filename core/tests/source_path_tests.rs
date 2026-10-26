// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::source_path::SourcePath;
use std::cmp::Ordering;

fn path(text: &str) -> SourcePath {
    SourcePath::new(String::from(text))
}

#[test]
fn as_given_of_a_native_path_keeps_its_own_separators() {
    // Arrange
    let windows = SourcePath::native(String::from("C:\\work\\src\\lib.rs"), '\\');

    // Act
    let given = windows.as_given();

    // Assert
    assert_eq!(given, "C:\\work\\src\\lib.rs");
    assert_eq!(windows.as_str(), "C:/work/src/lib.rs");
}

#[test]
fn as_str_returns_the_text_it_was_given() {
    // Arrange
    let source = path("/p/src/lib.rs");

    // Act
    let text = source.as_str();

    // Assert
    assert_eq!(text, "/p/src/lib.rs");
}

#[test]
fn cmp_orders_paths_by_component_rather_than_by_byte() {
    // Arrange
    let nested = path("/a/b");
    let sibling = path("/a-b");

    // Act
    let order = nested.cmp(&sibling);

    // Assert
    assert_eq!(order, Ordering::Less);
}

#[test]
fn default_is_the_empty_path() {
    // Arrange & Act
    let empty = SourcePath::default();

    // Assert
    assert_eq!(empty.as_str(), "");
    assert_eq!(empty.file_name(), None);
}

#[test]
fn eq_of_a_native_path_and_its_slashed_text_is_true() {
    // Arrange
    let windows = SourcePath::native(String::from("C:\\work\\src"), '\\');

    // Act
    let same = windows == path("C:/work/src");

    // Assert
    assert!(same, "two spellings of one path are one path");
}

#[test]
fn eq_with_a_trailing_separator_is_the_same_path() {
    // Arrange
    let plain = path("/p/src");
    let trailing = path("/p/src/");

    // Act
    let same = plain == trailing;

    // Assert
    assert!(same);
}

#[test]
fn extension_of_a_dotfile_returns_none() {
    // Arrange
    let source = path("/p/.rs");

    // Act
    let extension = source.extension();

    // Assert
    assert_eq!(extension, None);
}

#[test]
fn extension_of_a_file_without_one_returns_none() {
    // Arrange
    let source = path("/p/src/Makefile");

    // Act
    let extension = source.extension();

    // Assert
    assert_eq!(extension, None);
}

#[test]
fn extension_of_a_rust_file_returns_rs() {
    // Arrange
    let source = path("/p/src/lib.rs");

    // Act
    let extension = source.extension();

    // Assert
    assert_eq!(extension, Some("rs"));
}

#[test]
fn file_name_of_a_root_returns_none() {
    // Arrange
    let source = path("/");

    // Act
    let name = source.file_name();

    // Assert
    assert_eq!(name, None);
}

#[test]
fn given_below_a_base_it_is_not_under_returns_none() {
    // Arrange
    let base = SourcePath::native(String::from("C:\\work"), '\\');
    let file = SourcePath::native(String::from("C:\\elsewhere\\lib.rs"), '\\');

    // Act
    let rest = file.given_below(&base);

    // Assert
    assert_eq!(rest, None);
}

#[test]
fn given_below_a_native_base_returns_the_rest_as_given() {
    // Arrange
    let base = SourcePath::native(String::from("C:\\work"), '\\');
    let file = SourcePath::native(String::from("C:\\work\\src\\lib.rs"), '\\');

    // Act
    let rest = file.given_below(&base);

    // Assert
    assert_eq!(rest, Some("src\\lib.rs"));
}

#[test]
fn is_absolute_of_a_drive_path_is_true() {
    // Arrange
    let windows = path("C:/repo/dry4rust-baseline.json");

    // Act
    let absolute = windows.is_absolute();

    // Assert
    assert!(absolute, "a Windows path reaches the core as `C:/...`");
}

#[test]
fn is_absolute_of_a_name_holding_a_colon_is_false() {
    // Arrange
    let named = path("c:recorded.json");

    // Act
    let absolute = named.is_absolute();

    // Assert
    assert!(!absolute, "a drive needs its separator, as `C:/` does");
}

#[test]
fn is_absolute_of_a_relative_path_is_false() {
    // Arrange
    let relative = path("ci/recorded.json");

    // Act
    let absolute = relative.is_absolute();

    // Assert
    assert!(!absolute);
}

#[test]
fn is_absolute_of_a_rooted_path_is_true() {
    // Arrange
    let rooted = path("/projects/thing");

    // Act
    let absolute = rooted.is_absolute();

    // Assert
    assert!(absolute);
}

#[test]
fn join_of_an_absolute_name_replaces_the_path() {
    // Arrange & Act
    let joined = path("/p").join("/q");

    // Assert
    assert_eq!(joined.as_str(), "/q");
}

#[test]
fn join_onto_a_directory_adds_the_name_below_it() {
    // Arrange & Act
    let joined = path("/p").join("src");

    // Assert
    assert_eq!(joined, path("/p/src"));
}

#[test]
fn join_onto_a_native_path_adds_its_own_separator_to_the_given_text() {
    // Arrange
    let windows = SourcePath::native(String::from("C:\\work"), '\\');

    // Act
    let joined = windows.join("ci/recorded.json");

    // Assert
    assert_eq!(joined.to_string(), "C:\\work\\ci/recorded.json");
    assert_eq!(joined.as_str(), "C:/work/ci/recorded.json");
}

#[test]
fn join_onto_a_native_path_ending_in_a_slash_adds_no_separator() {
    // Arrange
    let windows = SourcePath::native(String::from("C:/work/"), '\\');

    // Act
    let joined = windows.join("dry4rust-baseline.json");

    // Assert
    assert_eq!(joined.to_string(), "C:/work/dry4rust-baseline.json");
}

#[test]
fn join_onto_a_path_ending_in_a_separator_adds_none() {
    // Arrange & Act
    let joined = path("/p/").join("src");

    // Assert
    assert_eq!(joined.as_str(), "/p/src");
}

#[test]
fn parent_of_a_file_returns_its_directory() {
    // Arrange
    let source = path("/p/src/lib.rs");

    // Act
    let parent = source.parent();

    // Assert
    assert_eq!(parent, Some(path("/p/src")));
}

#[test]
fn parent_of_a_name_alone_returns_the_empty_path() {
    // Arrange
    let source = path("Cargo.toml");

    // Act
    let parent = source.parent();

    // Assert
    assert_eq!(
        parent.map(|parent| String::from(parent.as_str())),
        Some(String::new())
    );
}

#[test]
fn parent_of_a_native_path_keeps_its_given_text() {
    // Arrange
    let windows = SourcePath::native(String::from("C:\\work\\src\\lib.rs"), '\\');

    // Act
    let parent = windows.parent();

    // Assert
    let parent = parent.expect("a file has a directory");
    assert_eq!(parent.to_string(), "C:\\work\\src");
    assert_eq!(parent.as_str(), "C:/work/src");
}

#[test]
fn parent_of_a_top_level_entry_returns_the_root() {
    // Arrange
    let source = path("/p");

    // Act
    let parent = source.parent();

    // Assert
    assert_eq!(
        parent.map(|parent| String::from(parent.as_str())),
        Some(String::from("/"))
    );
}

#[test]
fn parent_of_the_root_returns_none() {
    // Arrange
    let source = path("/");

    // Act
    let parent = source.parent();

    // Assert
    assert_eq!(parent, None);
}

#[test]
fn strip_prefix_of_a_sibling_sharing_the_prefix_text_returns_none() {
    // Arrange
    let base = path("/home/user/project");
    let source = path("/home/user/projectx/src/lib.rs");

    // Act
    let rest = source.strip_prefix(&base);

    // Assert
    assert_eq!(rest, None);
}

#[test]
fn strip_prefix_under_the_base_returns_the_rest_as_written() {
    // Arrange
    let base = path("/home/user/project");
    let source = path("/home/user/project//src/lib.rs");

    // Act
    let rest = source.strip_prefix(&base);

    // Assert
    assert_eq!(rest, Some("src/lib.rs"));
}

#[test]
fn strip_prefix_with_a_relative_base_under_an_absolute_path_returns_none() {
    // Arrange
    let base = path("home/user");
    let source = path("/home/user/lib.rs");

    // Act
    let rest = source.strip_prefix(&base);

    // Assert
    assert_eq!(rest, None);
}

#[test]
fn to_string_displays_the_text_it_was_given() {
    // Arrange
    let source = path("/p/src/lib.rs");

    // Act
    let text = source.to_string();

    // Assert
    assert_eq!(text, "/p/src/lib.rs");
}

#[test]
fn to_string_of_a_native_path_shows_it_as_given() {
    // Arrange
    let windows = SourcePath::native(String::from("C:\\work\\src\\lib.rs"), '\\');

    // Act
    let shown = windows.to_string();

    // Assert
    assert_eq!(shown, "C:\\work\\src\\lib.rs");
}

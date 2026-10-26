// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust_cli::system::portable_path::PortablePath;
use std::path::MAIN_SEPARATOR;
use std::path::Path;

#[cfg(target_os = "linux")]
mod not_utf8 {
    use dry4rust_cli::system::portable_path::PortablePath;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    #[test]
    fn lossy_of_a_path_that_is_not_utf8_replaces_what_is_not() {
        // Arrange
        let path = Path::new(OsStr::from_bytes(b"/p/\xff.rs"));

        // Act
        let source = PortablePath::lossy(path);

        // Assert
        assert_eq!(source.as_str(), "/p/\u{FFFD}.rs");
    }

    #[test]
    fn of_a_path_that_is_not_utf8_returns_none() {
        // Arrange
        let path = Path::new(OsStr::from_bytes(b"/p/\xff.rs"));

        // Act
        let source = PortablePath::of(path);

        // Assert
        assert!(source.is_none());
    }
}

#[test]
fn lossy_of_a_utf8_path_keeps_its_text() {
    // Arrange
    let path = Path::new("/p/src/lib.rs");

    // Act
    let source = PortablePath::lossy(path);

    // Assert
    assert_eq!(source.as_str(), "/p/src/lib.rs");
}

#[test]
fn of_a_utf8_path_keeps_its_text() {
    // Arrange
    let path = Path::new("/p/src/lib.rs");

    // Act
    let source = PortablePath::of(path);

    // Assert
    assert_eq!(
        source.map(|source| source.to_string()),
        Some(String::from("/p/src/lib.rs"))
    );
}

// `\` separates only on Windows; elsewhere it is part of a name and must stay.
#[test]
fn of_text_turns_a_backslash_into_a_slash_only_where_it_separates() {
    // Arrange
    let text = "project\\src\\lib.rs";

    // Act
    let source = PortablePath::of_text(text);

    // Assert
    let expected = if MAIN_SEPARATOR == '\\' {
        "project/src/lib.rs"
    } else {
        "project\\src\\lib.rs"
    };
    assert_eq!(source.as_str(), expected);
}

#[test]
fn pattern_turns_a_backslash_into_a_slash_only_where_it_separates() {
    // Arrange
    let text = "src\\generated";

    // Act
    let pattern = PortablePath::pattern(text);

    // Assert
    let expected = if MAIN_SEPARATOR == '\\' {
        "src/generated"
    } else {
        "src\\generated"
    };
    assert_eq!(pattern, expected);
}

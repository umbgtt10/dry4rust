// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::cmp::Ordering;
use core::fmt;

// A path held as text, because the core has no `std::path`. `/` is its only
// separator: the edge that hands paths in normalises the platform's own to it.
// It compares by component, so `/a/b` and `/a/b/` are one path; `.` and `..`
// are compared as written, which `std::path` would normalise -- neither walk
// nor command line hands one over inside a path.
#[derive(Debug, Clone, Default)]
pub struct SourcePath {
    text: String,
}

impl SourcePath {
    #[must_use]
    pub const fn new(text: String) -> Self {
        Self { text }
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        self.text.as_str()
    }

    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        let name = self.file_name()?;
        match name.rsplit_once('.') {
            Some((stem, extension)) if !stem.is_empty() => Some(extension),
            _ => None,
        }
    }

    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.components()
            .last()
            .copied()
            .filter(|name| *name != "..")
    }

    // Absolute where it starts at a root -- `/`, or a drive such as `C:/` once
    // the edge has turned a Windows path's separators into `/`. A relative path
    // is read from wherever it is joined to.
    #[must_use]
    pub fn is_absolute(&self) -> bool {
        self.has_root() || self.has_drive()
    }

    #[must_use]
    pub fn join(&self, name: &str) -> Self {
        if self.text.is_empty() || name.starts_with(Self::is_separator) {
            return Self::new(String::from(name));
        }
        if self.text.ends_with(Self::is_separator) {
            return Self::new(format!("{}{name}", self.text));
        }
        Self::new(format!("{}/{name}", self.text))
    }

    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        if self.components().is_empty() {
            return None;
        }
        let trimmed = self.text.trim_end_matches(Self::is_separator);
        let cut = trimmed.rfind(Self::is_separator).unwrap_or(0);
        let head = trimmed[..cut].trim_end_matches(Self::is_separator);
        if head.is_empty() && self.has_root() {
            return Some(Self::new(String::from(&self.text[..1])));
        }
        Some(Self::new(String::from(head)))
    }

    #[must_use]
    pub fn strip_prefix(&self, base: &Self) -> Option<&str> {
        if self.has_root() != base.has_root() {
            return None;
        }
        let mut rest = self.text.as_str();
        for expected in base.components() {
            rest = rest.trim_start_matches(Self::is_separator);
            let end = rest.find(Self::is_separator).unwrap_or(rest.len());
            if &rest[..end] != expected {
                return None;
            }
            rest = &rest[end..];
        }
        Some(rest.trim_start_matches(Self::is_separator))
    }

    fn components(&self) -> Vec<&str> {
        self.text
            .split(Self::is_separator)
            .filter(|component| !component.is_empty())
            .collect()
    }

    const fn has_drive(&self) -> bool {
        let bytes = self.text.as_bytes();
        bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/'
    }

    fn has_root(&self) -> bool {
        self.text.starts_with(Self::is_separator)
    }

    const fn is_separator(character: char) -> bool {
        character == '/'
    }
}

impl fmt::Display for SourcePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

impl PartialEq for SourcePath {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for SourcePath {}

impl PartialOrd for SourcePath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SourcePath {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .has_root()
            .cmp(&self.has_root())
            .then_with(|| self.components().cmp(&other.components()))
    }
}

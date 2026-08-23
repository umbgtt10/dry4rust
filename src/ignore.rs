// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::fingerprint::Fingerprint;
use crate::grouper::DuplicateGroup;
use std::fs;
use std::io::Error;
use toml::from_str;
use toml::to_string_pretty;

const IGNORE_FILE_NAME: &str = ".dry4rust-ignore.toml";

/// An entry in the ignore file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IgnoreEntry {
    /// The fingerprint of the duplicated code.
    pub fingerprint: String,
    /// Optional reason for ignoring.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Names of the code units in the group (for documentation).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}

/// The ignore file structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IgnoreFile {
    #[serde(default)]
    pub ignore: Vec<IgnoreEntry>,
}

/// Get the path to the ignore file for a project root.
#[must_use]
pub fn ignore_file_path(root: &Path) -> PathBuf {
    root.join(IGNORE_FILE_NAME)
}

/// Load the ignore file from disk.
#[must_use]
pub fn load_ignore_file(root: &Path) -> IgnoreFile {
    let path = ignore_file_path(root);
    if !path.exists() {
        return IgnoreFile::default();
    }
    fs::read_to_string(&path).map_or_else(
        |_| IgnoreFile::default(),
        |content| from_str(&content).unwrap_or_default(),
    )
}

/// Save the ignore file to disk.
pub fn save_ignore_file(root: &Path, ignore_file: &IgnoreFile) -> std::io::Result<()> {
    let path = ignore_file_path(root);
    let content = to_string_pretty(ignore_file)
        .map_err(|e| Error::other(format!("Failed to serialize ignore file: {e}")))?;
    fs::write(path, content)
}

/// Add an ignore entry for a fingerprint.
pub fn add_ignore(
    ignore_file: &mut IgnoreFile,
    fingerprint: &Fingerprint,
    reason: Option<String>,
    members: Vec<String>,
) {
    let fp_hex = fingerprint.to_hex();
    // Don't add duplicates
    if ignore_file.ignore.iter().any(|e| e.fingerprint == fp_hex) {
        return;
    }
    ignore_file.ignore.push(IgnoreEntry {
        fingerprint: fp_hex,
        reason,
        members,
    });
}

/// Remove an ignore entry by fingerprint.
pub fn remove_ignore(ignore_file: &mut IgnoreFile, fingerprint: &str) -> bool {
    let initial_len = ignore_file.ignore.len();
    ignore_file.ignore.retain(|e| e.fingerprint != fingerprint);
    ignore_file.ignore.len() < initial_len
}

/// Check if a fingerprint is ignored.
#[must_use]
pub fn is_ignored(ignore_file: &IgnoreFile, fingerprint: &Fingerprint) -> bool {
    let fp_hex = fingerprint.to_hex();
    ignore_file.ignore.iter().any(|e| e.fingerprint == fp_hex)
}

/// Filter out ignored groups from a list of duplicate groups.
#[must_use]
pub fn filter_ignored(
    groups: Vec<DuplicateGroup>,
    ignore_file: &IgnoreFile,
) -> Vec<DuplicateGroup> {
    groups
        .into_iter()
        .filter(|g| !is_ignored(ignore_file, &g.fingerprint))
        .collect()
}

/// Find ignore entries whose fingerprint doesn't match any live group.
#[must_use]
pub fn find_stale_entries<'a>(
    ignore_file: &'a IgnoreFile,
    live_fingerprints: &HashSet<Fingerprint>,
) -> Vec<&'a IgnoreEntry> {
    ignore_file
        .ignore
        .iter()
        .filter(|entry| {
            !Fingerprint::from_hex(&entry.fingerprint)
                .is_some_and(|fp| live_fingerprints.contains(&fp)) // invalid hex is always stale
        })
        .collect()
}

/// Remove and return stale ignore entries.
pub fn remove_stale_entries(
    ignore_file: &mut IgnoreFile,
    live_fingerprints: &HashSet<Fingerprint>,
) -> Vec<IgnoreEntry> {
    let mut stale = Vec::new();
    let mut live = Vec::new();
    for entry in ignore_file.ignore.drain(..) {
        let is_live = Fingerprint::from_hex(&entry.fingerprint)
            .is_some_and(|fp| live_fingerprints.contains(&fp));
        if is_live {
            live.push(entry);
        } else {
            stale.push(entry);
        }
    }
    ignore_file.ignore = live;
    stale
}

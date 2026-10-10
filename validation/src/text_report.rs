// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub struct TextReport;

impl TextReport {
    /// The first fingerprint a text report names, which is how these tests learn a
    /// value only the tool can produce.
    #[must_use]
    pub fn fingerprint_in(report: &str) -> String {
        report
            .lines()
            .find(|line| line.contains("fingerprint:"))
            .and_then(|line| {
                let start = line.find("fingerprint: ")? + 13;
                let end = line[start..].find(',')?;
                Some(line[start..start + end].to_owned())
            })
            .expect("the report names a fingerprint")
    }
}

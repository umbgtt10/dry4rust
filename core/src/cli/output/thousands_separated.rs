// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::ToString;
use core::fmt;
use core::fmt::Write as _;

/// A count with a comma between each group of three digits, as the text
/// report states its totals: 1234567 as `1,234,567`.
pub struct ThousandsSeparated {
    pub count: usize,
}

impl fmt::Display for ThousandsSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = self.count.to_string();
        for (index, digit) in digits.chars().enumerate() {
            if index > 0 && (digits.len() - index).is_multiple_of(3) {
                f.write_char(',')?;
            }
            f.write_char(digit)?;
        }
        Ok(())
    }
}

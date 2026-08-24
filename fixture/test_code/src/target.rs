// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub fn sum_values(items: &[i32]) -> i32 {
    let mut total = 0;
    for item in items {
        if *item > 0 {
            total += item;
        }
    }
    total
}

pub fn count_values(items: &[i32]) -> i32 {
    let mut total = 0;
    for item in items {
        if *item > 0 {
            total += item;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    pub fn test_helper(items: &[i32]) -> i32 {
        let mut total = 0;
        for item in items {
            if *item > 0 {
                total += item;
            }
        }
        total
    }
}

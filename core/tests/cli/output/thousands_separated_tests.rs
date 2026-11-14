// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::cli::output::thousands_separated::ThousandsSeparated;

fn written(count: usize) -> String {
    ThousandsSeparated { count }.to_string()
}

#[test]
fn fmt_of_a_count_below_a_thousand_writes_it_whole() {
    // Arrange & Act
    let (zero, most) = (written(0), written(999));

    // Assert
    assert_eq!((zero.as_str(), most.as_str()), ("0", "999"));
}

#[test]
fn fmt_of_a_count_of_millions_separates_every_group_of_three() {
    // Arrange & Act
    let millions = written(1_234_567);

    // Assert
    assert_eq!(millions, "1,234,567");
}

// Six digits fill two groups exactly: no separator leads them.
#[test]
fn fmt_of_a_count_whose_digits_fill_whole_groups_writes_no_leading_separator() {
    // Arrange & Act
    let (thousand, six_digits) = (written(1_000), written(123_456));

    // Assert
    assert_eq!(
        (thousand.as_str(), six_digits.as_str()),
        ("1,000", "123,456")
    );
}

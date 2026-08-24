// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::analysis_config::AnalysisConfig;
use dry4rust::config::Config;

#[test]
fn analysis_config_carries_only_the_two_floors_a_parser_needs() {
    // Arrange
    let config = Config {
        min_nodes: 7,
        min_lines: 3,
        sub_function: true,
        exclude_tests: true,
        ..Config::default()
    };

    // Act
    let analysis = config.analysis_config();

    // Assert
    assert_eq!(
        analysis,
        AnalysisConfig {
            min_nodes: 7,
            min_lines: 3
        },
        "a backend turns source into code units; nothing else in Config is its \
         business"
    );
}

#[test]
fn eq_distinguishes_two_configs_that_differ_in_either_floor() {
    // Arrange
    let base = AnalysisConfig {
        min_nodes: 10,
        min_lines: 0,
    };

    // Act & Assert
    assert_eq!(
        base,
        AnalysisConfig {
            min_nodes: 10,
            min_lines: 0
        }
    );
    assert_ne!(
        base,
        AnalysisConfig {
            min_nodes: 11,
            min_lines: 0
        }
    );
    assert_ne!(
        base,
        AnalysisConfig {
            min_nodes: 10,
            min_lines: 1
        }
    );
}

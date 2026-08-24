// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::analysis_config::AnalysisConfig;
use dry4rust::analyzer::LanguageAnalyzer;
use dry4rust::rust::rust_analyzer::RustAnalyzer;
use std::path::PathBuf;

#[test]
fn rust_analyzer_through_trait() {
    // Arrange & Act
    let analyzer = RustAnalyzer::new();
    let config = AnalysisConfig {
        min_nodes: 1,
        min_lines: 0,
    };
    let source = r#"
        fn foo(x: i32) -> i32 {
            let y = x + 1;
            y * 2
        }
        #[test]
        fn test_foo() {
            let z = 1;
            let w = z + 1;
            assert_eq!(w, 2);
        }
    "#;
    let path = PathBuf::from("test.rs");
    let units = analyzer.parse_file(&path, source, &config).unwrap();

    // Both units should be present (filtering is done by analyze())

    // Assert
    assert!(units.len() >= 2);

    // Production code should not be tagged as test
    let prod: Vec<_> = units.iter().filter(|u| u.name == "foo").collect();
    assert_eq!(prod.len(), 1);
    assert!(!prod[0].is_test);

    // Test code should be tagged
    let test: Vec<_> = units.iter().filter(|u| u.name == "test_foo").collect();
    assert_eq!(test.len(), 1);
    assert!(test[0].is_test);

    // Default is_test_code() delegates to is_test field
    assert!(!analyzer.is_test_code(prod[0]));
}

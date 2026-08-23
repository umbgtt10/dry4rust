// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::rust::parser::*;
use std::fs;
use std::path::Path;
use syn::parse_str;
use tempfile::TempDir;

fn write_and_parse(code: &str, min_nodes: usize) -> Vec<CodeUnit> {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.rs");
    fs::write(&file, code).unwrap();
    parse_file(&file, min_nodes, 0).unwrap()
}

#[test]
fn cfg_test_impl_blocks_tagged_as_test() {
    let code = r#"
        struct Foo;

        impl Foo {
            fn production(&self) -> i32 {
                let x = 42;
                x + 1
            }
        }

        #[cfg(test)]
        impl Foo {
            fn test_helper(&self) -> i32 {
                let x = 42;
                x + 1
            }
        }
    "#;

    let units = write_and_parse(code, 1);
    let prod: Vec<_> = units
        .iter()
        .filter(|u| u.name.contains("production"))
        .collect();
    let helper: Vec<_> = units
        .iter()
        .filter(|u| u.name.contains("test_helper"))
        .collect();

    assert_eq!(prod.len(), 1);
    assert!(!prod[0].is_test);
    assert_eq!(helper.len(), 1);
    assert!(helper[0].is_test);
}

#[test]
fn cfg_test_module_functions_tagged_as_test() {
    let code = r#"
        fn production(x: i32) -> i32 {
            let y = x + 1;
            y * 2
        }

        #[cfg(test)]
        mod tests {
            fn helper(x: i32) -> i32 {
                let y = x + 1;
                y * 2
            }
        }
    "#;

    let units = write_and_parse(code, 1);
    let prod: Vec<_> = units.iter().filter(|u| u.name == "production").collect();
    let helper: Vec<_> = units.iter().filter(|u| u.name == "helper").collect();

    assert_eq!(prod.len(), 1);
    assert!(!prod[0].is_test);
    assert_eq!(helper.len(), 1);
    assert!(helper[0].is_test);
}

#[test]
fn code_unit_has_line_numbers() {
    let units = write_and_parse(
        r#"
fn first() {
let x = 1;
}

fn second() {
let y = 2;
}
        "#,
        1,
    );
    assert!(units.len() >= 2);
    // First function starts at line 2
    assert!(units[0].line_start > 0);
    assert!(units[0].line_end >= units[0].line_start);
}

#[test]
fn code_unit_kind_display() {
    assert_eq!(CodeUnitKind::Function.to_string(), "function");
    assert_eq!(CodeUnitKind::Method.to_string(), "method");
    assert_eq!(CodeUnitKind::Closure.to_string(), "closure");
}

#[test]
fn different_functions_different_fingerprint() {
    let units = write_and_parse(
        r#"
        fn add(x: i32) -> i32 {
            x + 1
        }
        fn mul(x: i32) -> i32 {
            x * 2
        }
        "#,
        1,
    );
    let fns: Vec<_> = units
        .iter()
        .filter(|u| u.kind == CodeUnitKind::Function)
        .collect();
    assert_eq!(fns.len(), 2);
    assert_ne!(fns[0].fingerprint, fns[1].fingerprint);
}

#[test]
fn duplicate_functions_same_fingerprint() {
    let units = write_and_parse(
        r#"
        fn foo(x: i32) -> i32 {
            let y = x + 1;
            y * 2
        }
        fn bar(a: i32) -> i32 {
            let b = a + 1;
            b * 2
        }
        "#,
        1,
    );
    let fns: Vec<_> = units
        .iter()
        .filter(|u| u.kind == CodeUnitKind::Function)
        .collect();
    assert_eq!(fns.len(), 2);
    assert_eq!(fns[0].fingerprint, fns[1].fingerprint);
}

#[test]
fn extracts_closures() {
    let units = write_and_parse(
        r#"
        fn foo() {
            let f = |x: i32, y: i32| {
                let sum = x + y;
                let product = x * y;
                sum + product
            };
        }
        "#,
        1,
    );
    let closures: Vec<_> = units
        .iter()
        .filter(|u| u.kind == CodeUnitKind::Closure)
        .collect();
    assert!(!closures.is_empty());
}

#[test]
fn extracts_methods_from_impl() {
    let units = write_and_parse(
        r#"
        struct Foo;
        impl Foo {
            fn bar(&self) -> i32 {
                42
            }
            fn baz(&mut self, val: i32) {
                let _ = val + 1;
            }
        }
        "#,
        1,
    );
    let methods: Vec<_> = units
        .iter()
        .filter(|u| u.kind == CodeUnitKind::Method)
        .collect();
    assert_eq!(methods.len(), 2);
    assert!(methods[0].name.contains("Foo::bar"));
    assert!(methods[1].name.contains("Foo::baz"));
}

#[test]
fn extracts_top_level_functions() {
    let units = write_and_parse(
        r#"
        fn foo(x: i32) -> i32 {
            let y = x + 1;
            y * 2
        }
        fn bar() {
            println!("hello");
        }
        "#,
        1,
    );
    let fns: Vec<_> = units
        .iter()
        .filter(|u| u.kind == CodeUnitKind::Function)
        .collect();
    assert_eq!(fns.len(), 2);
    assert_eq!(fns[0].name, "foo");
    assert_eq!(fns[1].name, "bar");
}

#[test]
fn extracts_trait_impl_methods() {
    let units = write_and_parse(
        r#"
        struct Foo;
        trait MyTrait {
            fn do_thing(&self) -> i32;
        }
        impl MyTrait for Foo {
            fn do_thing(&self) -> i32 {
                let x = 42;
                x + 1
            }
        }
        "#,
        1,
    );
    let trait_impls: Vec<_> = units
        .iter()
        .filter(|u| u.kind == CodeUnitKind::TraitImplBlock)
        .collect();
    assert_eq!(trait_impls.len(), 1);
    assert!(trait_impls[0].name.contains("Foo"));
    assert!(trait_impls[0].name.contains("MyTrait"));
    assert!(trait_impls[0].name.contains("do_thing"));
}

#[test]
fn handles_parse_errors_gracefully() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("broken.rs");
    fs::write(&file, "fn broken( { }").unwrap();
    let result = parse_file(&file, 1, 0);
    assert!(result.is_err());
}

#[test]
fn min_line_count_filters_short_functions() {
    let code = r#"
fn short(x: i32) -> i32 {
x + 1
}

fn longer(x: i32) -> i32 {
let a = x + 1;
let b = a * 2;
let c = b - 3;
let d = c + 4;
a + b + c + d
}
    "#;
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.rs");
    fs::write(&file, code).unwrap();

    // With min_line_count=0, both functions should appear
    let units_all = parse_file(&file, 1, 0).unwrap();
    assert!(units_all.len() >= 2);

    // With min_line_count=5, only the longer function should pass
    let units_filtered = parse_file(&file, 1, 5).unwrap();
    assert!(units_filtered.len() < units_all.len());
    for unit in &units_filtered {
        let lines = unit.line_end.saturating_sub(unit.line_start) + 1;
        assert!(lines >= 5, "unit {} has only {lines} lines", unit.name);
    }
}

#[test]
fn non_test_code_not_tagged() {
    let code = r#"
        fn production(x: i32) -> i32 {
            let y = x + 1;
            y * 2
        }
        #[test]
        fn my_test() {
            let x = 1;
            let y = x + 1;
            assert_eq!(y, 2);
        }
    "#;

    let units = write_and_parse(code, 1);
    let non_test: Vec<_> = units.iter().filter(|u| !u.is_test).collect();
    assert!(!non_test.is_empty());
    assert!(non_test.iter().all(|u| u.name != "my_test"));
}

#[test]
fn parse_files_collects_warnings() {
    let tmp = TempDir::new().unwrap();
    let good = tmp.path().join("good.rs");
    let bad = tmp.path().join("bad.rs");
    fs::write(&good, "fn good() { let x = 1; }").unwrap();
    fs::write(&bad, "fn bad( {").unwrap();
    let (units, warnings) = parse_files(&[good, bad], 1, 0);
    assert!(!units.is_empty());
    assert_eq!(warnings.len(), 1);
}

#[test]
fn parse_source_works() {
    let path = Path::new("test.rs");
    let source = "fn foo(x: i32) -> i32 { x + 1 }";
    let units = parse_source(path, source, 1, 0).unwrap();
    assert_eq!(units.len(), 1);
    assert_eq!(units[0].name, "foo");
}

#[test]
fn respects_min_node_count() {
    let units_low = write_and_parse(
        r#"
        fn tiny() -> i32 { 1 }
        fn bigger(x: i32) -> i32 {
            let a = x + 1;
            let b = a * 2;
            a + b
        }
        "#,
        1,
    );
    let units_high = write_and_parse(
        r#"
        fn tiny() -> i32 { 1 }
        fn bigger(x: i32) -> i32 {
            let a = x + 1;
            let b = a * 2;
            a + b
        }
        "#,
        20,
    );
    assert!(units_low.len() >= units_high.len());
}

#[test]
fn test_functions_tagged_as_test() {
    let code = r#"
        fn production(x: i32) -> i32 {
            let y = x + 1;
            y * 2
        }
        #[test]
        fn my_test() {
            let x = 1;
            let y = x + 1;
            assert_eq!(y, 2);
        }
    "#;

    let units = write_and_parse(code, 1);
    let prod: Vec<_> = units.iter().filter(|u| u.name == "production").collect();
    let test: Vec<_> = units.iter().filter(|u| u.name == "my_test").collect();

    assert_eq!(prod.len(), 1);
    assert!(!prod[0].is_test);
    assert_eq!(test.len(), 1);
    assert!(test[0].is_test);
}

#[test]
fn test_has_cfg_test_attr() {
    let file: syn::File = parse_str(
        r#"
        #[cfg(test)]
        mod tests {}
        mod normal {}
        "#,
    )
    .unwrap();

    let items = &file.items;
    if let syn::Item::Mod(m) = &items[0] {
        assert!(has_cfg_test_attr(&m.attrs));
    } else {
        panic!("expected module");
    }
    if let syn::Item::Mod(m) = &items[1] {
        assert!(!has_cfg_test_attr(&m.attrs));
    } else {
        panic!("expected module");
    }
}

#[test]
fn test_has_test_attr() {
    let file: syn::File = parse_str(
        r#"
        #[test]
        fn my_test() {}
        fn normal() {}
        "#,
    )
    .unwrap();

    let items = &file.items;
    if let syn::Item::Fn(f) = &items[0] {
        assert!(has_test_attr(&f.attrs));
    } else {
        panic!("expected function");
    }
    if let syn::Item::Fn(f) = &items[1] {
        assert!(!has_test_attr(&f.attrs));
    } else {
        panic!("expected function");
    }
}

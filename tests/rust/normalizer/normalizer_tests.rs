// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::rust::normalizer::*;
use syn::parse_str;

fn normalize_code_expr(code: &str) -> NormalizedNode {
    let expr = parse_expr(code);
    let mut ctx = NormalizationContext::new();
    normalize_expr(&expr, &mut ctx)
}

fn parse_expr(code: &str) -> syn::Expr {
    parse_str::<syn::Expr>(code).unwrap()
}

fn parse_fn(code: &str) -> syn::ItemFn {
    parse_str::<syn::ItemFn>(code).unwrap()
}

#[test]
fn array_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("[1, 2, 3]");

    // Assert
    assert_eq!(n.kind, NodeKind::Array);
    assert_eq!(n.children.len(), 3);
}

#[test]
fn assign_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("x = 5");

    // Assert
    assert_eq!(n.kind, NodeKind::Assign);
}

#[test]
fn await_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("fut.await");

    // Assert
    assert_eq!(n.kind, NodeKind::Await);
}

#[test]
fn binary_ops_preserved() {
    // Arrange & Act
    let n1 = normalize_code_expr("a + b");
    let n2 = normalize_code_expr("a - b");

    // Assert
    assert_ne!(n1, n2);
}

#[test]
fn bool_literals_normalize_as_placeholders() {
    // Arrange & Act
    let n1 = normalize_code_expr("true");
    let n2 = normalize_code_expr("false");

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn cast_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("x as f64");

    // Assert
    assert_eq!(n.kind, NodeKind::Cast);
}

#[test]
fn closures_normalized() {
    // Arrange & Act
    let code1 = "|x| x + 1";
    let code2 = "|y| y + 1";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn complex_function_normalization() {
    // Arrange & Act
    let code1 = r#"
        fn process(data: Vec<i32>) -> Result<i32, String> {
            let mut sum = 0;
            for item in data.iter() {
                if *item > 0 {
                    sum += *item;
                }
            }
            Ok(sum)
        }
    "#;
    let code2 = r#"
        fn compute(values: Vec<i32>) -> Result<i32, String> {
            let mut total = 0;
            for val in values.iter() {
                if *val > 0 {
                    total += *val;
                }
            }
            Ok(total)
        }
    "#;
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let (sig1, body1) = normalize_item_fn(&f1);
    let (sig2, body2) = normalize_item_fn(&f2);

    // Assert
    assert_eq!(sig1, sig2);
    assert_eq!(body1, body2);
}

#[test]
fn different_macro_names_produce_different_nodes() {
    // Arrange & Act
    let n1 = normalize_code_expr("println!(\"hello\")");
    let n2 = normalize_code_expr("eprintln!(\"hello\")");

    // Assert
    assert_ne!(n1, n2);
}

#[test]
fn empty_block_normalized() {
    // Arrange & Act
    let code = "fn foo() {}";
    let f = parse_fn(code);
    let (_, body) = normalize_item_fn(&f);

    // Assert
    assert_eq!(body.kind, NodeKind::Block);
    assert!(body.children.is_empty());
}

#[test]
fn field_access_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("foo.bar");

    // Assert
    assert_eq!(n.kind, NodeKind::FieldAccess);
}

#[test]
fn for_loops_normalized() {
    // Arrange & Act
    let code1 = "for i in 0..10 { println!(\"hello\") }";
    let code2 = "for j in 0..10 { println!(\"world\") }";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn if_else_structure_preserved() {
    // Arrange & Act
    let code1 = "if x > 0 { x } else { -x }";
    let code2 = "if a > 0 { a } else { -a }";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn if_vs_if_else_different() {
    // Arrange & Act
    let code1 = "if x > 0 { x }";
    let code2 = "if x > 0 { x } else { 0 }";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);

    // Assert
    assert_ne!(n1, n2);
}

#[test]
fn impl_block_methods_normalized() {
    // Arrange & Act
    let code = r#"
        impl Foo {
            fn bar(&self) -> i32 { self.x + 1 }
            fn baz(&mut self, val: i32) { self.x = val; }
        }
    "#;
    let item: syn::ItemImpl = parse_str(code).unwrap();
    let methods = normalize_impl_block(&item);

    // Assert
    assert_eq!(methods.len(), 2);
    assert_eq!(methods[0].0, "bar");
    assert_eq!(methods[1].0, "baz");
}

#[test]
fn index_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("arr[0]");

    // Assert
    assert_eq!(n.kind, NodeKind::Index);
}

#[test]
fn literal_kind_preserved_but_value_erased() {
    // Arrange & Act
    let n1 = normalize_code_expr("42");
    let n2 = normalize_code_expr("99");
    let n3 = normalize_code_expr("3.14");

    // Assert
    assert_eq!(n1, n2); // both are Int
    assert_ne!(n1, n3); // Int vs Float
}

#[test]
fn loop_normalized() {
    // Arrange & Act
    let code = "loop { break; }";
    let n = normalize_code_expr(code);

    // Assert
    assert_eq!(n.kind, NodeKind::Loop);
}

#[test]
fn macro_call_node_count() {
    // Arrange & Act
    let n = normalize_code_expr("println!(\"a\", \"b\")");
    // 1 for MacroCall + 2 args (Literal each = 1)

    // Assert
    assert_eq!(count_nodes(&n), 3);
}

#[test]
fn macro_invocations_produce_macro_call() {
    // Arrange & Act
    let n = normalize_code_expr("println!(\"hello\")");

    // Assert
    match &n.kind {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "println");
            assert_eq!(n.children.len(), 1);
            assert_eq!(
                n.children[0],
                NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Str))
            );
        }
        _ => panic!("Expected MacroCall node, got {:?}", n),
    }
}

#[test]
fn match_arms_normalized() {
    // Arrange & Act
    let code = r#"match x { 0 => "zero", _ => "other" }"#;
    let n = normalize_code_expr(code);
    // Match -> [expr, arm0, arm1]

    // Assert
    assert_eq!(n.kind, NodeKind::Match);
    // children[0] is expr, children[1..] are arms
    assert_eq!(n.children.len(), 3); // expr + 2 arms
}

#[test]
fn method_calls_normalized() {
    // Arrange & Act
    let code1 = "x.foo(y)";
    let code2 = "a.foo(b)";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn multi_segment_macro_path_uses_last_segment() {
    // Arrange & Act
    let n = normalize_code_expr("std::println!(\"hello\")");

    // Assert
    match &n.kind {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "println");
        }
        _ => panic!("Expected MacroCall node, got {:?}", n),
    }
}

#[test]
fn node_counting_works() {
    // Arrange & Act
    let code = "fn foo(x: i32) -> i32 { x + 1 }";
    let f = parse_fn(code);
    let (sig, body) = normalize_item_fn(&f);
    let sig_count = count_nodes(&sig);
    let body_count = count_nodes(&body);

    // Assert
    assert!(sig_count > 0);
    assert!(body_count > 0);
}

#[test]
fn pat_macro_normalized() {
    // Arrange & Act
    let code = "fn foo(x: i32) { match x { my_pat!(x) => {} _ => {} } }";

    // Assert
    if let Ok(f) = parse_str::<syn::ItemFn>(code) {
        let (_, body) = normalize_item_fn(&f);

        assert!(count_nodes(&body) > 0);
    }
}

#[test]
fn range_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("0..10");
    // Range -> [from_or_None, to_or_None]

    // Assert
    assert_eq!(n.kind, NodeKind::Range);
    assert_eq!(n.children.len(), 2);
    assert!(!n.children[0].is_none());
    assert!(!n.children[1].is_none());
}

#[test]
fn reference_expressions_normalized() {
    // Arrange & Act
    let n1 = normalize_code_expr("&x");
    let n2 = normalize_code_expr("&mut x");

    // Assert
    assert_ne!(n1, n2); // mutability matters
}

#[test]
fn reindex_from_real_function_subtrees() {
    // Arrange & Act
    let f1 = parse_fn("fn foo(x: i32, y: i32) -> i32 { if x > 0 { let z = y + 1; z } else { x } }");
    let f2 = parse_fn(
        "fn bar(unused: i32, a: i32, b: i32) -> i32 { if a > 0 { let c = b + 1; c } else { a } }",
    );
    let (_, body1) = normalize_item_fn(&f1);
    let (_, body2) = normalize_item_fn(&f2);

    // Extract the then_branch from each: Block -> stmts[0] -> If -> children[1]
    let then1 = match &body1.kind {
        NodeKind::Block => match &body1.children[0].kind {
            NodeKind::If => body1.children[0].children[1].clone(),
            _ => panic!("expected If"),
        },
        _ => panic!("expected Block"),
    };
    let then2 = match &body2.kind {
        NodeKind::Block => match &body2.children[0].kind {
            NodeKind::If => body2.children[0].children[1].clone(),
            _ => panic!("expected If"),
        },
        _ => panic!("expected Block"),
    };

    // Assert
    assert_ne!(then1, then2);
    assert_eq!(reindex_placeholders(&then1), reindex_placeholders(&then2));
}

#[test]
fn renamed_variables_produce_identical_trees() {
    // Arrange & Act
    let code1 = "fn foo(x: i32) -> i32 { let y = x + 1; y }";
    let code2 = "fn bar(a: i32) -> i32 { let b = a + 1; b }";
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let (sig1, body1) = normalize_item_fn(&f1);
    let (sig2, body2) = normalize_item_fn(&f2);

    // Assert
    assert_eq!(sig1, sig2);
    assert_eq!(body1, body2);
}

#[test]
fn return_expression_normalized() {
    // Arrange & Act
    let n1 = normalize_code_expr("return 42");
    let n2 = normalize_code_expr("return 99");

    // Assert
    assert_eq!(n1, n2); // both return Int literals
}

#[test]
fn same_macro_different_arg_count_are_different() {
    // Arrange & Act
    let n1 = normalize_code_expr("println!(\"a\")");
    let n2 = normalize_code_expr("println!(\"a\", \"b\")");

    // Assert
    assert_ne!(n1, n2);
}

#[test]
fn same_macro_different_literal_values_are_equal() {
    // Arrange & Act
    let n1 = normalize_code_expr("println!(\"hello\")");
    let n2 = normalize_code_expr("println!(\"world\")");

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn string_literals_are_equal() {
    // Arrange & Act
    let n1 = normalize_code_expr("\"hello\"");
    let n2 = normalize_code_expr("\"world\"");

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn struct_init_normalized() {
    // Arrange & Act
    let code1 = "Foo { x: 1, y: 2 }";
    let code2 = "Bar { a: 1, b: 2 }";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);
    // Both have StructInit; children[0] is rest_or_None, rest are fields

    // Assert
    assert_eq!(n1.kind, NodeKind::StructInit);
    assert_eq!(n2.kind, NodeKind::StructInit);
    // Same number of fields
    assert_eq!(n1.children.len(), n2.children.len());
}

#[test]
fn structural_changes_produce_different_trees() {
    // Arrange & Act
    let code1 = "fn foo(x: i32) -> i32 { x + 1 }";
    let code2 = "fn foo(x: i32) -> i32 { x * 1 }";
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let (_, body1) = normalize_item_fn(&f1);
    let (_, body2) = normalize_item_fn(&f2);

    // Assert
    assert_ne!(body1, body2);
}

#[test]
fn try_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("result?");

    // Assert
    assert_eq!(n.kind, NodeKind::Try);
}

#[test]
fn tuple_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("(1, 2, 3)");

    // Assert
    assert_eq!(n.kind, NodeKind::Tuple);
    assert_eq!(n.children.len(), 3);
}

#[test]
fn tuple_pattern_normalized() {
    // Arrange & Act
    let code1 = "fn foo() { let (a, b) = (1, 2); }";
    let code2 = "fn bar() { let (x, y) = (1, 2); }";
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let (_, body1) = normalize_item_fn(&f1);
    let (_, body2) = normalize_item_fn(&f2);

    // Assert
    assert_eq!(body1, body2);
}

#[test]
fn type_position_macro_normalized() {
    // Arrange & Act
    let code = "fn foo() -> my_type!(i32) {}";

    // Assert
    if let Ok(f) = parse_str::<syn::ItemFn>(code) {
        let (sig, _) = normalize_item_fn(&f);

        assert!(count_nodes(&sig) > 0);
    }
}

#[test]
fn unary_ops_preserved() {
    // Arrange & Act
    let n1 = normalize_code_expr("!x");
    let n2 = normalize_code_expr("-x");

    // Assert
    assert_ne!(n1, n2);
}

#[test]
fn unparseable_macro_args_produce_opaque() {
    // Arrange & Act
    let n = normalize_code_expr("vec![x; 10]");

    // Assert
    match &n.kind {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "vec");
            assert_eq!(n.children.len(), 1);
            assert_eq!(n.children[0], NormalizedNode::leaf(NodeKind::Opaque));
        }
        _ => panic!("Expected MacroCall node, got {:?}", n),
    }
}

#[test]
fn unparseable_macro_differs_from_no_args() {
    // Arrange & Act
    let n_empty = normalize_code_expr("my_macro!()");
    let n_unparseable = normalize_code_expr("vec![x; 10]");

    // Assert
    match (&n_empty.kind, &n_unparseable.kind) {
        (NodeKind::MacroCall { .. }, NodeKind::MacroCall { .. }) => {
            assert!(n_empty.children.is_empty());
            assert_eq!(n_unparseable.children.len(), 1);
            assert_eq!(
                n_unparseable.children[0],
                NormalizedNode::leaf(NodeKind::Opaque)
            );
        }
        _ => panic!("Expected MacroCall nodes"),
    }
}

#[test]
fn vec_macro_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("vec![1, 2, 3]");

    // Assert
    match &n.kind {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "vec");
            assert_eq!(n.children.len(), 3);
        }
        _ => panic!("Expected MacroCall node, got {:?}", n),
    }
}

#[test]
fn while_loop_normalized() {
    // Arrange & Act
    let code1 = "while x > 0 { x = x - 1; }";
    let code2 = "while a > 0 { a = a - 1; }";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);

    // Assert
    assert_eq!(n1, n2);
}

// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use dry4rust::node::LiteralKind;
use dry4rust::node::NodeKind;
use dry4rust::node::NormalizedNode;
use dry4rust::node::PlaceholderKind;
use dry4rust::rust::normalizer::normalization_context::NormalizationContext;
use syn::BinOp;
use syn::Block;
use syn::Expr;
use syn::ExprClosure;
use syn::ItemFn;
use syn::ItemImpl;
use syn::Lit;
use syn::Macro;
use syn::Member;
use syn::Pat;
use syn::Stmt;
use syn::Type;
use syn::UnOp;
use syn::parse::Parser;
use syn::parse_str;

fn normalize_code_expr(code: &str) -> NormalizedNode {
    let expr = parse_expr(code);
    let mut ctx = NormalizationContext::new();
    ctx.normalize_expr(&expr)
}

fn parse_expr(code: &str) -> Expr {
    parse_str::<Expr>(code).unwrap()
}

fn parse_fn(code: &str) -> ItemFn {
    parse_str::<ItemFn>(code).unwrap()
}

fn pat_of(source: &str) -> String {
    let pat = Pat::parse_single
        .parse_str(source)
        .expect("the pattern parses");
    let mut ctx = NormalizationContext::new();
    format!("{:?}", ctx.normalize_pat(&pat))
}

fn type_of(source: &str) -> String {
    let ty: Type = parse_str(source).expect("the type parses");
    let mut ctx = NormalizationContext::new();
    format!("{:?}", ctx.normalize_type(&ty))
}

#[test]
fn array_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("[1, 2, 3]");

    // Assert
    assert_eq!(*n.kind(), NodeKind::Array);
    assert_eq!(n.children().len(), 3);
}

#[test]
fn assign_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("x = 5");

    // Assert
    assert_eq!(*n.kind(), NodeKind::Assign);
}

#[test]
fn await_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("fut.await");

    // Assert
    assert_eq!(*n.kind(), NodeKind::Await);
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
    assert_eq!(*n.kind(), NodeKind::Cast);
}

#[test]
fn complex_function_normalization() {
    // Arrange & Act
    let code1 = r"
        fn process(data: Vec<i32>) -> Result<i32, String> {
            let mut sum = 0;
            for item in data.iter() {
                if *item > 0 {
                    sum += *item;
                }
            }
            Ok(sum)
        }
    ";
    let code2 = r"
        fn compute(values: Vec<i32>) -> Result<i32, String> {
            let mut total = 0;
            for val in values.iter() {
                if *val > 0 {
                    total += *val;
                }
            }
            Ok(total)
        }
    ";
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let normalized_body1 = NormalizationContext::normalize_item_fn(&f1);
    let sig1 = normalized_body1.signature;
    let body1 = normalized_body1.body;
    let normalized_body2 = NormalizationContext::normalize_item_fn(&f2);
    let sig2 = normalized_body2.signature;
    let body2 = normalized_body2.body;

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
    let body = NormalizationContext::normalize_item_fn(&f).body;

    // Assert
    assert_eq!(*body.kind(), NodeKind::Block);
    assert!(body.children().is_empty());
}

#[test]
fn field_access_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("foo.bar");

    // Assert
    assert_eq!(*n.kind(), NodeKind::FieldAccess);
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
    let code = r"
        impl Foo {
            fn bar(&self) -> i32 { self.x + 1 }
            fn baz(&mut self, val: i32) { self.x = val; }
        }
    ";
    let item: ItemImpl = parse_str(code).unwrap();
    let methods = NormalizationContext::normalize_impl_block(&item);

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
    assert_eq!(*n.kind(), NodeKind::Index);
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
fn macro_call_node_count() {
    // Arrange & Act
    let n = normalize_code_expr("println!(\"a\", \"b\")");
    // 1 for MacroCall + 2 args (Literal each = 1)

    // Assert
    assert_eq!(NormalizedNode::count_nodes(&n), 3);
}

#[test]
fn macro_invocations_produce_macro_call() {
    // Arrange & Act
    let n = normalize_code_expr("println!(\"hello\")");

    // Assert
    match &n.kind() {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "println");
            assert_eq!(n.children().len(), 1);
            assert_eq!(
                n.children()[0],
                NormalizedNode::leaf(NodeKind::Literal(LiteralKind::Str))
            );
        }
        _ => panic!("Expected MacroCall node, got {n:?}"),
    }
}

#[test]
fn match_arms_normalized() {
    // Arrange & Act
    let code = r#"match x { 0 => "zero", _ => "other" }"#;
    let n = normalize_code_expr(code);
    // Match -> [expr, arm0, arm1]

    // Assert
    assert_eq!(*n.kind(), NodeKind::Match);
    // children[0] is expr, children[1..] are arms
    assert_eq!(n.children().len(), 3); // expr + 2 arms
}

#[test]
fn member_to_string_for_a_named_field_returns_the_name() {
    // Arrange
    let member: Member = parse_str("field").expect("parses");

    // Act & Assert
    assert_eq!(NormalizationContext::member_to_string(&member), "field");
}

#[test]
fn member_to_string_for_a_tuple_index_returns_the_index() {
    // Arrange
    let member: Member = parse_str("0").expect("parses");

    // Act & Assert
    assert_eq!(NormalizationContext::member_to_string(&member), "0");
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
    match &n.kind() {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "println");
        }
        _ => panic!("Expected MacroCall node, got {n:?}"),
    }
}

#[test]
fn node_counting_works() {
    // Arrange & Act
    let code = "fn foo(x: i32) -> i32 { x + 1 }";
    let f = parse_fn(code);
    let normalized_body = NormalizationContext::normalize_item_fn(&f);
    let sig = normalized_body.signature;
    let body = normalized_body.body;
    let sig_count = NormalizedNode::count_nodes(&sig);
    let body_count = NormalizedNode::count_nodes(&body);

    // Assert
    assert!(sig_count > 0);
    assert!(body_count > 0);
}

#[test]
fn normalize_bin_op_maps_different_operators_to_different_kinds() {
    // Arrange
    let add: BinOp = parse_str("+").expect("parses");
    let sub: BinOp = parse_str("-").expect("parses");

    // Act & Assert
    assert_ne!(
        NormalizationContext::normalize_bin_op(&add),
        NormalizationContext::normalize_bin_op(&sub)
    );
    assert_eq!(
        NormalizationContext::normalize_bin_op(&add),
        NormalizationContext::normalize_bin_op(&add)
    );
}

#[test]
fn normalize_block_erases_the_names_its_statements_bind() {
    // Arrange
    let a: Block = parse_str("{ let x = 1; x + 1 }").expect("parses");
    let b: Block = parse_str("{ let y = 1; y + 1 }").expect("parses");

    // Act
    let na = NormalizationContext::new().normalize_block(&a);
    let nb = NormalizationContext::new().normalize_block(&b);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_block_keeps_blocks_of_different_shape_apart() {
    // Arrange
    let a: Block = parse_str("{ let x = 1; x + 1 }").expect("parses");
    let b: Block = parse_str("{ let x = 1; }").expect("parses");

    // Act
    let na = NormalizationContext::new().normalize_block(&a);
    let nb = NormalizationContext::new().normalize_block(&b);

    // Assert
    assert_ne!(na, nb);
}

#[test]
fn normalize_closure_expr_ignores_the_parameter_names() {
    // Arrange
    let a: ExprClosure = parse_str("|x: i32| x + 1").expect("parses");
    let b: ExprClosure = parse_str("|y: i32| y + 1").expect("parses");

    // Act
    let na = NormalizationContext::normalize_closure_expr(&a);
    let nb = NormalizationContext::normalize_closure_expr(&b);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_closures_ignores_the_parameter_names() {
    // Arrange & Act
    let code1 = "|x| x + 1";
    let code2 = "|y| y + 1";
    let n1 = normalize_code_expr(code1);
    let n2 = normalize_code_expr(code2);

    // Assert
    assert_eq!(n1, n2);
}

#[test]
fn normalize_impl_item_fn_splits_a_method_into_signature_and_body() {
    // Arrange
    let item: ItemImpl =
        parse_str("impl Foo { fn bar(&self, x: i32) -> i32 { x + 1 } }").expect("parses");
    let syn::ImplItem::Fn(method) = item.items.first().expect("one method").clone() else {
        panic!("expected a method");
    };

    // Act
    let normalized_body = NormalizationContext::normalize_impl_item_fn(&method);
    let sig = normalized_body.signature;
    let body = normalized_body.body;

    // Assert
    assert_ne!(sig, body);
    assert_ne!(*sig.kind(), NodeKind::None);
}

#[test]
fn normalize_lit_erases_the_value_but_keeps_the_type() {
    // Arrange
    let small: Lit = parse_str("42").expect("parses");
    let large: Lit = parse_str("99999").expect("parses");
    let text: Lit = parse_str(r#""hello""#).expect("parses");

    // Act & Assert
    assert_eq!(
        NormalizationContext::normalize_lit(&small),
        NormalizationContext::normalize_lit(&large)
    );
    assert_ne!(
        NormalizationContext::normalize_lit(&small),
        NormalizationContext::normalize_lit(&text)
    );
}

#[test]
fn normalize_loop_produces_a_loop_node() {
    // Arrange & Act
    let code = "loop { break; }";
    let n = normalize_code_expr(code);

    // Assert
    assert_eq!(*n.kind(), NodeKind::Loop);
}

#[test]
fn normalize_macro_gives_the_same_macro_the_same_node() {
    // Arrange
    let a: Macro = parse_str(r#"println!("one")"#).expect("parses");
    let b: Macro = parse_str(r#"println!("two")"#).expect("parses");

    // Act
    let na = NormalizationContext::new().normalize_macro(&a);
    let nb = NormalizationContext::new().normalize_macro(&b);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_pat_gives_a_tuple_struct_the_same_kind_as_a_braced_struct() {
    // Arrange & Act
    let tuple_struct = pat_of("Some(inner)");
    let braced = pat_of("Point { x }");

    // Assert
    assert!(tuple_struct.contains("PatStruct"), "got {tuple_struct}");
    assert!(braced.contains("PatStruct"), "got {braced}");
    assert!(
        !tuple_struct.contains("FieldValue"),
        "a tuple struct destructures by position, so its elements are bare"
    );
    assert!(
        braced.contains("FieldValue"),
        "a braced struct destructures by name, so each field carries the name          alongside the binding -- which is why the two are not interchangeable          despite sharing a kind"
    );
}

#[test]
fn normalize_pat_gives_differently_named_bindings_the_same_node() {
    // Arrange
    let a: Pat = Pat::parse_single.parse_str("x").expect("parses");
    let b: Pat = Pat::parse_single.parse_str("y").expect("parses");

    // Act
    let na = NormalizationContext::new().normalize_pat(&a);
    let nb = NormalizationContext::new().normalize_pat(&b);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_pat_keeps_a_literal_pattern_apart_from_a_wildcard() {
    // Arrange & Act
    let literal = pat_of("42");
    let wildcard = pat_of("_");

    // Assert
    assert!(literal.contains("PatLiteral"));
    assert!(wildcard.contains("PatWild"));
}

#[test]
fn normalize_pat_over_a_mutable_reference_records_the_mutability() {
    // Arrange & Act
    let shared = pat_of("&value");
    let mutable = pat_of("&mut value");

    // Assert
    assert_ne!(
        shared, mutable,
        "a pattern binding through &mut is not the same shape as one through &"
    );
}

#[test]
fn normalize_pat_over_a_range_pattern_yields_a_range_node() {
    // Arrange & Act
    let node = pat_of("1..=9");

    // Assert
    assert!(node.contains("PatRange"), "got {node}");
}

#[test]
fn normalize_pat_over_a_rest_pattern_yields_a_rest_node() {
    // Arrange & Act
    let node = pat_of("[first, ..]");

    // Assert
    assert!(node.contains("PatRest"), "got {node}");
}

#[test]
fn normalize_pat_over_a_slice_pattern_yields_a_slice_node() {
    // Arrange & Act
    let node = pat_of("[a, b, c]");

    // Assert
    assert!(node.contains("PatSlice"), "got {node}");
}

#[test]
fn normalize_pat_over_a_struct_pattern_yields_a_struct_node() {
    // Arrange & Act
    let node = pat_of("Point { x, y }");

    // Assert
    assert!(node.contains("PatStruct"), "got {node}");
}

#[test]
fn normalize_pat_over_an_alternation_yields_an_or_node() {
    // Arrange
    let pat = Pat::parse_multi
        .parse_str("Red | Green | Blue")
        .expect("an alternation needs parse_multi; parse_single rejects it");
    let mut ctx = NormalizationContext::new();

    // Act
    let node = format!("{:?}", ctx.normalize_pat(&pat));

    // Assert
    assert!(node.contains("PatOr"), "got {node}");
}

#[test]
fn normalize_signature_gives_alpha_equivalent_signatures_the_same_node() {
    // Arrange
    let a: ItemFn = parse_str("fn f(x: i32) -> i32 { x }").expect("parses");
    let b: ItemFn = parse_str("fn g(y: i32) -> i32 { y }").expect("parses");

    // Act
    let na = NormalizationContext::new().normalize_signature(&a.sig);
    let nb = NormalizationContext::new().normalize_signature(&b.sig);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_stmt_gives_alpha_equivalent_lets_the_same_node() {
    // Arrange
    let a: Stmt = parse_str("let x = 1;").expect("parses");
    let b: Stmt = parse_str("let y = 1;").expect("parses");

    // Act
    let na = NormalizationContext::new().normalize_stmt(&a);
    let nb = NormalizationContext::new().normalize_stmt(&b);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_type_gives_distinct_types_distinct_placeholders_in_one_context() {
    // Arrange -- types become positional placeholders, so telling two apart is
    // only meaningful within a single context. In separate contexts the first
    // type of each is deliberately the same node.
    let a: Type = parse_str("i32").expect("parses");
    let b: Type = parse_str("Vec<String>").expect("parses");
    let mut ctx = NormalizationContext::new();

    // Act
    let na = ctx.normalize_type(&a);
    let nb = ctx.normalize_type(&b);

    // Assert
    assert_ne!(na, nb);
}

#[test]
fn normalize_type_gives_the_same_type_the_same_node() {
    // Arrange
    let a: Type = parse_str("i32").expect("parses");
    let b: Type = parse_str("i32").expect("parses");

    // Act
    let mut ctx = NormalizationContext::new();
    let na = ctx.normalize_type(&a);
    let nb = ctx.normalize_type(&b);

    // Assert
    assert_eq!(na, nb);
}

#[test]
fn normalize_type_handles_every_type_form_it_claims_to() {
    // Arrange -- one of each arm the match names, so none of them is reached
    // for the first time by a user
    let forms = [
        "i32",           // Path
        "&str",          // Reference
        "(i32, u8)",     // Tuple
        "[u8]",          // Slice
        "[u8; 4]",       // Array
        "impl Iterator", // ImplTrait
        "_",             // Infer
        "!",             // Never
        "(i32)",         // Paren
        "make_type!()",  // Macro
        "dyn Iterator",  // the fallback arm
    ];

    // Act
    let mut ctx = NormalizationContext::new();
    let nodes: Vec<_> = forms
        .iter()
        .map(|src| {
            let ty: Type = parse_str(src).unwrap_or_else(|e| panic!("{src} parses: {e}"));
            ctx.normalize_type(&ty)
        })
        .collect();

    // Assert
    assert_eq!(nodes.len(), forms.len());
}

#[test]
fn normalize_type_keeps_never_and_infer_apart() {
    // Arrange
    let never: Type = parse_str("!").expect("parses");
    let infer: Type = parse_str("_").expect("parses");

    // Act
    let mut ctx = NormalizationContext::new();
    let a = ctx.normalize_type(&never);
    let b = ctx.normalize_type(&infer);

    // Assert
    assert_ne!(a, b);
}

#[test]
fn normalize_type_over_a_never_type_yields_a_never_node() {
    // Arrange & Act
    let node = type_of("!");

    // Assert
    assert!(node.contains("TypeNever"), "got {node}");
}

#[test]
fn normalize_type_over_a_parenthesised_type_unwraps_the_parentheses() {
    // Arrange & Act
    let bare = type_of("i32");
    let wrapped = type_of("(i32)");

    // Assert
    assert_eq!(
        bare, wrapped,
        "parentheses group the source, not the type itself"
    );
}

#[test]
fn normalize_type_over_a_slice_and_an_array_keeps_them_apart() {
    // Arrange & Act
    let slice = type_of("[u8]");
    let array = type_of("[u8; 4]");

    // Assert
    assert!(slice.contains("TypeSlice"), "got {slice}");
    assert!(array.contains("TypeArray"), "got {array}");
}

#[test]
fn normalize_type_over_an_impl_trait_yields_an_impl_trait_node() {
    // Arrange & Act
    let node = type_of("impl Iterator<Item = u8>");

    // Assert
    assert!(node.contains("TypeImplTrait"), "got {node}");
}

#[test]
fn normalize_type_over_an_inferred_type_yields_an_infer_node() {
    // Arrange & Act
    let node = type_of("_");

    // Assert
    assert!(node.contains("TypeInfer"), "got {node}");
}

#[test]
fn normalize_type_over_the_unit_type_yields_a_unit_node() {
    // Arrange & Act
    let node = type_of("()");

    // Assert
    assert!(node.contains("TypeUnit"), "got {node}");
}

#[test]
fn normalize_type_sees_through_parentheses_to_the_inner_type() {
    // Arrange
    let bare: Type = parse_str("i32").expect("parses");
    let parenthesised: Type = parse_str("(i32)").expect("parses");

    // Act
    let a = NormalizationContext::new().normalize_type(&bare);
    let b = NormalizationContext::new().normalize_type(&parenthesised);

    // Assert
    assert_eq!(a, b);
}

#[test]
fn normalize_un_op_maps_different_operators_to_different_kinds() {
    // Arrange
    let neg: UnOp = parse_str("-").expect("parses");
    let not: UnOp = parse_str("!").expect("parses");

    // Act & Assert
    assert_ne!(
        NormalizationContext::normalize_un_op(&neg),
        NormalizationContext::normalize_un_op(&not)
    );
}

#[test]
fn pat_macro_normalized() {
    // Arrange & Act
    let code = "fn foo(x: i32) { match x { my_pat!(x) => {} _ => {} } }";

    // Assert
    if let Ok(f) = parse_str::<ItemFn>(code) {
        let body = NormalizationContext::normalize_item_fn(&f).body;

        assert!(NormalizedNode::count_nodes(&body) > 0);
    }
}

#[test]
fn placeholder_counts_each_kind_from_zero_independently() {
    // Arrange
    let mut ctx = NormalizationContext::new();

    // Act
    let var = ctx.placeholder("a", PlaceholderKind::Variable);
    let ty = ctx.placeholder("a", PlaceholderKind::Type);

    // Assert
    assert_eq!(var, 0);
    assert_eq!(ty, 0);
}

#[test]
fn placeholder_for_distinct_identifiers_returns_rising_indices() {
    // Arrange
    let mut ctx = NormalizationContext::new();

    // Act
    let first = ctx.placeholder("a", PlaceholderKind::Variable);
    let second = ctx.placeholder("b", PlaceholderKind::Variable);

    // Assert
    assert_eq!(first, 0);
    assert_eq!(second, 1);
}

#[test]
fn placeholder_for_the_same_identifier_twice_returns_the_same_index() {
    // Arrange
    let mut ctx = NormalizationContext::new();

    // Act
    let first = ctx.placeholder("value", PlaceholderKind::Variable);
    let again = ctx.placeholder("value", PlaceholderKind::Variable);

    // Assert
    assert_eq!(first, again);
}

#[test]
fn range_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("0..10");
    // Range -> [from_or_None, to_or_None]

    // Assert
    assert_eq!(*n.kind(), NodeKind::Range);
    assert_eq!(n.children().len(), 2);
    assert!(!n.children()[0].is_none());
    assert!(!n.children()[1].is_none());
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
    let body1 = NormalizationContext::normalize_item_fn(&f1).body;
    let body2 = NormalizationContext::normalize_item_fn(&f2).body;

    // Extract the then_branch from each: Block -> stmts[0] -> If -> children[1]
    let then1 = match &body1.kind() {
        NodeKind::Block => match &body1.children()[0].kind() {
            NodeKind::If => body1.children()[0].children()[1].clone(),
            _ => panic!("expected If"),
        },
        _ => panic!("expected Block"),
    };
    let then2 = match &body2.kind() {
        NodeKind::Block => match &body2.children()[0].kind() {
            NodeKind::If => body2.children()[0].children()[1].clone(),
            _ => panic!("expected If"),
        },
        _ => panic!("expected Block"),
    };

    // Assert
    assert_ne!(then1, then2);
    assert_eq!(
        NormalizedNode::reindex_placeholders(&then1),
        NormalizedNode::reindex_placeholders(&then2)
    );
}

#[test]
fn renamed_variables_produce_identical_trees() {
    // Arrange & Act
    let code1 = "fn foo(x: i32) -> i32 { let y = x + 1; y }";
    let code2 = "fn bar(a: i32) -> i32 { let b = a + 1; b }";
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let normalized_body1 = NormalizationContext::normalize_item_fn(&f1);
    let sig1 = normalized_body1.signature;
    let body1 = normalized_body1.body;
    let normalized_body2 = NormalizationContext::normalize_item_fn(&f2);
    let sig2 = normalized_body2.signature;
    let body2 = normalized_body2.body;

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
    assert_eq!(*n1.kind(), NodeKind::StructInit);
    assert_eq!(*n2.kind(), NodeKind::StructInit);
    // Same number of fields
    assert_eq!(n1.children().len(), n2.children().len());
}

#[test]
fn structural_changes_produce_different_trees() {
    // Arrange & Act
    let code1 = "fn foo(x: i32) -> i32 { x + 1 }";
    let code2 = "fn foo(x: i32) -> i32 { x * 1 }";
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let body1 = NormalizationContext::normalize_item_fn(&f1).body;
    let body2 = NormalizationContext::normalize_item_fn(&f2).body;

    // Assert
    assert_ne!(body1, body2);
}

#[test]
fn try_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("result?");

    // Assert
    assert_eq!(*n.kind(), NodeKind::Try);
}

#[test]
fn tuple_expression_normalized() {
    // Arrange & Act
    let n = normalize_code_expr("(1, 2, 3)");

    // Assert
    assert_eq!(*n.kind(), NodeKind::Tuple);
    assert_eq!(n.children().len(), 3);
}

#[test]
fn tuple_pattern_normalized() {
    // Arrange & Act
    let code1 = "fn foo() { let (a, b) = (1, 2); }";
    let code2 = "fn bar() { let (x, y) = (1, 2); }";
    let f1 = parse_fn(code1);
    let f2 = parse_fn(code2);
    let body1 = NormalizationContext::normalize_item_fn(&f1).body;
    let body2 = NormalizationContext::normalize_item_fn(&f2).body;

    // Assert
    assert_eq!(body1, body2);
}

#[test]
fn type_position_macro_normalized() {
    // Arrange & Act
    let code = "fn foo() -> my_type!(i32) {}";

    // Assert
    if let Ok(f) = parse_str::<ItemFn>(code) {
        let sig = NormalizationContext::normalize_item_fn(&f).signature;

        assert!(NormalizedNode::count_nodes(&sig) > 0);
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
    match &n.kind() {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "vec");
            assert_eq!(n.children().len(), 1);
            assert_eq!(n.children()[0], NormalizedNode::leaf(NodeKind::Opaque));
        }
        _ => panic!("Expected MacroCall node, got {n:?}"),
    }
}

#[test]
fn unparseable_macro_differs_from_no_args() {
    // Arrange & Act
    let n_empty = normalize_code_expr("my_macro!()");
    let n_unparseable = normalize_code_expr("vec![x; 10]");

    // Assert
    match (&n_empty.kind(), &n_unparseable.kind()) {
        (NodeKind::MacroCall { .. }, NodeKind::MacroCall { .. }) => {
            assert!(n_empty.children().is_empty());
            assert_eq!(n_unparseable.children().len(), 1);
            assert_eq!(
                n_unparseable.children()[0],
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
    match &n.kind() {
        NodeKind::MacroCall { name } => {
            assert_eq!(name, "vec");
            assert_eq!(n.children().len(), 3);
        }
        _ => panic!("Expected MacroCall node, got {n:?}"),
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

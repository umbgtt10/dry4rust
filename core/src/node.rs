// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::collections::HashSet;

/// Kinds of literals — preserves type but erases value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralKind {
    Int,
    Float,
    Str,
    ByteStr,
    CStr,
    Byte,
    Char,
    Bool,
    Null,
}

/// Kinds of placeholders — what the original identifier referred to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceholderKind {
    Variable,
    Function,
    Type,
    Lifetime,
    Label,
}

/// Binary operators.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitXor,
    BitAnd,
    BitOr,
    Shl,
    Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    BitXorAssign,
    BitAndAssign,
    BitOrAssign,
    ShlAssign,
    ShrAssign,
    FloorDiv,
    Pow,
    In,
    NotIn,
    Is,
    IsNot,
    FloorDivAssign,
    PowAssign,
    Other,
}

/// Unary operators.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnOpKind {
    Deref,
    Not,
    Neg,
    Other,
}

/// The kind of a normalized AST node. Carries only non-child data
/// (operator kinds, literal kinds, placeholder indices, mutability flags, macro names).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeKind {
    // Blocks and statements
    Block,
    LetBinding,
    Semi,
    Paren,

    // Literals and identifiers
    Literal(LiteralKind),
    Placeholder(PlaceholderKind, usize),

    // Operations
    BinaryOp(BinOpKind),
    UnaryOp(UnOpKind),
    Range,

    // Calls and access
    Call,
    MethodCall,
    FieldAccess,
    Index,
    Path,

    // Closures and functions
    Closure,
    FnSignature,

    // Control flow
    Return,
    Break,
    Continue,
    Assign,

    // References and pointers
    Reference {
        mutable: bool,
    },

    // Compound types
    Tuple,
    Array,
    Set,
    Repeat,

    // Type operations
    Cast,
    StructInit,

    // Async/error
    Await,
    Yield,
    Try,

    // Control flow structures
    If,
    Match,
    MatchArm,
    Loop,
    While,
    ForLoop,
    LetExpr,

    // Patterns
    PatWild,
    PatPlaceholder(PlaceholderKind, usize),
    PatTuple,
    PatStruct,
    PatOr,
    PatLiteral,
    PatReference {
        mutable: bool,
    },
    PatSlice,
    PatRest,
    PatRange,

    // Types
    TypePlaceholder(PlaceholderKind, usize),
    TypeReference {
        mutable: bool,
    },
    TypeTuple,
    TypeSlice,
    TypeArray,
    TypePath,
    TypeImplTrait,
    TypeInfer,
    TypeUnit,
    TypeNever,

    // Field initializer (name = value)
    FieldValue,

    // Macro invocations
    MacroCall {
        name: String,
    },

    // Opaque — unsupported constructs
    Opaque,

    /// Sentinel for absent optional children, ensuring fixed child positions
    /// for correct zip alignment in similarity comparison.
    None,
}

/// A normalized AST node. Uses a data-driven `{ kind, children }` representation
/// instead of a large enum with differently-shaped variants. This allows generic
/// traversal algorithms (count_nodes, reindex, count_matching, extract) to work
/// without exhaustive matching on every variant.
///
/// ## Child ordering conventions
///
/// - **Fixed with None sentinels** (always same child count):
///   - `If` -> [condition, then_branch, else_or_None]
///   - `LetBinding` -> [pattern, type_or_None, init_or_None, diverge_or_None]
///   - `Range` / `PatRange` -> [from_or_None, to_or_None]
///   - `MatchArm` -> [pattern, guard_or_None, body]
/// - **Fixed children first, variable after** (for zip alignment):
///   - `Call` -> [func, arg0, arg1, ...]
///   - `MethodCall` -> [receiver, method, arg0, ...]
///   - `Closure` -> [body, param0, ...]
///   - `FnSignature` -> [return_type_or_None, param0, ...]
///   - `Match` -> [expr, arm0, arm1, ...]
///   - `StructInit` -> [rest_or_None, field0, field1, ...]
///   - `MacroCall` -> [arg0, arg1, ...]
/// - **Variable-length (0 or 1)**: `Return`, `Break` -> `[]` or `[value]`
/// - **Homogeneous**: `Block`, `Tuple`, `Array`, `Path`, `PatTuple`, etc. -> [elem0, ...]
/// - **All other fixed**: e.g. `BinaryOp` -> [left, right], `ForLoop` -> [pat, iter, body]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NormalizedNode {
    pub kind: NodeKind,
    pub children: Vec<Self>,
}

impl NormalizedNode {
    /// Create a leaf node (no children).
    #[must_use]
    pub const fn leaf(kind: NodeKind) -> Self {
        Self {
            kind,
            children: vec![],
        }
    }

    /// Create a node with children.
    #[must_use]
    pub const fn with_children(kind: NodeKind, children: Vec<Self>) -> Self {
        Self { kind, children }
    }

    /// Create a None sentinel node.
    #[must_use]
    pub const fn none() -> Self {
        Self::leaf(NodeKind::None)
    }

    /// Convert an `Option<NormalizedNode>` to a node, using the [`Self::none`] sentinel for
    /// absent values.
    pub fn opt(node: Option<Self>) -> Self {
        node.unwrap_or_else(Self::none)
    }

    /// Check if this is a None sentinel node.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self.kind, NodeKind::None)
    }
}

// -- Placeholder re-indexing --------------------------------------------------

/// Collects all placeholder occurrences in depth-first order, building
/// a mapping from (kind, old_index) -> new_sequential_index.
fn collect_placeholder_order(
    node: &NormalizedNode,
    order: &mut Vec<(PlaceholderKind, usize)>,
    seen: &mut HashSet<(PlaceholderKind, usize)>,
) {
    match &node.kind {
        NodeKind::Placeholder(kind, idx)
        | NodeKind::PatPlaceholder(kind, idx)
        | NodeKind::TypePlaceholder(kind, idx) => {
            if seen.insert((*kind, *idx)) {
                order.push((*kind, *idx));
            }
        }
        _ => {}
    }
    for child in &node.children {
        collect_placeholder_order(child, order, seen);
    }
}

/// Applies the reindex mapping to a node, returning a new node with remapped indices.
fn apply_reindex(
    node: &NormalizedNode,
    mapping: &HashMap<(PlaceholderKind, usize), usize>,
) -> NormalizedNode {
    let kind = match &node.kind {
        NodeKind::Placeholder(kind, idx) => {
            let new_idx = mapping.get(&(*kind, *idx)).copied().unwrap_or(*idx);
            NodeKind::Placeholder(*kind, new_idx)
        }
        NodeKind::PatPlaceholder(kind, idx) => {
            let new_idx = mapping.get(&(*kind, *idx)).copied().unwrap_or(*idx);
            NodeKind::PatPlaceholder(*kind, new_idx)
        }
        NodeKind::TypePlaceholder(kind, idx) => {
            let new_idx = mapping.get(&(*kind, *idx)).copied().unwrap_or(*idx);
            NodeKind::TypePlaceholder(*kind, new_idx)
        }
        other => other.clone(),
    };
    let children = node
        .children
        .iter()
        .map(|c| apply_reindex(c, mapping))
        .collect();
    NormalizedNode { kind, children }
}

/// Re-index all placeholders in a sub-tree so that indices start from 0
/// per kind, assigned by first-occurrence depth-first order.
/// This allows comparing sub-trees extracted from different function contexts.
#[must_use]
pub fn reindex_placeholders(node: &NormalizedNode) -> NormalizedNode {
    let mut order = Vec::new();
    let mut seen = HashSet::new();
    collect_placeholder_order(node, &mut order, &mut seen);

    // Build mapping: (kind, old_index) -> new sequential index per kind
    let mut counters: HashMap<PlaceholderKind, usize> = HashMap::new();
    let mut mapping: HashMap<(PlaceholderKind, usize), usize> = HashMap::new();
    for (kind, old_idx) in order {
        let counter = counters.entry(kind).or_insert(0);
        mapping.insert((kind, old_idx), *counter);
        *counter += 1;
    }

    apply_reindex(node, &mapping)
}

/// Count the number of nodes in a normalized tree.
/// None sentinel nodes are not counted.
pub fn count_nodes(node: &NormalizedNode) -> usize {
    if node.is_none() {
        return 0;
    }
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

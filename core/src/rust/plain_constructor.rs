// Copyright (c) 2026 Matjaz Domen Pecan
// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::Expr;
use syn::FnArg;
use syn::Ident;
use syn::ImplItemFn;
use syn::Member;
use syn::Pat;
use syn::Path;
use syn::Stmt;

/// A constructor whose whole body is the struct built from its own parameters -- `Self { x, y }`,
/// `Self { start: first }`, `Self(left, right)`. Its fingerprint is the struct's shape: two such constructors with
/// as many fields match whatever they build, so a check that counted them would count fields, not copied logic. A
/// constructor that converts, defaults or computes a field is not plain and stays a code unit.
pub struct PlainConstructor;

impl PlainConstructor {
    /// Whether `method`, in an inherent impl of `type_name`, only moves its parameters into a new value of it.
    #[must_use]
    pub fn is_plain(method: &ImplItemFn, type_name: &str) -> bool {
        let [Stmt::Expr(expression, None)] = method.block.stmts.as_slice() else {
            return false;
        };
        let parameters = Self::parameters(method);
        let is_parameter = |value: &Expr| match value {
            Expr::Path(path) => path
                .path
                .get_ident()
                .is_some_and(|ident| parameters.iter().any(|parameter| parameter == ident)),
            _ => false,
        };
        match expression {
            Expr::Struct(built) => {
                built.rest.is_none()
                    && Self::names_the_type(&built.path, type_name)
                    && built.fields.iter().all(|field| {
                        matches!(field.member, Member::Named(_)) && is_parameter(&field.expr)
                    })
            }
            Expr::Call(call) => {
                matches!(&*call.func, Expr::Path(path) if Self::names_the_type(&path.path, type_name))
                    && call.args.iter().all(is_parameter)
            }
            _ => false,
        }
    }

    fn names_the_type(path: &Path, type_name: &str) -> bool {
        path.get_ident()
            .is_some_and(|ident| ident == "Self" || ident == type_name)
    }

    fn parameters(method: &ImplItemFn) -> Vec<Ident> {
        method
            .sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                FnArg::Typed(typed) => match &*typed.pat {
                    Pat::Ident(pattern) => Some(pattern.ident.clone()),
                    _ => None,
                },
                FnArg::Receiver(_) => None,
            })
            .collect()
    }
}

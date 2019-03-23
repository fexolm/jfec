use indextree::{Arena, NodeId};

use super::ast;
use super::ast::helpers::*;

pub fn expect_var(name: &str, scope: NodeId, arena: &Arena<ast::Scope>) -> bool {
    if let Ok(sym) = scope.try_lookup(&name.to_string(), arena) {
        if let ast::Symbol::Variable(ref var) = *sym {
            var.name == name.to_string()
        } else {
            false
        }
    } else {
        false
    }
}
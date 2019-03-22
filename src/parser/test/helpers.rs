use super::ast;
use super::ast::helpers::*;

pub fn expect_var(name: &str, scope: &ast::Scope) -> bool {
    let sym = scope.try_lookup(&name.to_string()).unwrap();
    if let ast::Symbol::Variable(ref var) = *sym {
        var.name == name.to_string()
    } else {
        false
    }
}
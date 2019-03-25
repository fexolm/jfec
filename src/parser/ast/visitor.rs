use super::nodes::*;

#[macro_export]
macro_rules! walk_list {
    ($visitor: expr, $method: ident, $list: expr) => {
        for elem in $list {
            $visitor.$method(elem)
        }
    };
    ($visitor: expr, $method: ident, $list: expr, $($extra_args: expr),*) => {
        for elem in $list {
            $visitor.$method(elem, $($extra_args,)*)
        }
    }
}

pub trait Visitor<'ast>: Sized {
    fn visit_mod(&mut self, m: &'ast Module) { walk_mod(self, m) }
    fn visit_fn_decl(&mut self, f: &'ast FnDecl) { walk_fn_decl(self, f) }
    fn visit_stmt(&mut self, s: &'ast Stmt) { walk_stmt(self, s) }
    fn visit_expr(&mut self, e: &'ast Expr) { walk_expr(self, e) }
    fn visit_block(&mut self, b: &'ast BlockStmt) { walk_block(self, b) }
}

pub fn walk_mod<'a, V: Visitor<'a>>(visitor: &mut V, module: &'a Module) {
    walk_list!(visitor, visit_fn_decl, &module.functions)
}


pub fn walk_fn_decl<'a, V: Visitor<'a>>(visitor: &mut V, decl: &'a FnDecl) {
    visitor.visit_block(&decl.body);
}


pub fn walk_stmt<'a, V: Visitor<'a>>(visitor: &mut V, stmt: &'a Stmt) {
    match stmt {
        Stmt::Assign(ref assign) => {
            visitor.visit_expr(&assign.value);
        },
        Stmt::Block(ref block) => {
            visitor.visit_block(&block);
        },
        Stmt::Expr(ref expr) => {
            visitor.visit_expr(&expr);
        },
    }
}

pub fn walk_expr<'a, V: Visitor<'a>>(visitor: &mut V, expr: &'a Expr) {
    match expr {
        Expr::Id(_) => {},
        Expr::Call(ref call) => {
            walk_list!(visitor, visit_expr, &call.params);
        },
    }
}

pub fn walk_block<'a, V: Visitor<'a>>(visitor: &mut V, block: &'a BlockStmt) {
    walk_list!(visitor, visit_stmt, &block.list);
}


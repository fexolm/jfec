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
    fn visit_item(&mut self, i: &'ast Item) { walk_item(self, i) }
    fn visit_fn_decl(&mut self, f: &'ast FnDecl) { walk_fn_decl(self, f) }
    fn visit_stmt(&mut self, s: &'ast Stmt) { walk_stmt(self, s) }
    fn visit_expr(&mut self, e: &'ast Expr) { walk_expr(self, e) }
    fn visit_block(&mut self, b: &'ast Block) { walk_block(self, b) }
    fn visit_arg(&mut self, _: &'ast Arg) {}
}

pub fn walk_mod<'a, V: Visitor<'a>>(visitor: &mut V, module: &'a Module) {
    walk_list!(visitor, visit_item, &module.items)
}

pub fn walk_item<'a, V: Visitor<'a>>(visitor: &mut V, item: &'a Item) {
    match item.kind {
        ItemKind::Fn(ref decl, ref block) => {
            visitor.visit_fn_decl(&decl);
            visitor.visit_block(&block);
        }
    }
}

pub fn walk_fn_decl<'a, V: Visitor<'a>>(visitor: &mut V, decl: &'a FnDecl) {
    walk_list!(visitor, visit_arg, &decl.inputs)
}


pub fn walk_stmt<'a, V: Visitor<'a>>(visitor: &mut V, stmt: &'a Stmt) {
    match stmt.kind {
        StmtKind::Local(ref local) => {
            visitor.visit_expr(&local.value);
        },
        StmtKind::Block(ref block) => {
            visitor.visit_block(&block);
        },
        StmtKind::Expr(ref expr) => {
            visitor.visit_expr(&expr);
        },
        StmtKind::Item(ref item) => {
            visitor.visit_item(&item);
        },
    }
}

pub fn walk_expr<'a, V: Visitor<'a>>(visitor: &mut V, expr: &'a Expr) {
    match expr.kind {
        ExprKind::Ident(_) => {},
        ExprKind::Literal(_) => {},
        ExprKind::Call(ref call) => {
            walk_list!(visitor, visit_expr, &call.params);
        },
    }
}

pub fn walk_block<'a, V: Visitor<'a>>(visitor: &mut V, block: &'a Block) {
    walk_list!(visitor, visit_stmt, &block.list);
}


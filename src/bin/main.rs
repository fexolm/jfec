extern crate jfec;

use jfec::parser;
use jfec::parser::ast;
use jfec::parser::ast::visitor::{*};
use std::fs;

struct DebugVisitor;

impl<'ast> Visitor<'ast> for DebugVisitor {
    fn visit_item(&mut self, item: &'ast ast::Item) {
        println!("item {}", item.ident);
        walk_item(self, item);
        println!("enditem");
    }
    fn visit_stmt(&mut self, s: &'ast ast::Stmt) {
        println!("statement {:?}", s);
        walk_stmt(self, s);
        println!("endstmt");
    }
    fn visit_expr(&mut self, e: &'ast ast::Expr) {
        println!("expr {:?}", e);
        walk_expr(self, e);
        println!("endexpr");
    }
    fn visit_block(&mut self, b: &'ast ast::Block) {
        println!("block");
        walk_block(self, b);
        println!("endblock");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = fs::read_to_string("program.ce").expect("cannot read file");

    let ast = parser::create_ast(&file).expect("cannot create ast");

    let mut visitor = DebugVisitor {};

    walk_mod(&mut visitor, &ast);

    Ok(())
}

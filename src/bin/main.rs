extern crate jfec;

use jfec::codegen::ssa::visitor::SSAVisitor;
use jfec::parser;
use jfec::parser::ast::visitor::Visitor;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = fs::read_to_string("program.ce").expect("cannot read file");

    let ast = parser::create_ast(&file).expect("cannot create ast");

    let mut visitor = SSAVisitor::new();

    visitor.visit_mod(&ast);

    Ok(())
}

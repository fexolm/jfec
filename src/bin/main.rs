extern crate jfec;
use jfec::{parser, ast};
use std::fs;

fn main() {
    let file = fs::read_to_string("program.ce").expect("cannot read file");

    let ast = parser::create_ast(&file).expect("cannot create ast");

    for f in ast.functions {
        println!("function: {}", f.name);
        for p in f.inputs {
            println!("type: {}, name: {}", p.typ, p.name);
        }

        if let ast::Stmt::Block(ref block) = *f.body {
            for stmt in &block.list {
                println!("{:?}", stmt)
            }
        } else {
            println!("error");
        }
    }
}

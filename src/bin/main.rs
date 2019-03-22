extern crate jfec;

use jfec::parser;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = fs::read_to_string("program.ce").expect("cannot read file");

    let ast = parser::create_ast(&file).expect("cannot create ast");

    for f in ast.functions {
        println!("function: {}", f.name);
        for p in f.inputs {
            println!("type: {}, name: {}", p.typ, p.name);
        }

        for stmt in &f.body.list {
            println!("{:?}", stmt)
        }
    }
    Ok(())
}

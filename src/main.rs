extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

fn main() {
    let ast = parser::create_ast("program.ce");

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

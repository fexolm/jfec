extern crate jfec;

use jfec::parser;
use std::fs;
use jfec::codegen::ssa::emmiter;
use jfec::codegen::ssa::pretty_print;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = fs::read_to_string("program.ce").expect("cannot read file");

    let ast = parser::create_ast(&file).expect("cannot create ast");

    let ssa = emmiter::to_ssa(&ast);

    pretty_print::print_mod(&ssa);

    Ok(())
}

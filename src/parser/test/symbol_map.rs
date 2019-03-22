use std::fs;

use crate::ast;
use crate::ast::helpers::*;
use crate::parser;

#[test]
fn test_symbol_map() {
    let file = fs::read_to_string("testdata/tests.jfec").expect("cannot read file");
    let ast = parser::create_ast(&file).unwrap();

    for f in ast.functions {
        if f.name == "bar" {
            let block = &f.body;
            let sym = block.scope.try_lookup(&"c".to_string()).unwrap();
            if let ast::Symbol::Variable(ref var) = *sym {
                assert_eq!(var.name, "c".to_string());
            } else {
                unreachable!()
            }
        }
    }
}
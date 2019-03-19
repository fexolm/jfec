use crate::parser;
use crate::ast;
use std::fs;

#[test]
fn test_symbol_map() {
    let file = fs::read_to_string("testdata/tests.jfec").expect("cannot read file");
    let ast = if let Ok(ast) = parser::create_ast(&file) {
        ast
    } else {
        unreachable!()
    };

    for f in ast.functions {
        if f.name == "bar" {
            if let ast::Stmt::Block(ref block) = *f.body {
                match block.scope.lookup(&"c".to_string()) {
                    Some(sym) => {
                        if let ast::Symbol::Variable {ref name, ref typ} = *sym {
                            assert_eq!(name, &"c".to_string());
                        } else {
                            unreachable!()
                        }
                    }
                    _ => unreachable!()
                }
            } else {
                unreachable!()
            }
        }
    }
}
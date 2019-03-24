use std::fs;

use super::ast::helpers::StmtExtensions;
use super::helpers;
use super::parser;

#[test]
fn test1() {
    let file = fs::read_to_string("testdata/parser/symbol_map/test1.jfec")
        .expect("cannot read file");
    let ast = parser::create_ast(&file).unwrap();
    let arena = ast.arena;
    for f in ast.functions {
        if f.name == "bar" {
            let scope = f.body.scope;
            assert!(helpers::expect_var("a", scope, &arena));
            assert!(helpers::expect_var("b", scope, &arena));
            assert!(helpers::expect_var("c", scope, &arena));
            assert!(!helpers::expect_var("foo", scope, &arena));

            let block = f.body.list[1].as_block().expect("Second statement must be block");
            assert!(helpers::expect_var("a", block.scope, &arena));
            assert!(helpers::expect_var("b", block.scope, &arena));
            assert!(helpers::expect_var("c", block.scope, &arena));
            assert!(helpers::expect_var("foo", block.scope, &arena));
        }
    }
}
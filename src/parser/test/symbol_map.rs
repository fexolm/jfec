use std::fs;

use super::parser;
use super::helpers;
#[test]
fn test_symbol_map() {
    let file = fs::read_to_string("testdata/tests.jfec").expect("cannot read file");
    let ast = parser::create_ast(&file).unwrap();

    for f in ast.functions {
        if f.name == "bar" {
            let scope = &f.body.scope;
            assert!(helpers::expect_var("c", &scope));
            assert!(helpers::expect_var("a", &scope));
        }
    }
}
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CSVParser;

use pest::iterators::Pair;

fn parse_fn_param(decl: Pair<Rule>) -> ast::FnParam {
    let mut iter = decl.into_inner();
    return ast::FnParam { name: iter.next().unwrap().as_str().to_string(),
                          typ: iter.next().unwrap().as_str().to_string() }
}

fn parse_fn_params(decl: Pair<Rule>) -> Vec<ast::FnParam> {
    let mut res = vec!();
    for el in decl.into_inner() {
        match el.as_rule() {
            Rule::param => {
                res.push(parse_fn_param(el));
            },
            _ => unreachable!(),
        }
    }
    return res;
}

fn parse_fn_ret(decl: Pair<Rule>) -> String {
    decl.as_str().to_string()
}

fn parse_fn_body(decl: Pair<Rule>) -> Vec<ast::Stmt> {
    vec!()
}

fn parse_fn_decl(decl: Pair<Rule>) -> ast::FnDecl {
    let mut res = ast::FnDecl::default();

    for el in decl.into_inner() {
        match el.as_rule() {
            Rule::id => {
                res.name = el.as_str().to_string();
            }
            Rule::param_list => {
                res.params = parse_fn_params(el);
            },
            Rule::ret_typ => {
                res.ret = parse_fn_ret(el);
            },
            Rule::body => {
                res.body = parse_fn_body(el);
            },
            _ => unreachable!(),
        }
    }
    res
}

fn create_ast(program: Pair<Rule>) -> ast::Program {
    let mut fn_decls = vec!();
    for decl in program.into_inner() {
        match decl.as_rule() {
            Rule::decl => {
                fn_decls.push(parse_fn_decl(decl.into_inner().next().unwrap()));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    ast::Program { functions: fn_decls }
}

fn main() {
    let unparsed_file = fs::read_to_string("program.ce").expect("cannot read file");

    let program = CSVParser::parse(Rule::program, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let ast = create_ast(program);

    for f in ast.functions {
        println!("function: {}", f.name);
        for p in f.params {
            println!("type: {}, name: {}", p.typ, p.name);
        }
    }
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

use pest::iterators::Pair;

fn parse_fn_params(decl: Pair<Rule>) {
    println!("{}", decl.as_str());
}

fn parse_fn_ret(decl: Pair<Rule>) {
    println!("{}", decl.as_str());
}

fn parse_fn_body(decl: Pair<Rule>) {
    println!("{}", decl.as_str());
}

fn parse_id(decl: Pair<Rule>) {
    println!("{}", decl.as_str());
}

fn parse_fn_decl(decl: Pair<Rule>) {
    for el in decl.into_inner() {
        match el.as_rule() {
            Rule::id => {
                parse_id(el);
            }
            Rule::param_list => {
                parse_fn_params(el);
            },
            Rule::ret_typ => {
                parse_fn_ret(el);
            },
            Rule::body => {
                parse_fn_body(el);
            },
            _ => unreachable!(),
        }
    }
}

fn create_ast(program: Pair<Rule>) -> ast::Program {
    for decl in program.into_inner() {
        match decl.as_rule() {
            Rule::decl => {
                parse_fn_decl(decl.into_inner().next().unwrap());
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    ast::Program { functions: Vec::new() }
}

fn main() {
    let unparsed_file = fs::read_to_string("program.ce").expect("cannot read file");

    let program = CSVParser::parse(Rule::program, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    create_ast(program);
}

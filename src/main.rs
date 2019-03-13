extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JFECParser;

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

fn parse_call_params(params: Pair<Rule>) -> Vec<ast::Expr> {
    let mut res = vec!();
    for el in params.into_inner() {
        match el.as_rule() {
            Rule::expr => {
               res.push(parse_expr(el));
            },
            _ => unreachable!(),
        }
    }
    res
}

fn parse_call_expr(expr: Pair<Rule>) -> ast::Expr {
    let mut name = String::default();
    let mut params = vec!();
    for el in expr.into_inner() {
        match el.as_rule() {
            Rule::id => {
                name = el.as_str().to_string();
            },
            Rule::call_params => {
                params = parse_call_params(el);
            },
            _ => unreachable!()
        }
    }
    ast::Expr::Call(name, params)
}

fn parse_expr(e: Pair<Rule>) -> ast::Expr {
    let expr = e.into_inner().next().unwrap();
    match expr.as_rule() {
        Rule::id => {
            return ast::Expr::Id(expr.as_str().to_string());
        },
        Rule::call_expr => {
            return parse_call_expr(expr);
        },
        _ => unreachable!(),
    }
}

fn parse_fn_ret(decl: Pair<Rule>) -> String {
    decl.as_str().to_string()
}

fn parse_stmt(decl: Pair<Rule>) -> ast::Stmt {
    let stmt = decl.into_inner().next().unwrap();
    match stmt.as_rule() {
        Rule::assign_stmt => {
            let mut iter = stmt.into_inner();
            let var = iter.next().unwrap().as_str();
            iter.next();
            let val = iter.next().unwrap();
            return ast::Stmt::Assign(var.to_string(), parse_expr(val));
        },
        Rule::expr_stmt => {
            return ast::Stmt::Expr(parse_expr(stmt.into_inner().next().unwrap()));
        },
        Rule::block_stmt => {
            return parse_block(stmt);
        },
        _ => unreachable!(),
    }
}

fn parse_block(decl: Pair<Rule>) -> ast::Stmt {
    let mut list = vec!();
    for el in decl.into_inner().next().unwrap().into_inner() {
        match el.as_rule() {
            Rule::stmt => {
                list.push(parse_stmt(el));
            },
            _ => unreachable!(),
        }
    }
    ast::Stmt::Block(list)
}

fn parse_fn_decl(decl: Pair<Rule>) -> ast::FnDecl {
    let mut name = String::default();
    let mut params = vec!();
    let mut ret = String::default();
    let mut body = ast::Stmt::Invalid;

    for el in decl.into_inner() {
        match el.as_rule() {
            Rule::id => {
                name = el.as_str().to_string();
            }
            Rule::param_list => {
                params = parse_fn_params(el);
            },
            Rule::ret_typ => {
                ret = parse_fn_ret(el);
            },
            Rule::block_stmt => {
                body = parse_block(el);
            },
            _ => unreachable!(),
        }
    }
    ast::FnDecl {name, params, ret, body}
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

    let program = JFECParser::parse(Rule::program, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let ast = create_ast(program);

    for f in ast.functions {
        println!("function: {}", f.name);
        for p in f.params {
            println!("type: {}, name: {}", p.typ, p.name);
        }

        if let ast::Stmt::Block(ref block) = f.body {
            for stmt in block {
                println!("{:?}", stmt)
            }
        } else {
            println!("error");
        }
    }
}

use pest::iterators::Pair;
use pest::Parser;

use std::fs;

use crate::ast;

use super::utils;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct JFECParser;

fn parse_arg(arg_p: Pair<Rule>) -> ast::Arg {
    let mut iter = arg_p.into_inner();
    return ast::Arg {
        name: utils::next_string(&mut iter),
        typ: utils::next_string(&mut iter),
    };
}

fn parse_args(args_p: Pair<Rule>) -> Vec<ast::Arg> {
    let mut res = vec!();
    for p in args_p.into_inner() {
        match p.as_rule() {
            Rule::param => {
                res.push(parse_arg(p));
            }
            _ => unreachable!(),
        }
    }
    return res;
}

fn parse_call_params(call_params_p: Pair<Rule>) -> Vec<Box<ast::Expr>> {
    let mut res = vec!();
    for p in call_params_p.into_inner() {
        match p.as_rule() {
            Rule::expr => {
                res.push(parse_expr(p));
            }
            _ => unreachable!(),
        }
    }
    res
}

fn parse_call_expr(call_expr_p: Pair<Rule>) -> Box<ast::Expr> {
    let mut name = String::default();
    let mut params = vec!();
    for expr_p in call_expr_p.into_inner() {
        match expr_p.as_rule() {
            Rule::id => {
                name = utils::to_string(expr_p);
            }
            Rule::call_params => {
                params = parse_call_params(expr_p);
            }
            _ => unreachable!()
        }
    }
    Box::new(ast::Expr::Call(ast::CallExpr { name, params }))
}

fn parse_expr(expr_p: Pair<Rule>) -> Box<ast::Expr> {
    let expr = utils::next_iter(expr_p);
    match expr.as_rule() {
        Rule::id => {
            return Box::new(
                ast::Expr::Id(ast::IdExpr { name: utils::to_string(expr) }));
        }
        Rule::call_expr => {
            return parse_call_expr(expr);
        }
        _ => unreachable!(),
    }
}

fn parse_stmt(stmt_p: Pair<Rule>) -> Box<ast::Stmt> {
    let stmt = utils::next_iter(stmt_p);
    match stmt.as_rule() {
        Rule::assign_stmt => {
            let mut iter = stmt.into_inner();
            let name = utils::next_string(&mut iter);
            let typ = utils::next_string(&mut iter);
            let val = iter.next().unwrap();
            return Box::new(ast::Stmt::Assign(
                ast::AssignStmt { name, typ, value: parse_expr(val) }
            ));
        }
        Rule::expr_stmt => {
            return Box::new(ast::Stmt::Expr(parse_expr(utils::next_iter(stmt))));
        }
        Rule::block_stmt => {
            return parse_block(stmt);
        }
        _ => unreachable!(),
    }
}

fn parse_block(block_p: Pair<Rule>) -> Box<ast::Stmt> {
    let mut list = vec!();
    for p in utils::next_iter(block_p).into_inner() {
        match p.as_rule() {
            Rule::stmt => {
                list.push(parse_stmt(p));
            }
            _ => unreachable!(),
        }
    }
    Box::new(ast::Stmt::Block(ast::BlockStmt { list }))
}

fn parse_fn_decl(fndecl_p: Pair<Rule>) -> ast::FnDecl {
    let mut name = String::default();
    let mut inputs = vec!();
    let mut output = String::default();
    let mut body = Box::new(ast::Stmt::Invalid);

    for p in fndecl_p.into_inner() {
        match p.as_rule() {
            Rule::id => {
                name = p.as_str().to_string();
            }
            Rule::param_list => {
                inputs = parse_args(p);
            }
            Rule::ret_typ => {
                output = utils::to_string(p)
            }
            Rule::block_stmt => {
                body = parse_block(p);
            }
            _ => unreachable!(),
        }
    }
    ast::FnDecl { name, inputs, output, body }
}

pub fn create_ast(filename: &str) -> ast::Module {
    let file = fs::read_to_string(filename).expect("cannot read file");

    let module = JFECParser::parse(Rule::program, &file)
        .expect("unsuccessful parse").next().unwrap();

    let mut fn_decls = vec!();
    for decl in module.into_inner() {
        match decl.as_rule() {
            Rule::decl => {
                fn_decls.push(parse_fn_decl(decl.into_inner().next().unwrap()));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    ast::Module { functions: fn_decls }
}

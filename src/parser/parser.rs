use pest::iterators::Pair;
use pest::Parser;

use std::fs;
use std::io;
use crate::ast;

use super::utils;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct JFECParser;

fn parse_arg(arg_p: Pair<Rule>) -> Result<ast::Arg, io::Error> {
    let mut iter = arg_p.into_inner();
    let name = utils::next_string(&mut iter)?;
    let typ = utils::next_string(&mut iter)?;
    return Ok(ast::Arg { name, typ });
}

fn parse_args(args_p: Pair<Rule>) -> Result<Vec<ast::Arg>, io::Error> {
    let mut res = vec!();
    for p in args_p.into_inner() {
        match p.as_rule() {
            Rule::param => {
                let arg = parse_arg(p)?;
                res.push(arg);
            }
            _ => unreachable!(),
        }
    }
    return Ok(res);
}

fn parse_call_params(call_params_p: Pair<Rule>) -> Result<Vec<Box<ast::Expr>>, io::Error> {
    let mut res = vec!();
    for p in call_params_p.into_inner() {
        match p.as_rule() {
            Rule::expr => {
                let expr = parse_expr(p)?;
                res.push(expr);
            }
            _ => unreachable!(),
        }
    }
    Ok(res)
}

fn parse_call_expr(call_expr_p: Pair<Rule>) -> Result<Box<ast::Expr>, io::Error> {
    let mut name = String::default();
    let mut params = vec!();
    for expr_p in call_expr_p.into_inner() {
        match expr_p.as_rule() {
            Rule::id => {
                name = utils::to_string(expr_p);
            }
            Rule::call_params => {
                params = parse_call_params(expr_p)?;
            }
            _ => unreachable!()
        }
    }
    Ok(Box::new(ast::Expr::Call(ast::CallExpr { name, params })))
}

fn parse_expr(expr_p: Pair<Rule>) -> Result<Box<ast::Expr>, io::Error> {
    let expr = utils::inner_next(expr_p)?;
    match expr.as_rule() {
        Rule::id => {
            return Ok(Box::new(
                ast::Expr::Id(ast::IdExpr { name: utils::to_string(expr) })));
        }
        Rule::call_expr => {
            return parse_call_expr(expr);
        }
        _ => unreachable!(),
    }
}

fn parse_stmt(stmt_p: Pair<Rule>) -> Result<Box<ast::Stmt>, io::Error> {
    let stmt = utils::inner_next(stmt_p)?;
    match stmt.as_rule() {
        Rule::assign_stmt => {
            let mut iter = stmt.into_inner();
            let name = utils::next_string(&mut iter)?;
            let typ = utils::next_string(&mut iter)?;
            let val = utils::get_next(&mut iter)?;
            let value = parse_expr(val)?;
            return Ok(Box::new(ast::Stmt::Assign(
                ast::AssignStmt { name, typ, value }
            )));
        }
        Rule::expr_stmt => {
            let next = utils::inner_next(stmt)?;
            let expr = parse_expr(next)?;
            return Ok(Box::new(ast::Stmt::Expr(expr)));
        }
        Rule::block_stmt => {
            let block = parse_block(stmt)?;
            return Ok(block);
        }
        _ => unreachable!(),
    }
}

fn parse_block(block_p: Pair<Rule>) -> Result<Box<ast::Stmt>, io::Error> {
    let mut list = vec!();
    let next = utils::inner_next(block_p)?;
    for p in next.into_inner() {
        match p.as_rule() {
            Rule::stmt => {
                let stmt = parse_stmt(p)?;
                list.push(stmt);
            }
            _ => unreachable!(),
        }
    }
    Ok(Box::new(ast::Stmt::Block(ast::BlockStmt { list })))
}

fn parse_fn_decl(fndecl_p: Pair<Rule>) -> Result<ast::FnDecl, io::Error> {
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
                inputs = parse_args(p)?;
            }
            Rule::ret_typ => {
                output = utils::to_string(p);
            }
            Rule::block_stmt => {
                body = parse_block(p)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(ast::FnDecl { name, inputs, output, body })
}


pub fn create_ast(filename: &str) -> Result<ast::Module, io::Error> {
    let file = fs::read_to_string(filename)?;
    let mut parsed = JFECParser::parse(Rule::program, &file)?;
    let module = utils::get_next(&mut parsed)?;

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
    Ok(ast::Module { functions: fn_decls })
}

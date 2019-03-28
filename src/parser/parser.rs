use pest::iterators::Pair;
use pest::Parser;
use std::io;

use super::ast;
use super::utils;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct JFECParser;

fn parse_arg(arg_p: Pair<Rule>) -> Result<ast::Arg, io::Error> {
    let mut iter = arg_p.into_inner();
    let name = utils::next_string(&mut iter)?;
    let typ = utils::next_string(&mut iter)?;
    return Ok(ast::Arg { ident: name, typ });
}

fn parse_args(args_p: Pair<Rule>) -> Result<Vec<Box<ast::Arg>>, io::Error> {
    let mut res = vec!();
    for p in args_p.into_inner() {
        match p.as_rule() {
            Rule::param => {
                let arg = parse_arg(p)?;
                res.push(Box::new(arg));
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

fn parse_call_expr(call_expr_p: Pair<Rule>) -> Result<Box<ast::Call>, io::Error> {
    let mut ident = String::default();
    let mut params = vec!();
    for expr_p in call_expr_p.into_inner() {
        match expr_p.as_rule() {
            // TODO: use any expr here (blocker is left recursion in grammar)
            Rule::ident => {
                ident = utils::to_string(expr_p);
            }
            Rule::call_params => {
                params = parse_call_params(expr_p)?;
            }
            _ => unreachable!()
        }
    }
    Ok(Box::new(ast::Call { ident, params }))
}

fn parse_expr(expr_p: Pair<Rule>) -> Result<Box<ast::Expr>, io::Error> {
    let expr = utils::inner_next(expr_p)?;
    match expr.as_rule() {
        Rule::ident => {
            return Ok(Box::new(
                ast::Expr {
                    kind: ast::ExprKind::Ident(utils::to_string(expr))
                }));
        }
        Rule::call_expr => {
            return Ok(Box::new(
                ast::Expr {
                    kind: ast::ExprKind::Call(
                        parse_call_expr(expr)?
                    )
                }))
        },
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
            return Ok(Box::new(ast::Stmt {
                kind: ast::StmtKind::Local(
                    Box::new(ast::Local {
                        ident: name,
                        typ,
                        value,
                    })
                )
            }));
        }
        Rule::expr_stmt => {
            let next = utils::inner_next(stmt)?;
            let expr = parse_expr(next)?;
            return Ok(Box::new(
                ast::Stmt {
                    kind: ast::StmtKind::Expr(expr)
                }
            ));
        }
        Rule::block_stmt => {
            let block = parse_block(stmt)?;
            return Ok(Box::new(ast::Stmt {
                kind: ast::StmtKind::Block(block)
            }));
        }
        _ => unreachable!(),
    }
}

fn parse_block(block_p: Pair<Rule>) -> Result<ast::Block, io::Error> {
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
    Ok(ast::Block { list })
}

fn parse_fn_decl(fndecl_p: Pair<Rule>) -> Result<Box<ast::Item>, io::Error> {
    let mut name = String::default();
    let mut inputs = vec!();
    let mut output = None;
    let mut body = ast::Block::default();
    for p in fndecl_p.into_inner() {
        match p.as_rule() {
            Rule::ident => {
                name = p.as_str().to_string();
            }
            Rule::param_list => {
                inputs = parse_args(p)?;
            }
            Rule::ret_typ => {
                if let Ok(next) = utils::inner_next(p) {
                    // TODO: don't cause error to handle case with empty return type
                    let s = utils::to_string(next);
                    output = Some(s);
                }
            }
            Rule::block_stmt => {
                body = parse_block(p)?;
            }
            _ => unreachable!(),
        }
    }
    let fndecl = ast::FnDecl { inputs, output };
    Ok(Box::new(ast::Item {
        kind: ast::ItemKind::Fn(Box::new(fndecl), Box::new(body)),
        ident: name,
    }))
}

fn parse_decl(decl: Pair<Rule>) -> Result<Box<ast::Item>, io::Error> {
    match decl.as_rule() {
        Rule::fn_decl => parse_fn_decl(decl),
        _ => unreachable!(),
    }
}

pub fn create_ast(text: &String) -> Result<ast::Module, io::Error> {
    let mut parsed = JFECParser::parse(Rule::program, &text).expect("parse error");
    let module = utils::get_next(&mut parsed)?;
    let mut items = vec!();
    for decl in module.into_inner() {
        match decl.as_rule() {
            Rule::decl => {
                let next = utils::inner_next(decl)?;
                let item = parse_decl(next)?;
                items.push(item);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(ast::Module { items })
}
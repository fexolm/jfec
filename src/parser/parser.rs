use indextree::{Arena, NodeId};
use pest::iterators::Pair;
use pest::Parser;
use std::io;

use super::ast;
use super::ast::helpers::ScopeExtensions;
use super::ast::ScopeNode;
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

fn parse_stmt(stmt_p: Pair<Rule>, scope: NodeId, arena: &mut Arena<ast::Scope>)
              -> Result<Box<ast::Stmt>, io::Error> {
    let stmt = utils::inner_next(stmt_p)?;
    match stmt.as_rule() {
        Rule::assign_stmt => {
            let mut iter = stmt.into_inner();
            let name = utils::next_string(&mut iter)?;
            let typ = utils::next_string(&mut iter)?;
            let val = utils::get_next(&mut iter)?;
            let value = parse_expr(val)?;
            let t = scope.try_lookup(&typ, arena)?;
            let scope_var = scope.add_symbol(&name, ast::Symbol::new_var(name.clone(), t), arena);
            return Ok(Box::new(ast::Stmt::Assign(
                ast::AssignStmt { symbol: scope_var, value }
            )));
        }
        Rule::expr_stmt => {
            let next = utils::inner_next(stmt)?;
            let expr = parse_expr(next)?;
            return Ok(Box::new(ast::Stmt::Expr(expr)));
        }
        Rule::block_stmt => {
            let block = parse_block(stmt, scope, arena)?;
            return Ok(Box::new(ast::Stmt::Block(block)));
        }
        _ => unreachable!(),
    }
}

fn parse_block(block_p: Pair<Rule>, parent_scope: NodeId, arena: &mut Arena<ast::Scope>) -> Result<ast::BlockStmt, io::Error> {
    let mut list = vec!();
    let scope = parent_scope.add_child(arena);
    let next = utils::inner_next(block_p)?;
    for p in next.into_inner() {
        match p.as_rule() {
            Rule::stmt => {
                let stmt = parse_stmt(p, scope, arena)?;
                list.push(stmt);
            }
            _ => unreachable!(),
        }
    }
    Ok(ast::BlockStmt { list, scope })
}

fn parse_fn_decl(fndecl_p: Pair<Rule>, scope: NodeId, arena: &mut Arena<ast::Scope>) -> Result<ast::FnDecl, io::Error> {
    let mut name = String::default();
    let mut inputs = vec!();
    let mut output = String::default();
    let mut body = ast::BlockStmt::default();
    let fn_scope = scope.add_child(arena);
    for p in fndecl_p.into_inner() {
        match p.as_rule() {
            Rule::id => {
                name = p.as_str().to_string();
            }
            Rule::param_list => {
                inputs = parse_args(p)?;
                for i in &inputs {
                    let typ = scope.try_lookup(&i.typ, arena)?;
                    let name = &i.name;
                    fn_scope.add_symbol(name, ast::Symbol::new_var(name.clone(), typ), arena);
                }
            }
            Rule::ret_typ => {
                output = utils::to_string(p);
            }
            Rule::block_stmt => {
                body = parse_block(p, fn_scope, arena)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(ast::FnDecl { name, inputs, output, body })
}

pub fn create_ast(text: &String) -> Result<ast::Module, io::Error> {
    let mut arena = Arena::new();
    let mut parsed = JFECParser::parse(Rule::program, &text).expect("parse error");
    let module = utils::get_next(&mut parsed)?;
    let scope = ast::Scope::new(&mut arena);
    init_types(scope, &mut arena);
    let mut fn_decls = vec!();
    for decl in module.into_inner() {
        match decl.as_rule() {
            Rule::decl => {
                let next = utils::inner_next(decl)?;
                let decl = parse_fn_decl(next, scope, &mut arena)?;
                fn_decls.push(decl);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(ast::Module { functions: fn_decls, arena })
}

fn init_types(scope: NodeId, arena: &mut Arena<ast::Scope>) {
    scope.add_symbol(&"int".to_string(), ast::Symbol::Typ("int".to_string()), arena);
    scope.add_symbol(&"float".to_string(), ast::Symbol::Typ("float".to_string()), arena);
}
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast_v2;
use ast_v2 as ast;

use std::rc::Rc as Rc;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct JFECParser;

use pest::iterators::Pair;

fn parse_arg(decl: Pair<Rule>) -> ast::Arg {
    let mut iter = decl.into_inner();
    return ast::Arg {
        name: iter.next().unwrap().as_str().to_string(),
        typ: iter.next().unwrap().as_str().to_string()
    }
}

fn parse_args(decl: Pair<Rule>) -> Vec<ast::Arg> {
    let mut res = vec!();
    for el in decl.into_inner() {
        match el.as_rule() {
            Rule::param => {
                res.push(parse_arg(el));
            },
            _ => unreachable!(),
        }
    }
    return res;
}

fn parse_call_params(params: Pair<Rule>) -> Vec<Rc<ast::Expr>> {
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

fn parse_call_expr(expr: Pair<Rule>) -> Rc<ast::Expr> {
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
    Rc::new(ast::Expr::Call(ast::CallExpr {name, params}))
}

fn parse_expr(e: Pair<Rule>) -> Rc<ast::Expr> {
    let expr = e.into_inner().next().unwrap();
    match expr.as_rule() {
        Rule::id => {
            return Rc::new(
                ast::Expr::Id(ast::IdExpr {name :expr.as_str().to_string()}));
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

fn parse_stmt(decl: Pair<Rule>) -> Rc<ast::Stmt> {
    let stmt = decl.into_inner().next().unwrap();
    match stmt.as_rule() {
        Rule::assign_stmt => {
            let mut iter = stmt.into_inner();
            let name = iter.next().unwrap().as_str().to_string();
            let typ = iter.next().unwrap().as_str().to_string();
            let val = iter.next().unwrap();
            return Rc::new(ast::Stmt::Assign(
                ast::AssignStmt{ name, typ, value: parse_expr(val)}
            ));
        },
        Rule::expr_stmt => {
            return Rc::new(ast::Stmt::Expr(parse_expr(stmt.into_inner().next().unwrap())));
        },
        Rule::block_stmt => {
            return parse_block(stmt);
        },
        _ => unreachable!(),
    }
}

fn parse_block(decl: Pair<Rule>) -> Rc<ast::Stmt> {
    let mut list = vec!();
    for el in decl.into_inner().next().unwrap().into_inner() {
        match el.as_rule() {
            Rule::stmt => {
                list.push(parse_stmt(el));
            },
            _ => unreachable!(),
        }
    }
    Rc::new(ast::Stmt::Block(ast::BlockStmt{list}))
}

fn parse_fn_decl(decl: Pair<Rule>) -> ast::FnDecl {
    let mut name = String::default();
    let mut inputs = vec!();
    let mut output = String::default();
    let mut body = Rc::new(ast::Stmt::Invalid);

    for el in decl.into_inner() {
        match el.as_rule() {
            Rule::id => {
                name = el.as_str().to_string();
            }
            Rule::param_list => {
                inputs = parse_args(el);
            },
            Rule::ret_typ => {
                output = parse_fn_ret(el);
            },
            Rule::block_stmt => {
                body = parse_block(el);
            },
            _ => unreachable!(),
        }
    }
    ast::FnDecl {name, inputs, output, body}
}

fn create_ast(program: Pair<Rule>) -> ast::Module {
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
    ast::Module { functions: fn_decls }
}

fn main() {
    let unparsed_file = fs::read_to_string("program.ce").expect("cannot read file");

    let program = JFECParser::parse(Rule::program, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let ast = create_ast(program);

    for f in ast.functions {
        println!("function: {}", f.name);
        for p in f.inputs {
            println!("type: {}, name: {}", p.typ, p.name);
        }

        if let ast::Stmt::Block(ref block) = *f.body {
            for stmt in &block.list {
                println!("{:?}", stmt)
            }
        } else {
            println!("error");
        }
    }
}

use std::rc::Rc;

use super::symbol_map::*;

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<FnDecl>
}

#[derive(Debug)]
pub struct FnDecl {
    pub name: String,
    pub inputs: Vec<Arg>,
    pub output: String,
    pub body: BlockStmt,
}

#[derive(Debug)]
pub struct Arg {
    pub typ: String,
    pub name: String,
}

#[derive(Debug)]
pub enum Stmt {
    Invalid,
    Assign(AssignStmt),
    Block(BlockStmt),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct AssignStmt {
    pub symbol: Rc<Symbol>,
    pub value: Box<Expr>,
}

#[derive(Debug, Default)]
pub struct BlockStmt {
    pub list: Vec<Box<Stmt>>,
    pub scope: Rc<Scope>,
}

#[derive(Debug)]
pub enum Expr {
    Id(IdExpr),
    Call(CallExpr),
}

#[derive(Debug)]
pub struct IdExpr {
    pub name: String
}

#[derive(Debug)]
pub struct CallExpr {
    pub name: String,
    pub params: Vec<Box<Expr>>,
}
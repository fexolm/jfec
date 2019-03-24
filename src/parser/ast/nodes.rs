use indextree::{Arena, NodeId};
use std::rc::Rc;

use super::symbol_map::*;

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<FnDecl>,
    pub arena: Arena<Scope>,
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

#[derive(Debug)]
pub struct BlockStmt {
    pub list: Vec<Box<Stmt>>,
    pub scope: NodeId,
}

impl Default for BlockStmt {
    fn default() -> Self {
        BlockStmt {list: Vec::default(), scope: NodeId::new(0) }
    }
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
use super::symbol_map::*;

#[derive(Debug)]
pub struct Module<'a> {
    pub functions: Vec<FnDecl<'a>>
}

#[derive(Debug)]
pub struct FnDecl<'a> {
    pub name: String,
    pub inputs: Vec<Arg>,
    pub output: String,
    pub body: Box<Stmt<'a>>,
}

#[derive(Debug)]
pub struct Arg {
    pub typ: String,
    pub name: String,
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Invalid,
    Assign(AssignStmt),
    Block(BlockStmt<'a>),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct AssignStmt {
    pub name: String,
    pub typ: String,
    pub value: Box<Expr>,
}

#[derive(Debug)]
pub struct BlockStmt<'a> {
    pub list: Vec<Box<Stmt<'a>>>,
    pub scope: Scope<'a>,
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
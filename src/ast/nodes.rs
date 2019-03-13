use std::rc::Rc as Rc;

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<FnDecl>
}

#[derive(Debug)]
pub struct FnDecl {
    pub name: String,
    pub inputs: Vec<Arg>,
    pub output: String,
    pub body: Rc<Stmt>,
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
    Expr(Rc<Expr>),
}

#[derive(Debug)]
pub struct AssignStmt {
    pub name: String,
    pub typ: String,
    pub value: Rc<Expr>,
}

#[derive(Debug)]
pub struct BlockStmt {
    pub list: Vec<Rc<Stmt>>,
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
    pub params: Vec<Rc<Expr>>,
}
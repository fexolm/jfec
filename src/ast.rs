#[derive(Default)]
pub struct Program {
    pub functions: Vec<FnDecl>,
}

pub struct FnDecl {
    pub name: String,
    pub params: Vec<FnParam>,
    pub ret: String,
    pub body: Stmt,
}

#[derive(Default, Debug)]
pub struct FnParam {
    pub typ: String,
    pub name: String,
}

#[derive(Debug)]
pub enum Stmt {
    Invalid,
    Assign(String, Expr),
    Block(Vec<Stmt>),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Id(String),
    Call(String, Vec<Expr>),
}



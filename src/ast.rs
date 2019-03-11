pub struct Program {
    pub functions: Vec<FnDecl>,
}

pub struct FnDecl {
    pub name: String,
    pub params: Vec<FnParam>,
    pub ret: String,
    pub body: Vec<Stmt>
}

pub struct FnParam {
    pub typ: String,
    pub name: String,
}

pub enum Stmt {
    Assign(String, Expr),
}

pub enum Expr {
    Term(String),
    FnCall(String, Vec<Expr>),
}


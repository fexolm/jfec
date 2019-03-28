use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Module {
    pub items: Vec<Box<Item>>
}

#[derive(Debug)]
pub struct Item {
    pub ident: String,
    pub kind: ItemKind,
}

#[derive(Debug)]
pub enum ItemKind {
    Fn(Box<FnDecl>, Box<Block>),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
}

#[derive(Debug)]
pub struct FnDecl {
    pub inputs: Vec<Box<Arg>>,
    pub output: Option<String>,
}

#[derive(Debug)]
pub struct Arg {
    pub typ: String,
    pub ident: String,
}

#[derive(Debug)]
pub enum StmtKind {
    Local(Box<Local>),
    Item(Box<Item>),
    Block(Block),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct Local {
    pub ident: String,
    pub typ: String,
    pub value: Box<Expr>,
}

#[derive(Debug, Default)]
pub struct Block {
    pub list: Vec<Box<Stmt>>,
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug)]
pub enum ExprKind {
    Ident(String),
    Literal(Box<Literal>),
    Call(Box<Call>),
}

#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.kind {
            LiteralKind::Str(ref val) => write!(f, "\"{}\"", val),
            LiteralKind::Int(val) => write!(f, "{}", val),
            LiteralKind::Float(val) => write!(f, "{}", val),
            LiteralKind::Bool(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug)]
pub enum LiteralKind {
    Str(String),
    Int(u64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug)]
pub struct Call {
    pub ident: String,
    pub params: Vec<Box<Expr>>,
}
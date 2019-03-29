use std::fmt::{Display, Formatter, Result};

pub struct Instruction {
    pub kind: InstructionKind
}

pub enum InstructionKind {
    Call(Box<Call>),
    Add(Box<BinaryOp>),
    Sub(Box<BinaryOp>),
    Mul(Box<BinaryOp>),
    Div(Box<BinaryOp>),
    Assign(Box<Assign>),
}

#[derive(Copy, Clone)]
pub enum TypeKind {
    Int,
    String,
    Bool,
    Float,
}

impl TypeKind {
    pub fn from_string(s: &String) -> Self {
        use std::convert::AsRef;
        match s.as_ref() {
            "int" => TypeKind::Int,
            "string" => TypeKind::String,
            "bool" => TypeKind::Bool,
            "float" => TypeKind::Float,
            _ => unreachable!(),
        }
    }
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            TypeKind::Int => write!(f, "i64"),
            TypeKind::String => write!(f, "string"),
            TypeKind::Bool => write!(f, "bool"),
            TypeKind::Float => write!(f, "f64"),
        }
    }
}

pub struct Reg {
    pub id: String,
    pub typ: TypeKind,
}


pub struct Call {
    pub ident: String,
    pub args: Vec<Reg>,
    pub res: Option<Reg>,
}

pub struct BinaryOp {
    pub lhs: Reg,
    pub rhs: Reg,
    pub res: Reg,
}

pub enum Value {
    Reg(Reg),
    Literal(String),
}

pub struct Assign {
    pub lhs: Reg,
    pub rhs: Value,
}
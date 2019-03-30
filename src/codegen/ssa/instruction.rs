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
    I32,
    I64,
    F32,
    F64,
    Bool,
}

impl TypeKind {
    pub fn from_string(s: &String) -> Self {
        use std::convert::AsRef;
        match s.as_ref() {
            "i32" => TypeKind::I32,
            "i64" => TypeKind::I64,
            "f32" => TypeKind::F32,
            "f64" => TypeKind::F64,
            "bool" => TypeKind::Bool,
            _ => unreachable!(),
        }
    }
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            TypeKind::I32 => write!(f, "i32"),
            TypeKind::I64 => write!(f, "i64"),
            TypeKind::F32 => write!(f, "f32"),
            TypeKind::F64 => write!(f, "f64"),
            TypeKind::Bool => write!(f, "bool"),
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
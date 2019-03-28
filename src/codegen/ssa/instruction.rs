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

pub enum TypeKind {
    Int,
    String,
    Bool,
    Float,
    Ptr(Box<TypeKind>),
}

impl TypeKind {
    pub fn from_string(s: &String) -> Self {
        use std::convert::AsRef;
        match s.as_ref() {
            "int" => TypeKind::Int,
            "string" => TypeKind::String,
            "bool" => TypeKind::Bool,
            "float" => TypeKind::Bool,
            _ => unreachable!(),
        }
    }
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            TypeKind::Int => write!(f, "int"),
            TypeKind::String => write!(f, "string"),
            TypeKind::Bool => write!(f, "bool"),
            TypeKind::Float => write!(f, "float"),
            TypeKind::Ptr(ref typ) => write!(f, "{}*", typ.to_string()),
        }
    }
}

pub struct Reg {
    pub id: String,
    pub typ: TypeKind,
}


pub struct Call {
    pub ident: String,
    //TODO: s/String/Reg (+ implement map<String/Reg> or smth like that)
    pub args: Vec<String>,
    //TODO: s/String/Option(Reg)
    pub res: String,
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
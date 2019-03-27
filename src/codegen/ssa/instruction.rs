use indextree::NodeId;

pub struct Instruction {
    pub kind: InstructionKind
}

pub enum InstructionKind {
    Call(Box<Call>),
    Add(Box<BinaryOp>),
    Sub(Box<BinaryOp>),
    Mul(Box<BinaryOp>),
    Div(Box<BinaryOp>),
    Alloca(Box<Alloca>),
    Load(Box<Load>),
    Store(Box<Store>),
}

pub enum TypeKind {
    Int,
    String,
    Char,
    Bool,
    Ptr(Box<TypeKind>),
}

pub struct Reg {
    pub id: NodeId,
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

pub struct Alloca {
    pub typ: TypeKind,
    pub res: Reg,
}

pub struct Load {
    pub reg: Reg,
    pub res: Reg,
}

pub enum Value {
    Reg(Reg),
    Literal(String),
}

pub struct Store {
    pub from: Value,
    pub to: Reg,
}

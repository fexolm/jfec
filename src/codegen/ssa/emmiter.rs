use crate::parser::ast;
use crate::parser::ast::visitor::*;

use super::instruction::*;
use super::visitor::SSAVisitor;

pub struct Module {
    pub functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Module { functions: vec!() }
    }
}

pub struct Function {
    pub name: String,
    pub params: Vec<Reg>,
    pub body: Vec<Instruction>,
    pub ret: Option<TypeKind>,
}

impl Function {
    pub fn new() -> Self {
        Function { name: String::default(), body: vec!(), params: vec!(), ret: None }
    }
}

pub fn to_ssa(ast: &ast::Module) -> Module {
    let mut visitor = SSAVisitor::new();

    visitor.visit_mod(&ast);

    return visitor.get_mod();
}
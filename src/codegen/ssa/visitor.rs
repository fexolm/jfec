use crate::parser::ast;
use ast::visitor::*;
use std::collections::HashMap;

pub struct SSAVisitor {
    var_num: i32,
    tmp_num: i32,
    locals: HashMap<String, i32>,
    stack: Vec<i32>,
}

impl SSAVisitor {
    pub fn new() -> Self {
        SSAVisitor { var_num: 0, tmp_num: 0, locals: HashMap::new(), stack: vec!() }
    }
}

impl<'ast> Visitor<'ast> for SSAVisitor {
    fn visit_fn_decl(&mut self, f: &'ast ast::FnDecl) {
        for arg in &f.inputs {
            println!("var_{} = arg_{}", self.var_num, &arg.ident);
            self.locals.insert(arg.ident.clone(), self.var_num);
            self.var_num+=1;
        }
        walk_fn_decl(self, f)
    }

    fn visit_expr(&mut self, e: &'ast ast::Expr) {
        match e.kind {
            ast::ExprKind::Ident(ref name) => {
                println!("tmp_{} = var_{};", self.tmp_num, self.locals[name]);
                self.stack.push(self.tmp_num);
                self.tmp_num+=1;
            },
            ast::ExprKind::Literal(ref val) => {
                println!("tmp_{} = {}", self.tmp_num, val);
                self.stack.push(self.tmp_num);
                self.tmp_num+=1;
            },
            ast::ExprKind::Call(ref call) => {
                for expr in &call.params {
                    self.visit_expr(expr);
                }
                print!("tmp_{} = call {} ( ", self.tmp_num, &call.ident);
                for _ in &call.params {
                    match self.stack.pop() {
                        Some(num) => print!("tmp_{} ", num),
                        _ => {},
                    }
                }
                self.stack.push(self.tmp_num);
                self.tmp_num+=1;
                println!(")");
            }
        }
    }
}
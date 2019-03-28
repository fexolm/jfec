use ast::visitor::*;

use crate::parser::ast;

struct VisitorState {
    last_tmp: i32,
    tmp_stack: Vec<String>,
}

impl VisitorState {
    fn new() -> Self {
        VisitorState { last_tmp: 0, tmp_stack: vec!() }
    }

    fn new_tmp(&mut self) -> String {
        let tmp = self.last_tmp.to_string();
        self.tmp_stack.push(self.last_tmp.to_string());
        self.last_tmp += 1;
        return tmp;
    }

    fn pop_last(&mut self) -> String {
        match self.tmp_stack.pop() {
            Some(num) => num,
            _ => unreachable!(),
        }
    }

    fn push_local(&mut self, var: String) {
        self.tmp_stack.push(var);
    }

    fn last_n(&mut self, n: usize) -> Vec<String> {
        let mut res = vec!["".to_string(); n];
        for i in 0..n {
            res[n - 1 - i] = self.pop_last();
        }
        return res;
    }
}

pub struct SSAVisitor {
    state: VisitorState,
}

impl SSAVisitor {
    pub fn new() -> Self {
        SSAVisitor { state: VisitorState::new() }
    }
}

impl<'ast> Visitor<'ast> for SSAVisitor {
    fn visit_expr(&mut self, e: &'ast ast::Expr) {
        match e.kind {
            ast::ExprKind::Ident(ref name) => {
                self.state.push_local(name.to_string());
            },
            ast::ExprKind::Literal(ref val) => {
                println!("${} = {}", self.state.new_tmp(), val);
            },
            ast::ExprKind::Call(ref call) => {
                for expr in &call.params {
                    self.visit_expr(expr);
                }
                let params = self.state.last_n(call.params.len());
                print!("${} = call {} ( ", self.state.new_tmp(), &call.ident);
                for p in &params {
                    print!("${} ", p);
                }
                println!(")");
            }
        }
    }
}

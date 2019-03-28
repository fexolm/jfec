use ast::visitor::*;
use super::emmiter;
use crate::parser::ast;
use super::instruction::*;

struct VisitorState {
    last_tmp: i32,
    tmp_stack: Vec<String>,
    m: emmiter::Module,
}

impl VisitorState {
    fn new() -> Self {
        VisitorState { last_tmp: 0, tmp_stack: vec!(), m: emmiter::Module::new() }
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

    fn push_function(&mut self, name: String) {
        let mut f = emmiter::Function::new();
        f.name = name;
        self.m.functions.push(f);
    }

    fn push_ret_type(&mut self, typ: Option<TypeKind>) {
        let len = self.m.functions.len();
        self.m.functions[len - 1].ret = typ;
    }

    fn push_fn_arg(&mut self, reg: Reg) {
        let len = self.m.functions.len();
        self.m.functions[len - 1].params.push(reg);
    }

    fn push_instruction(&mut self, inst: Instruction) {
        let len = self.m.functions.len();
        self.m.functions[len - 1].body.push(inst);
    }
}

pub struct SSAVisitor {
    state: VisitorState,
}

impl SSAVisitor {
    pub fn new() -> Self {
        SSAVisitor { state: VisitorState::new() }
    }

    pub fn get_mod(&mut self) -> emmiter::Module {
        let mut m = emmiter::Module::new();
        std::mem::swap(&mut m, &mut self.state.m);
        m
    }
}

fn literal_type(lit: &ast::Literal) -> TypeKind {
    match &lit.kind {
        ast::LiteralKind::Str(_) => TypeKind::String,
        ast::LiteralKind::Int(_) => TypeKind::Int,
        ast::LiteralKind::Float(_) => TypeKind::Float,
        ast::LiteralKind::Bool(_) => TypeKind::Bool,
    }
}

impl<'ast> Visitor<'ast> for SSAVisitor {
    fn visit_item(&mut self, item: &'ast ast::Item) {
        // We don't have any items except functions. So we assume item to be a function.
        self.state.push_function(item.ident.to_string());
        walk_item(self, item)
    }

    fn visit_fn_decl(&mut self, f: &'ast ast::FnDecl) {
        self.state.push_ret_type(
            if let Some(typ) = &f.output {
                Some(TypeKind::from_string(&typ))
            } else {
                None
            }
        );
        walk_fn_decl(self, f);
    }

    fn visit_arg(&mut self, a: &'ast ast::Arg) {
        let reg = Reg { id: a.ident.to_string(), typ: TypeKind::from_string(&a.typ) };
        self.state.push_fn_arg(reg);
    }

    fn visit_stmt(&mut self, s: &'ast ast::Stmt) {
        match s.kind {
            ast::StmtKind::Local(ref local) => {
                let reg = Reg {
                    id: local.ident.to_string(),
                    typ: TypeKind::from_string(&local.typ),
                };
                walk_stmt(self, s);
                let inst = Instruction {
                    kind: InstructionKind::Assign(Box::new(
                        Assign {
                            lhs: reg,
                            rhs: Value::Reg(Reg {
                                id: self.state.pop_last(),
                                typ: TypeKind::from_string(&local.typ),
                            })
                        }
                    ))
                };
                self.state.push_instruction(inst);
            },
            ast::StmtKind::Item(_) => {
                //nested item declarations are not supported yet
            },
            ast::StmtKind::Block(_) => {
                //nested blocks are not supported yet
            },
            ast::StmtKind::Expr(_) => {
                walk_stmt(self, s);
            },
        }

    }

    fn visit_expr(&mut self, e: &'ast ast::Expr) {
        match e.kind {
            ast::ExprKind::Ident(ref name) => {
                self.state.push_local(name.to_string());
            },
            ast::ExprKind::Literal(ref val) => {
                let v = Value::Literal(val.to_string());
                let reg = Reg {id: self.state.new_tmp(), typ: literal_type(&val) };
                let inst = Instruction {
                    kind: InstructionKind::Assign(Box::new(Assign {
                        lhs: reg,
                        rhs: v,
                    }))
                };
                self.state.push_instruction(inst);
            },
            ast::ExprKind::Call(ref call) => {
                for expr in &call.params {
                    self.visit_expr(expr);
                }
                let params = self.state.last_n(call.params.len());
                let inst = Instruction {
                    kind: InstructionKind::Call(Box::new(Call {
                        ident: call.ident.to_string(),
                        args: params,
                        res: self.state.new_tmp(),
                    }))
                };
                self.state.push_instruction(inst);
            }
        }
    }
}
extern crate itertools;

use itertools::Itertools;

use super::emmiter::*;
use super::instruction::*;

pub fn print_mod(m: &Module) {
    for f in &m.functions {
        print_fn(f);
        println!();
    }
}

pub fn print_fn(f: &Function) {
    print!("define ");
    if let Some(ref ret) = f.ret {
        print!("{} ", ret);
    }
    print!("@{}", f.name);

    println!("({}) {{", f.params.iter().map(|p| format!("{} ${}", p.typ, p.id)).format(", "));
    for inst in &f.body {
        print_inst(inst);
    }

    print!("}}");
}

pub fn print_inst(inst: &Instruction) {
    match inst.kind {
        InstructionKind::Call(ref call) => {
            print!("    ");
            if let Some(ref res) = call.res {
                print!("${} := call {}", res.id, res.typ);
            } else {
                print!("call");
            }
            print!(" @{} ({})", call.ident,
                   call.args.iter().map(|p| format!("{} ${}", p.typ, p.id)).format(", "));
            println!();
        },
        InstructionKind::Add(..) => {},
        InstructionKind::Sub(..) => {},
        InstructionKind::Mul(..) => {},
        InstructionKind::Div(..) => {},
        InstructionKind::Assign(ref assign) => {
            let lhs = &assign.lhs.id;
            let rhs = match assign.rhs {
                Value::Reg(ref reg) => &reg.id,
                Value::Literal(ref s) => s,
            };
            println!("    ${} := ${}", lhs, rhs);
        },
    }
}
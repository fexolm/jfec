#[macro_use]
extern crate criterion;
extern crate jfec;

use criterion::Criterion;
use jfec::codegen::ssa::emmiter;
use jfec::parser;
use std::fs;

fn bench_create_ast(c: &mut Criterion) {
    let file = fs::read_to_string("testdata/bench.jfec").expect("cannot read file");

    c.bench_function("create ast", |b| b.iter(|| {
        parser::create_ast(&file)
    }));
}

fn bench_create_ssa(c: &mut Criterion) {
    let file = fs::read_to_string("testdata/bench.jfec").expect("cannot read file");
    let ast = parser::create_ast(&file).expect("cannot create ast");

    c.bench_function("create ast", |b| b.iter(|| {
        emmiter::to_ssa(&ast)
    }));
}

criterion_group!(benches, bench_create_ast, bench_create_ssa);

criterion_main!(benches);
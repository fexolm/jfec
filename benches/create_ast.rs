#[macro_use]
extern crate criterion;
extern crate jfec;

use std::fs;

use criterion::Criterion;

use jfec::parser;

fn bench_create_ast(c: &mut Criterion) {
    let file = fs::read_to_string("testdata/bench.jfec").expect("cannot read file");

    c.bench_function("create ast", |b| b.iter(|| {
        parser::create_ast(&file)
    }));
}

criterion_group!(benches, bench_create_ast);
criterion_main!(benches);
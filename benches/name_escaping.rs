use criterion::{black_box, Criterion, criterion_group, criterion_main};
use kotlin_poet_rs::io::RenderKotlin;
use kotlin_poet_rs::spec::Name;

fn name_escaping_escaped() -> String {
    let name = Name::from("Hello, World!");
    return name.render_string();
}

fn name_escaping_unescaped() -> String {
    let name = Name::from("HelloWorld");
    return name.render_string();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("escaped", |b| b.iter(||
    black_box(
        name_escaping_escaped()
    )));

    c.bench_function("unescaped", |b| b.iter(||
    black_box(
        name_escaping_unescaped()
    )));
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}
criterion_main!(benches);
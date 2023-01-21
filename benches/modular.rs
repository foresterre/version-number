use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use version_number::parsers::modular::Parser as ModularParser;
use version_number::parsers::original::Parser as OriginalParser;

fn current_parse(input: &str) {
    let _ = OriginalParser::from_slice(input.as_bytes()).parse();
}

fn modular_parse(input: &str) {
    let _ = ModularParser::from_slice(input.as_bytes()).parse();
}

pub fn parse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parser");

    for input in [
        "1.0.0",
        "0.1.0",
        "0.0.1",
        "1.0",
        "0.1",
        "999.888.777",
        "999.888",
        "18446744073709551615.0",
        "18446744073709551615.0.0",
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::new("current[happy]", input),
            input,
            |b, bench_input| b.iter(|| current_parse(bench_input)),
        );

        group.bench_with_input(
            BenchmarkId::new("modular[happy]", input),
            input,
            |b, bench_input| b.iter(|| modular_parse(bench_input)),
        );
    }

    for input in [
        "00.1.1",
        "00.1",
        "0999.999.999",
        "999.0999.999",
        "999.999.0999",
        "0999.999",
        "999.0999",
        "unexpected",
        "1",
        "1,1",
        "1,1,1",
        "18446744073709551616.0.0",
    ]
    .iter()
    {
        group.bench_with_input(
            BenchmarkId::new("current[sad]", input),
            input,
            |b, bench_input| b.iter(|| current_parse(bench_input)),
        );

        group.bench_with_input(
            BenchmarkId::new("modular[sad]", input),
            input,
            |b, bench_input| b.iter(|| modular_parse(bench_input)),
        );
    }

    group.finish();
}

criterion_group!(benches, parse_benchmark,);
criterion_main!(benches);

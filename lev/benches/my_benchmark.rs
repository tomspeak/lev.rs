use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lev::lev;

fn criterion_benchmark(c: &mut Criterion) {
    let inputs = [
        ("cat", "hat"),
        ("hat", "cat"),
        ("hyundai", "honda"),
        ("Hello, world!", "Goodbye, world!"),
        ("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", "Bacon ipsum dolor amet fatback jowl chicken pork ball tip bacon sirloin, prosciutto hamburger. Venison swine leberkas salami, shank ground round shoulder porchetta doner.")
    ];

    let mut group = c.benchmark_group("LevvyBoi");

    for i in inputs {
        group.bench_with_input(
            BenchmarkId::new("lev", format!("{} x {}", i.0, i.1)),
            &i,
            |b, &i| b.iter(|| lev(black_box(i.0), black_box(i.1))),
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use std::time::{SystemTime, Instant};

use criterion::{criterion_group, criterion_main, Criterion};

fn systemtime_now() {
    let _now = SystemTime::now();
}

fn instant_now() {
    let _new = Instant::now();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("nows");
    group.bench_function("SystemTime::now", |b| b.iter(||
        {
            systemtime_now()
        }
    ));

    group.bench_function("Instant::now", |b| b.iter(||
        {
            instant_now()
        }
    ));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
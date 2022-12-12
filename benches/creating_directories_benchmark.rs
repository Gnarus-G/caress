use std::fs;

use caress::{mkdir, mkdir_alt, mkdir_alt2, test_utils::TestDirs};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const DIR_COUNT: usize = 1000;

fn std_function_benchmark(c: &mut Criterion) {
    c.bench_function("stdlib-function", |b| {
        b.iter(|| {
            let td = TestDirs::new("stdlib-method", DIR_COUNT);

            fs::create_dir_all(&td.path).unwrap();

            td.remove();
        })
    });
}

fn mkdir_benchmark(c: &mut Criterion) {
    c.bench_function("mkdir-function", |b| {
        b.iter(|| {
            let td = TestDirs::new("mkdir-method", DIR_COUNT);

            mkdir(&td.path).unwrap();

            td.remove();
        })
    });
}

fn mkdir_alt_benchmark(c: &mut Criterion) {
    c.bench_function("mkdir_alt-function", |b| {
        b.iter(|| {
            let td = TestDirs::new("mkdir_alt-method", DIR_COUNT);

            mkdir_alt(&mut td.path.clone()).unwrap();

            td.remove();
        })
    });
}

fn mkdir_alt2_benchmark(c: &mut Criterion) {
    c.bench_function("mkdir_alt2-function", |b| {
        b.iter(|| {
            let td = TestDirs::new("mkdir_alt2-method", DIR_COUNT);

            mkdir_alt2(&mut td.path.clone()).unwrap();

            td.remove();
        })
    });
}

criterion_group!(
    benches,
    mkdir_alt2_benchmark,
    mkdir_alt_benchmark,
    mkdir_benchmark,
    std_function_benchmark,
);

criterion_main!(benches);

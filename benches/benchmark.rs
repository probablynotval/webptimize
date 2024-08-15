use std::{env, path::Path};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::tempdir;
use webptimize::convert_to_webp;

fn bench(c: &mut Criterion) {
    let crate_root = env::current_dir().unwrap();
    let mut group = c.benchmark_group("conversions");
    group.sample_size(10);

    group.bench_function("test50", |b| {
        b.iter(|| {
            convert_to_webp(
                black_box(Path::new(&crate_root.join("tests/images/test50"))),
                black_box(Some(tempdir()?.path())),
                black_box(false),
                black_box(80.0),
            )
        })
    });

    group.bench_function("test100", |b| {
        b.iter(|| {
            convert_to_webp(
                black_box(Path::new(&crate_root.join("tests/images/test100"))),
                black_box(Some(tempdir()?.path())),
                black_box(false),
                black_box(80.0),
            )
        })
    });

    group.bench_function("test200", |b| {
        b.iter(|| {
            convert_to_webp(
                black_box(Path::new(&crate_root.join("tests/images/test200"))),
                black_box(Some(tempdir()?.path())),
                black_box(false),
                black_box(80.0),
            )
        })
    });

    group.bench_function("test500", |b| {
        b.iter(|| {
            convert_to_webp(
                black_box(Path::new(&crate_root.join("tests/images/test500"))),
                black_box(Some(tempdir()?.path())),
                black_box(false),
                black_box(80.0),
            )
        })
    });

    group.bench_function("test50-consistent", |b| {
        b.iter(|| {
            convert_to_webp(
                black_box(Path::new(
                    &crate_root.join("tests/images/test50-consistent"),
                )),
                black_box(Some(tempdir()?.path())),
                black_box(false),
                black_box(80.0),
            )
        })
    });

    group.bench_function("test50-inconsistent", |b| {
        b.iter(|| {
            convert_to_webp(
                black_box(Path::new(
                    &crate_root.join("tests/images/test50-inconsistent"),
                )),
                black_box(Some(tempdir()?.path())),
                black_box(false),
                black_box(80.0),
            )
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);

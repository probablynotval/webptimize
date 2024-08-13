use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use webptimize::convert_to_webp;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversions");
    group.sample_size(10);
    group.bench_function("convert", |b| {
        b.iter(|| {
            convert_to_webp(
                black_box(Path::new(
                    "/home/val/Documents/Repos/webptimize/tests/images",
                )),
                black_box(Some(Path::new(
                    "/home/val/Documents/Repos/webptimize/tests/images/webp",
                ))),
                black_box(false),
                black_box(90.0),
            )
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use hydrogen_i18n::metadata::MetadataBuilder;
use tokio::runtime::Builder;

fn bench_metadata(c: &mut Criterion) {
    let mut group = c.benchmark_group("metadata");

    group.bench_function("metadata", |b| {
        b.iter(|| MetadataBuilder::load_dir("tests/data"));
    });

    group.bench_function("tokio_metadata", |b| {
        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

        b.to_async(runtime)
            .iter(|| MetadataBuilder::tokio_load_dir("tests/data"));
    });

    group.finish();
}

criterion_group!(benches, bench_metadata);
criterion_main!(benches);

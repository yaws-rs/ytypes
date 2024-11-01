use criterion::{black_box, criterion_group, criterion_main, Criterion};

use yuri::Uri;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("yuri::Uri New full HTTPs URL", |b| {
        let s = "https://foo:secret@foobar.test:666/?q=a&m=s#fragemnt";

        b.iter(|| {
            let _uri = Uri::new(black_box(s)).expect("Failed to parse URI");
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

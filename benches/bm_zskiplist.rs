use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

use curly_giggle::collection::skiplist::zskiplist::ZSkipList;

fn bm_zskiplist_create(c: &mut Criterion) {
    let mut group = c.benchmark_group("ZSkiplist - create");
    group.measurement_time(Duration::new(10, 0)); // Set measurement time to 10 seconds for more stable results

    group.bench_function("ZSkiplist - create", |b| {
        b.iter(|| {
            let _ = ZSkipList::<bool>::zsl_create();
        })
    });

    group.finish();
}

criterion_group!(create_bench, bm_zskiplist_create);
criterion_main!(create_bench);
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use curly_giggle::sds::SDS;
use std::time::Duration;

fn bm_sds_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("String vs MySDS New");
    group.measurement_time(Duration::new(10, 0)); // Set measurement time to 10 seconds for more stable results

    group.bench_function("String new", |b| {
        b.iter(|| {
            let _ = String::from("Hello");
        })
    });
    
    group.bench_function("MySDS new", |b| {
        b.iter(|| {
            let _ = SDS::sdsnew("Hello");
        })
    });

    group.finish();
}


fn bm_sds_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("String vs MySDS Append");
    group.measurement_time(Duration::new(10, 0)); // Set measurement time to 10 seconds for more stable results

    let s = "Hello, World!";

    group.bench_function("String append", |b| {
        b.iter(|| {
            let mut string = String::from("Hello");
            string.push_str(black_box(s));
        })
    });

    group.bench_function("MySDS append", |b| {
        b.iter(|| {
            let mut sds = SDS::sdsnew("Hello");
            sds.sdscat(black_box(s));
        })
    });

    group.finish();
}

fn bm_sds_len(c: &mut Criterion) {
    let mut group = c.benchmark_group("String vs MySDS len");
    group.measurement_time(Duration::new(10, 0)); // Set measurement time to 10 seconds for more stable results
    
    group.bench_function("String len", |b| {
        b.iter(|| {
            let string = String::from("Hello");
            let _ = string.len();
        })
    });

    group.bench_function("MySDS len", |b| {
        b.iter(|| {
            let sds = SDS::sdsnew("Hello");
            let _ = sds.sdslen();
        })
    });

    group.finish();
}

fn bm_sds_clear(c: &mut Criterion) {
    let mut group = c.benchmark_group("String vs MySDS clear");
    group.measurement_time(Duration::new(10, 0)); // Set measurement time to 10 seconds for more stable results
    
    group.bench_function("String clear", |b| {
        b.iter(|| {
            let mut string = String::from("Hello");
            string.clear();
        })
    });


    group.bench_function("MySDS clear", |b| {
        b.iter(|| {
            let mut sds = SDS::sdsnew("Hello");
            sds.sdsclear();
        })
    });

    group.finish();
}

criterion_group!(new_bench, bm_sds_new);
criterion_group!(append_bench, bm_sds_append);
criterion_group!(len_bench, bm_sds_len);
criterion_group!(clear_bench, bm_sds_clear);
criterion_main!(new_bench,append_bench, len_bench, clear_bench);
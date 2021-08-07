use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use uuid::Uuid;
use uuid_readable_rs::{generate, generate_from, short, short_from};

pub fn main_benches(c: &mut Criterion) {
    c.bench_function("generate", |b| b.iter(|| generate()));
    c.bench_function("short", |b| b.iter(|| short()));
    let uuid = Uuid::new_v4();
    c.bench_function("generate_from", |b| b.iter(|| generate_from(uuid)));
    c.bench_function("short_from", |b| b.iter(|| short_from(uuid)));
}

criterion_group!(benches, main_benches);
criterion_main!(benches);

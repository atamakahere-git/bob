use bob::{
    mutref::{CatMutRefBuilderBorrowTypes, CatMutRefBuilderOwnedTypes},
    owned::{CatOwnedBuilderBorrowTypes, CatOwnedBuilderOwnedTypes},
    DefaultBuilder, RandomBuilder,
};

use criterion::{criterion_group, criterion_main, Criterion};

fn randmutrefowned(c: &mut Criterion) {
    c.bench_function("randmutrefowned", |b| {
        b.iter(CatMutRefBuilderOwnedTypes::random_build)
    });
}

fn randmutrefbrw(c: &mut Criterion) {
    c.bench_function("randmutrefbrw", |b| {
        b.iter(CatMutRefBuilderBorrowTypes::random_build)
    });
}

fn randownedowned(c: &mut Criterion) {
    c.bench_function("randownedowned", |b| {
        b.iter(CatOwnedBuilderOwnedTypes::random_build)
    });
}

fn randownedbrw(c: &mut Criterion) {
    c.bench_function("randownedbrw", |b| {
        b.iter(CatOwnedBuilderBorrowTypes::random_build)
    });
}

fn defmutrefowned(c: &mut Criterion) {
    c.bench_function("defmutrefowned", |b| {
        b.iter(CatMutRefBuilderOwnedTypes::default_build)
    });
}

fn defmutrefbrw(c: &mut Criterion) {
    c.bench_function("defmutrefbrw", |b| {
        b.iter(CatMutRefBuilderBorrowTypes::default_build)
    });
}

fn defownedowned(c: &mut Criterion) {
    c.bench_function("defownedowned", |b| {
        b.iter(CatOwnedBuilderOwnedTypes::default_build)
    });
}

fn defownedbrw(c: &mut Criterion) {
    c.bench_function("defownedbrw", |b| {
        b.iter(CatOwnedBuilderBorrowTypes::default_build)
    });
}

criterion_group!(
    rand,
    randmutrefowned,
    randmutrefbrw,
    randownedowned,
    randownedbrw
);

criterion_group!(
    def,
    defmutrefowned,
    defmutrefbrw,
    defownedowned,
    defownedbrw
);
criterion_main!(rand, def);

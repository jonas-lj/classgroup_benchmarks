use criterion::criterion_main;
use std::ops::{Mul, MulAssign};

use classgroup::gmp::mpz::Mpz;
use classgroup::{gmp_classgroup::GmpClassGroup, ClassGroup};
use classgroup_benchmarks::*;
use criterion::{criterion_group, BenchmarkGroup, Criterion};
use std::str::FromStr;

fn benchmark(c: &mut Criterion) {
    let mut group: BenchmarkGroup<_> = c.benchmark_group("classgroup");

    let d = -Mpz::from_str(D_512).unwrap();
    let x = GmpClassGroup::from_ab_discriminant(
        Mpz::from_str(A1_512).unwrap(),
        Mpz::from_str(B1_512).unwrap(),
        d.clone(),
    );
    let y = GmpClassGroup::from_ab_discriminant(
        Mpz::from_str(A1_512).unwrap(),
        Mpz::from_str(B1_512).unwrap(),
        d.clone(),
    );
    let mut z = y.clone();
    group.bench_function("compose/512", move |b| {
        b.iter(|| {
            let mut res = x.clone().mul(&y);
            res.reduce()
        })
    });
    group.bench_function("double/512", move |b| {
        b.iter(|| {
            z.square();
            z.reduce()
        })
    });

    let d = -Mpz::from_str(D_1024).unwrap();
    let x = GmpClassGroup::from_ab_discriminant(
        Mpz::from_str(A1_1024).unwrap(),
        Mpz::from_str(B1_1024).unwrap(),
        d.clone(),
    );
    let y = GmpClassGroup::from_ab_discriminant(
        Mpz::from_str(A1_1024).unwrap(),
        Mpz::from_str(B1_1024).unwrap(),
        d.clone(),
    );
    let mut z = y.clone();
    group.bench_function("compose/1024", move |b| {
        b.iter(|| {
            let mut res = x.clone().mul(&y);
            res.reduce()
        })
    });
    group.bench_function("double/1024", move |b| {
        b.iter(|| {
            z.square();
            z.reduce()
        })
    });
}

criterion_group! {
    name = classgroup;
    config = Criterion::default().sample_size(100);
    targets =
        benchmark,
}

criterion_main!(classgroup);

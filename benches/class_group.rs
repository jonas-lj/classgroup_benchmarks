use class_group::pari_init;
use class_group::ABDeltaTriple;
use class_group::BinaryQF;
use classgroup_benchmarks::*;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};
use curv::BigInt;
use num_traits::Num;

fn benchmark(c: &mut Criterion) {
    unsafe {
        pari_init(100000000000, 0);
    }

    let mut group: BenchmarkGroup<_> = c.benchmark_group("class_group");

    let x = BinaryQF::binary_quadratic_form_disc(&ABDeltaTriple {
        a: BigInt::from_str_radix(A1_512, 10).unwrap(),
        b: BigInt::from_str_radix(B1_512, 10).unwrap(),
        delta: BigInt::from_str_radix(D_512, 10).unwrap(),
    });

    let y = BinaryQF::binary_quadratic_form_disc(&ABDeltaTriple {
        a: BigInt::from_str_radix(A2_512, 10).unwrap(),
        b: BigInt::from_str_radix(B2_512, 10).unwrap(),
        delta: BigInt::from_str_radix(D_512, 10).unwrap(),
    });

    group.bench_function("compose/512", move |b| b.iter(|| x.compose(&y).reduce()));

    let x = BinaryQF::binary_quadratic_form_disc(&ABDeltaTriple {
        a: BigInt::from_str_radix(A1_1024, 10).unwrap(),
        b: BigInt::from_str_radix(B1_1024, 10).unwrap(),
        delta: BigInt::from_str_radix(D_1024, 10).unwrap(),
    });

    let y = BinaryQF::binary_quadratic_form_disc(&ABDeltaTriple {
        a: BigInt::from_str_radix(A2_1024, 10).unwrap(),
        b: BigInt::from_str_radix(B2_1024, 10).unwrap(),
        delta: BigInt::from_str_radix(D_1024, 10).unwrap(),
    });

    group.bench_function("compose/1024", move |b| b.iter(|| x.compose(&y).reduce()));
}

criterion_group! {
    name = class_group;
    config = Criterion::default().sample_size(100);
    targets =
        benchmark,
}

criterion_main!(class_group);

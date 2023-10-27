use criterion::criterion_main;

use criterion::{criterion_group, BenchmarkGroup, Criterion};
use fastcrypto_vdf::class_group::{Discriminant, QuadraticForm};
use fastcrypto_vdf::ParameterizedGroupElement;
use num_bigint::BigInt;
use num_traits::Num;
use rug::integer::Order;
use rug::Integer;

use classgroup_benchmarks::*;

fn bigint_to_integer(value: &BigInt) -> Integer {
    Integer::from_digits(&value.to_bytes_be().1, Order::Msf)
}

fn benchmark(c: &mut Criterion) {
    let mut group: BenchmarkGroup<_> = c.benchmark_group("fastcrypto");

    let d = Discriminant::try_from(BigInt::from_str_radix(D_512, 10).unwrap()).unwrap();
    let x = QuadraticForm::from_a_b_discriminant(
        bigint_to_integer(&BigInt::from_str_radix(A1_512, 10).unwrap()),
        bigint_to_integer(&BigInt::from_str_radix(B1_512, 10).unwrap()),
        &d,
    );
    let y = QuadraticForm::from_a_b_discriminant(
        bigint_to_integer(&BigInt::from_str_radix(A2_512, 10).unwrap()),
        bigint_to_integer(&BigInt::from_str_radix(B2_512, 10).unwrap()),
        &d,
    );
    let z = y.clone();

    group.bench_function("compose/512", move |b| b.iter(|| x.compose(&y)));
    group.bench_function("double/512", move |b| b.iter(|| z.clone().double()));

    let d = Discriminant::try_from(BigInt::from_str_radix(D_1024, 10).unwrap()).unwrap();
    let x = QuadraticForm::from_a_b_discriminant(
        bigint_to_integer(&BigInt::from_str_radix(A1_1024, 10).unwrap()),
        bigint_to_integer(&BigInt::from_str_radix(B1_1024, 10).unwrap()),
        &d,
    );
    let y = QuadraticForm::from_a_b_discriminant(
        bigint_to_integer(&BigInt::from_str_radix(B1_1024, 10).unwrap()),
        bigint_to_integer(&BigInt::from_str_radix(B2_1024, 10).unwrap()),
        &d,
    );
    let z = y.clone();
    group.bench_function("compose/1024", move |b| b.iter(|| x.compose(&y)));
    group.bench_function("double/1024", move |b| b.iter(|| z.clone().double()));
}

criterion_group! {
    name = fastcrypto;
    config = Criterion::default().sample_size(100);
    targets =
        benchmark,
}

criterion_main!(fastcrypto);

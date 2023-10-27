## Class group benchmarks

This repository contains benchmarks of all Rust implementations of imaginary ideal class groups that I could find.

The benchmarks include group composition in groups with 512 and 1024 bit discriminants respectively. Some libraries have doubling 
operations which are slightly faster, but only composition is benchmarked here.

The crates benchmarked are the `fastcrypto`, `class_group` and `classgroup` crates. 
All three depend on the [GMP library](https://gmplib.org/) for integer arithmetic, so this has to be installed on your system. 

I was not able to run the [`classygroup`](https://docs.rs/classygroup/latest/classygroup/) crate, but will be happy to 
add that if someone can help me get to work.

Run `cargo bench` to run all benchmarks.

### Benhcmarks

On my system (MacBook Pro, M1 Pro, 16 GB RAM, 8 cores (6 performance and 2 efficiency)) I get the following results:

| Crate                  | Version                                                                                            | 512 bit   | 1024 bit  | Comments                                             |
|------------------------|----------------------------------------------------------------------------------------------------|-----------|-----------|------------------------------------------------------|
| fastcrypto             | [9f507d](https://github.com/MystenLabs/fastcrypto/commit/9f507de7857aa44fa430267da3ff6973dec24b48) | 5.7556 µs | 11.771 µs | Uses the rug crate (GMP) for all integer arithmetic. |
| class_group (with gmp) | [0.6.1](https://crates.io/crates/class_group/0.6.1)                                                | 60.089 µs | 120.68 µs | Uses pari-gp for composition and GMP for reduction.  |
| classgroup             | [0.1.0](https://crates.io/crates/classgroup/0.1.0)                                                 | 9.3262 µs | 21.345 µs | Uses GMP (C++) binding for all integer arithmetic.   | 
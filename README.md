## Class group benchmarks

This repository contains benchmarks of all Rust implementations of ideal class groups that I could find.

The benchmarks include group composition in groups with 512 and 1024 bit discriminants respectively.

The crates benchmarked are the `fastcrypto`, `class_group` and `classgroup` crates. 
The latter two depend on the [GMP library](https://gmplib.org/), so this has to be installed on your system. 

I was not able to run the `classygroup` crate, but will be happy to add that if someone can help me get to work.

Run `cargo build` to run all benchmarks.

### Results

On my system (MacBook Pro, M1 Pro, 16 GB RAM, 8 cores (6 performance and 2 efficiency)) I get the following results:

| Crate                  | Version                                                                                            | 512 bit discriminant | 1024 bit discriminant | Comments                                                         |
|------------------------|----------------------------------------------------------------------------------------------------|----------------------|-----------------------|------------------------------------------------------------------|
| fastcrypto             | [da86fb](https://github.com/MystenLabs/fastcrypto/commit/da86fb467d9d7304a668e3b02bef3ace05b85b36) | 43.17 µs             | 94.792 µs             | Pure rust. Depends on num-bigint for arithmetic.                 |
| class_group (with gmp) | 0.6.1                                                                                              | 60.212 µs            | 123.8 µs              | pari-gp for composition and GMP for reduction.                   |
| classgroup             | 0.1.0                                                                                              | 9.6176 µs            | 21.827 µs             | Uses gmp (C++) binding for arithmetic. Only squaring was tested. | 
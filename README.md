# Project Euler
This repository contains my solutions to the first 100 problems of [Project Euler](https://projecteuler.net/).


## Prerequisites
To run the solutions, you'll need:
- [Rust](https://rust-lang.org/) installed (using [rustup](https://rustup.rs/) is recommended)
- `cargo`, which comes bundled with Rust.


## Usage
To solve a specific problem, you can run the program with the `solve` command followed by the problem number:
```sh
$ cargo run --release -- solve n
```

To solve all problems sequentially, use the `all` command:
```sh
$ cargo run --release -- all
```


### Running Tests
Each problem has an associated unit test to verify correctness. You can run the test for a specific problem by passing
`problem_xyz`, padding with zeros to make the problem number three digits:
```sh
$ cargo test -- problem_001
```

Alternatively, you can test all solutions:
```sh
$ cargo test
```


### Running Benchmarks
Benchmarking is handled using [Criterion](https://github.com/bheisler/criterion.rs). When benchmarking the solution to
a specific problem, make sure to pad the problem number with zeros, then run:
```sh
$ cargo bench -- problem_001
```

You can also benchmark all solutions, however this typically takes a while:
```sh
$ cargo bench
```


## Project Structure
The repository is structured as follows:
```
.
├── benches                 # Benchmarking setup
├── resources               # Problem-specific resources
├── src
│   ├── math                # Library with common math operations
│   ├── problems            # Every problem has its own file
│   ├── lib.rs              # Core logic, accessible for tests and benchmarks
│   └── main.rs             # Entry point, delegates to lib
├── tests                   # Unit test for every problem
├── build.rs                # Generates module listings and benchmarking setup
├── Cargo.toml              # Rust project configuration
├── LICENSE
└── README.md               # This file
```


## License
This project is licensed under the BSD 3-Clause License. See [LICENSE](./LICENSE) for details.


## Disclaimer
This repository contains solutions for the first 100 problems of Project Euler. It may spoil any fun solving them
yourself. Keep in mind that knowing the solution cannot be reversed! If you have any intents of solving the problems
for yourself, consider this your suggestion to avoid looking!

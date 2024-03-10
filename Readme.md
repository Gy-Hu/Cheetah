# Cheetah

## Algorithm Overview

This model checker is mainly based on Bounded Model Checking (BMC) algorithm. The algorithm is based on the following steps:

1. Initialize the model with initial values for all variables.
2. Unroll the transition relation for a fixed number of steps.
3. For each step, using bitwuzla to check if the property is satisfied.
4. If the property is not satisfied, return a counterexample.
5. If the property is satisfied, return a satisfying assignment.

Bitwuzla is a SOTA SMT solver that can handle complex formulas and is used in this project.

## Usage

Enter core, and run the following command:

1. `rustup target add x86_64-unknown-linux-musl`
2. `cargo build --release --target x86_64-unknown-linux-musl --examples`

Now, you can run the engine by executing the following command:
1. `ldd ./target/x86_64-unknown-linux-musl/release/examples/bmc`  - for checking the dynamic library dependencies.
2. `./target/x86_64-unknown-linux-musl/release/examples/bmc xxx/xxx/xxx.btor2`

## Prerequisites

1. Bitwuzla SMT solver - https://bitwuzla.github.io/
2. Rust environment (For rust dependencies, see [Cargo.toml](core/Cargo.toml))

## Acknowledgments

This project is a new version based on the work done by Kevin Laeufer in the project [Patron](https://github.com/ekiwi/patron). We extend our gratitude to Kevin Laeufer for the original creation and for making it available under the BSD 3-Clause License.

## Original Project

The original project, Patron, can be found here: https://github.com/ekiwi/patron. It was developed by Kevin Laeufer and is licensed under the BSD 3-Clause License. Our work is inspired by and built upon the foundation laid by Kevin Laeufer's Patron.

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE.md](core/LICENSE) file for details. The original work by Kevin Laeufer, upon which this project is based, is also licensed under the BSD 3-Clause License. Full credit goes to Kevin Laeufer for the original idea and implementation.

## Authors

- **Gary Hu** - *Initial work on the new version*

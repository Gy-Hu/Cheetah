# Cheetah

This is a parallel model checker for multiple properties in the Btor2 format. It is based on the Bounded Model Checking (BMC) algorithm. It applys dynamic unrolling to adjust the number of unrolling bounds to checke properties of different difficulties.

## Algorithm Overview

This model checker is mainly based on Bounded Model Checking (BMC) algorithm (you can check the original paper [here](https://www.cs.cmu.edu/~emc/papers/Books%20and%20Edited%20Volumes/Bounded%20Model%20Checking.pdf)). The algorithm is based on the following steps:

1. Initialize the model with initial values for all variables.
2. Unroll the transition relation for a fixed number of steps.
3. For each step, using bitwuzla to check if the property is satisfied.
4. If the property is not satisfied, return a counterexample.
5. If the property is satisfied, return a satisfying assignment.

Bitwuzla is a SOTA SMT solver that can handle complex formulas and is used in this project.

## Usage

Enter core, and run the following command (assuming you have Rust environment and Bitwuzla installed):

1. `rustup target add x86_64-unknown-linux-musl`
2. `cargo build --release --target x86_64-unknown-linux-musl --examples`

Now, you can run the engine by executing the following command:
1. `ldd ./target/x86_64-unknown-linux-musl/release/examples/cheetah`  - for checking the dynamic library dependencies.
2. `./target/x86_64-unknown-linux-musl/release/examples/cheetah xxx/xxx/xxx.btor2`

This compiled binary can be ran on any Linux system with the Bitwuzla SMT solver installed.

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

- **Gary Hu** - *Extend patron to Cheetah, support multiple properties and parallel running*

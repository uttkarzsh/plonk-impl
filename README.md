# PLONK Implementation in Rust

This is a PLONK implementation in Rust. The algorithm follows the algorithm end-to-end.
This protocol is arithmetic heavy as compared to its [Groth16 counterpart.](https://github.com/uttkarzsh/groth16-impl)
I have kept this implementation, very low-level and explicit, so that the code can be directly compared to its description in [Section 8.3 of the PLONK Paper.](https://eprint.iacr.org/2019/953.pdf)


## How to use it
 - Clone the repository
 - Define the curve to be used in `types.rs`
 - Define the constraint system in `constants.rs` (the selector polynomials, and the permutation polynomials)
 - Enter your witness in `witness.rs`
 - `cargo run`

#![allow(unused)]

mod constants;
mod proof;
mod verifier;
mod trusted_setup;
mod witness;
mod utils;

use ark_bn254::{Fr};
use proof::{Proof};
use verifier::{verify_proof};
use constants::{L};

fn main() {
    // let pub_inputs: [Fr; L] = [];
    // let proof: Proof = Proof::generate_proof();

    // let verification_successful: bool = verify_proof(&proof, &pub_inputs);

    // if verification_successful {
    //     println!("witness correct yay");
    // } else {
    //     println!("wrong witness lol");
    // }
}


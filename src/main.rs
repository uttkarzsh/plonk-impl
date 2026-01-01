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
use witness::{WITNESS};
use constants::{L};

fn main() {
    let pub_inputs: [Fr; L] = [Fr::from(-4i32)];
    let proof: Proof = Proof::generate_proof(&WITNESS, &pub_inputs);

    let verification_successful: bool = verify_proof(&proof, &pub_inputs);

    if verification_successful {
        println!("witness correct yay");
    } else {
        println!("wrong witness lol");
    }
}


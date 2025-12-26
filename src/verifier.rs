use ark_bn254::{Fr};
use crate::constants::{L};
use crate::proof::{Proof};

pub fn verify_proof(proof: &Proof, pub_inputs: &[Fr; L]) -> bool {
    true
}
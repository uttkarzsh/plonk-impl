use ark_ff::{Field as IField, UniformRand};
use crate::utils::curve_ops::{G1, G2};
use crate::constants::{N};
use crate::types::*;
use std::sync::LazyLock;
use rand::thread_rng;

pub struct SRS{
    pub ptau_g1: [G1Point; N+6],
    pub ptau_g2: [G2Point; 2],
}

impl SRS{
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Field = Field::rand(&mut rng);

        let mut ptau_g1: [G1Point; N+6] = [*G1; N+6];
        for i in 0..N+6 {
            ptau_g1[i] = *G1 * tau.pow([i as u64]);
        }

        let ptau_g2: [G2Point; 2] = [*G2, *G2 * tau];

        Self { ptau_g1, ptau_g2 }
    }
}

pub static GENERATED_SRS: LazyLock<SRS> = LazyLock::new(|| SRS::new());
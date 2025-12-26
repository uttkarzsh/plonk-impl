use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ff::{Field, UniformRand};
use crate::utils::curve_ops::{G1, G2};
use crate::constants::{N};
use std::sync::LazyLock;
use rand::thread_rng;

pub struct SRS{
    pub ptau_g1: [G1Projective; N],
    pub ptau_g2: [G2Projective; 2],
}

impl SRS{
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let tau: Fr = Fr::rand(&mut rng);

        let mut ptau_g1: [G1Projective; N] = [*G1; N];
        for i in 0..N {
            ptau_g1[i] = *G1 * tau.pow([i as u64]);
        }

        let ptau_g2: [G2Projective; 2] = [*G2, *G2 * tau];

        Self { ptau_g1, ptau_g2 }
    }
}

pub static GENERATED_SRS: LazyLock<SRS> = LazyLock::new(|| SRS::new());
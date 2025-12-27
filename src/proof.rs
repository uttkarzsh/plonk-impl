use ark_bn254::{Fr, G1Projective};
use ark_ff::{FftField, UniformRand};
use rand::thread_rng;
use crate::constants::*;
use crate::trusted_setup::{GENERATED_SRS};
use crate::utils::fiat_shamir::{FiatShamir};

pub struct Proof {
    // pub a_commitment: G1Projective,
    // pub b_commitment: G1Projective,
    // pub c_commitment: G1Projective,
    // pub z_commitment: G1Projective,
    // pub t_lo_commitment: G1Projective,
    // pub t_mid_commitment: G1Projective,
    // pub t_hi_commitment: G1Projective,
    // pub w_zeta_commitment: G1Projective,
    // pub w_zeta_omega_commitment: G1Projective,
    // pub a_opened: Fr,
    // pub b_opened: Fr,
    // pub c_opened: Fr,
    // pub s1_opened: Fr,
    // pub s2_opened: Fr,
    // pub z_omega_opened: Fr
}

impl Proof {
    pub fn generate_proof(WITNESS: &[Fr; 3*N], pub_inputs: &[Fr; L]) -> Self {
        let mut rng = thread_rng();
        let transcript: FiatShamir = FiatShamir::new();     //transcript for fiat shamir
        let omega: Fr = Fr::get_root_of_unity(N as u64).unwrap();   //nth root of unity

        //blinding scalars
        let mut b: [Fr; 9] = [Fr::from(1u64); 9];
        for i in 0..9{
            b[i] = Fr::rand(&mut rng);
        }

        //Round 1
        //Round 2
        //Round 3
        //Round 4
        //Round 5

        Self {}
    }
}
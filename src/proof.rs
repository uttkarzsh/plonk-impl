use ark_bn254::{Fr, G1Projective};
use ark_ff::{FftField, UniformRand};
use rand::thread_rng;
use crate::constants::*;
use crate::trusted_setup::{GENERATED_SRS};
use crate::utils::fiat_shamir::{FiatShamir};
use crate::utils::math::*;
use crate::utils::curve_ops::*;
use crate::witness::{Witness};

pub struct Proof {
    pub a_commitment: G1Projective,
    pub b_commitment: G1Projective,
    pub c_commitment: G1Projective,
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
    pub fn generate_proof(witness: &Witness, pub_inputs: &[Fr; L]) -> Self {
        let mut rng = thread_rng();
        let mut transcript: FiatShamir = FiatShamir::new();     //transcript for fiat shamir
           //nth root of unity

        //blinding scalars
        let mut b: [Fr; 9] = [Fr::from(1u64); 9];
        for i in 0..9{
            b[i] = Fr::rand(&mut rng);
        }

        ///Round 1
        let a_blind_zh: [Fr; N+2] = polynomial_multiplication(&[b[1], b[0]], &ZH_X);
        let b_blind_zh: [Fr; N+2] = polynomial_multiplication(&[b[3], b[2]], &ZH_X);
        let c_blind_zh: [Fr; N+2] = polynomial_multiplication(&[b[5], b[4]], &ZH_X);

        let ax: [Fr; N+2] = add(&a_blind_zh, &lagrange_interpolation(&DOMAIN, &witness.a_x));
        let bx: [Fr; N+2] = add(&b_blind_zh, &lagrange_interpolation(&DOMAIN, &witness.b_x));
        let cx: [Fr; N+2] = add(&c_blind_zh, &lagrange_interpolation(&DOMAIN, &witness.c_x));

        let a_commitment: G1Projective = sum_g1_array(&hadamard_g1(&GENERATED_SRS.ptau_g1, &ax));
        let b_commitment: G1Projective = sum_g1_array(&hadamard_g1(&GENERATED_SRS.ptau_g1, &bx));
        let c_commitment: G1Projective = sum_g1_array(&hadamard_g1(&GENERATED_SRS.ptau_g1, &cx));

        transcript.append_g1(&a_commitment);
        transcript.append_g1(&b_commitment);
        transcript.append_g1(&c_commitment);


        ///Round 2
        let beta: Fr = transcript.challenge();
        let gamma: Fr = transcript.challenge();



        ///Round 3
        let alpha: Fr = transcript.challenge();

        ///Round 4
        let zeta: Fr = transcript.challenge();

        ///Round 5
        let v: Fr = transcript.challenge();

        Self { a_commitment, b_commitment, c_commitment}
    }
}
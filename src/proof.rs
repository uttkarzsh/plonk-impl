use ark_bn254::{Fr, G1Projective};
use ark_ff::{FftField, UniformRand, Zero};
use rand::thread_rng;
use crate::constants::*;
use crate::trusted_setup::{GENERATED_SRS};
use crate::utils::fiat_shamir::{FiatShamir};
use crate::utils::math::*;
use crate::utils::curve_ops::*;
use crate::utils::proof_gen_utils::*;
use crate::witness::{Witness};

pub struct Proof {
    pub a_commitment: G1Projective,
    pub b_commitment: G1Projective,
    pub c_commitment: G1Projective,
    pub z_commitment: G1Projective,
    pub t_lo_commitment: G1Projective,
    pub t_mid_commitment: G1Projective,
    pub t_hi_commitment: G1Projective,
    pub w_zeta_commitment: G1Projective,
    pub w_zeta_omega_commitment: G1Projective,
    pub a_zeta: Fr,
    pub b_zeta: Fr,
    pub c_zeta: Fr,
    pub s1_zeta: Fr,
    pub s2_zeta: Fr,
    pub z_omega_zeta: Fr
}

impl Proof {
    pub fn generate_proof(witness: &Witness, pub_inputs: &[Fr; L]) -> Self {
        let mut rng = thread_rng();
        let mut transcript: FiatShamir = FiatShamir::new();     //transcript for fiat shamir

        //blinding scalars
        let mut b: [Fr; 9] = [Fr::from(1u64); 9];
        for i in 0..9{
            b[i] = Fr::rand(&mut rng);
        }

        let mut pis: [Fr; N] = [Fr::zero(); N];
        for i in 0..L {
            pis[i] = pub_inputs[i];
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

        let blind_zh: [Fr; N+3] = polynomial_multiplication(&[b[8], b[7], b[6]], &ZH_X);
        let zx: [Fr; N+3] = add(&blind_zh, &get_permutation_polynomial(&witness, beta, gamma));

        let z_commitment: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &zx);

        transcript.append_g1(&z_commitment);   


        ///Round 3
        let alpha: Fr = transcript.challenge();
        let pi_x: [Fr; N] = lagrange_interpolation(&DOMAIN, &pis);

        let arithmetic_constraint_poly: [Fr; 3*N + 2] = get_arithmetic_constraint_poly(&ax, &bx, &cx, &pi_x);
        let permutation_constraint_poly: [Fr; 4*N + 6] = get_permutation_constraint_polynomial(alpha, beta, gamma, &ax, &bx, &cx, &zx);
        let boundary_constraint_poly: [Fr; 2*N + 2] = get_boundary_constraint_poly(alpha, &zx);

        let tx_zhx: [Fr; 4*N + 6] = add_three_poly(&permutation_constraint_poly, &arithmetic_constraint_poly, &boundary_constraint_poly);

        let tx: [Fr; 3*N + 6] = polynomial_division(&tx_zhx, &ZH_X);

        let mut t_lo: [Fr; N] = [Fr::zero(); N];
        let mut t_mid: [Fr; N] = [Fr::zero(); N];
        let mut t_hi: [Fr; N+6] = [Fr::zero(); N+6];

        for i in 0..N {
            t_lo[i] = tx[i];
            t_mid[i] = tx[N + i];
            t_hi[i] = tx[2*N + i];
        }
        for i in 0..6 {
            t_hi[N + i] = tx[3*N + i];
        }

        let t_lo_commitment: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &t_lo); 
        let t_mid_commitment: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &t_mid); 
        let t_hi_commitment: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &t_hi);

        transcript.append_g1(&t_lo_commitment);
        transcript.append_g1(&t_mid_commitment);
        transcript.append_g1(&t_hi_commitment);


        ///Round 4
        let zeta: Fr = transcript.challenge();

        let a_zeta: Fr = evaluate_polynomial(&ax, zeta);
        let b_zeta: Fr = evaluate_polynomial(&bx, zeta);
        let c_zeta: Fr = evaluate_polynomial(&cx, zeta);
        let s1_zeta: Fr = evaluate_polynomial(&S_A, zeta);
        let s2_zeta: Fr = evaluate_polynomial(&S_B, zeta);
        let z_omega_zeta: Fr = evaluate_polynomial(&zx, zeta * *OMEGA);

        transcript.append_fr(&a_zeta);
        transcript.append_fr(&b_zeta);
        transcript.append_fr(&c_zeta);
        transcript.append_fr(&s1_zeta);
        transcript.append_fr(&s2_zeta);
        transcript.append_fr(&z_omega_zeta);


        ///Round 5
        let v: Fr = transcript.challenge();
        let rx: [Fr; N+6] = get_linearisation_poly(a_zeta, b_zeta, c_zeta, alpha, beta, gamma, zeta, z_omega_zeta, s1_zeta, s2_zeta, &zx, &pi_x, &t_lo, &t_mid, &t_hi);

        let w_zeta_x: [Fr; N+5] = get_opening_proof_poly_wz(&rx, &ax, &bx, &cx, zeta, a_zeta, b_zeta, c_zeta, s1_zeta, s2_zeta, v);
        let w_zeta_omega_x : [Fr; N+2] = get_opening_proof_poly_wzomega(&zx, zeta, z_omega_zeta);

        let w_zeta_commitment: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &w_zeta_x);
        let w_zeta_omega_commitment: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &w_zeta_omega_x);; 

        transcript.append_g1(&w_zeta_commitment);
        transcript.append_g1(&w_zeta_omega_commitment);



        //Final proof struct
        Self { a_commitment, b_commitment, c_commitment, z_commitment, t_lo_commitment, t_mid_commitment, t_hi_commitment, w_zeta_commitment, w_zeta_omega_commitment, a_zeta, b_zeta, c_zeta, s1_zeta, s2_zeta, z_omega_zeta }
    }
}
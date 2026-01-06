use ark_bn254::{Fr, G1Projective, G2Projective, Bn254};
use ark_ff::{Field, Zero};
use ark_ec::{pairing::Pairing};
use crate::constants::*;
use crate::proof::{Proof};
use crate::trusted_setup::{GENERATED_SRS};
use crate::utils::fiat_shamir::{FiatShamir};
use crate::utils::curve_ops::{evaluate_commitment, G1, G2};
use crate::utils::math::*;

pub fn verify_proof(proof: &Proof, pub_inputs: &[Fr; L]) -> bool {
    //compute challenges
    let mut transcript: FiatShamir = FiatShamir::new();     //transcript for fiat shamir
    transcript.append_g1(&proof.a_commitment);
    transcript.append_g1(&proof.b_commitment);
    transcript.append_g1(&proof.c_commitment);
    let beta: Fr = transcript.challenge();
    let gamma: Fr = transcript.challenge();
    transcript.append_g1(&proof.z_commitment);
    let alpha: Fr = transcript.challenge();
    transcript.append_g1(&proof.t_lo_commitment);
    transcript.append_g1(&proof.t_mid_commitment);
    transcript.append_g1(&proof.t_hi_commitment);
    let zeta: Fr = transcript.challenge();
    transcript.append_fr(&proof.a_zeta);
    transcript.append_fr(&proof.b_zeta);
    transcript.append_fr(&proof.c_zeta);
    transcript.append_fr(&proof.s1_zeta);
    transcript.append_fr(&proof.s2_zeta);
    transcript.append_fr(&proof.z_omega_zeta);
    let v: Fr = transcript.challenge();
    transcript.append_g1(&proof.w_zeta_commitment);
    transcript.append_g1(&proof.w_zeta_omega_commitment);
    let u : Fr = transcript.challenge();

    //constants generation
    let zh_zeta: Fr = evaluate_polynomial(&ZH_X, zeta);
    let l1_zeta: Fr = evaluate_polynomial(&lagrange_poly(0, &DOMAIN), zeta);

    let mut pis: [Fr; N] = [Fr::zero(); N];     //full public input polynomial
    for i in 0..L {
        pis[i] = pub_inputs[i];
    }

    let pi_zeta: Fr = evaluate_polynomial(&lagrange_interpolation(&DOMAIN, &pis), zeta);

    //G1 elements
    let q_m_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &Q_M);
    let q_l_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &Q_L);
    let q_r_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &Q_R);
    let q_o_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &Q_O);
    let q_c_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &Q_C);
    let s_a_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &S_A);
    let s_b_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &S_B);
    let s_c_g1: G1Projective = evaluate_commitment(&GENERATED_SRS.ptau_g1, &S_C);


    //r(x)'s constant term 
    let r_0: Fr = pi_zeta - l1_zeta * alpha * alpha - alpha * (proof.a_zeta + proof.s1_zeta * beta + gamma) * (proof.b_zeta + proof.s2_zeta * beta + gamma) * (proof.c_zeta + gamma) * proof.z_omega_zeta;

    // [D]_1 = 
    let d_1: G1Projective = 
        q_m_g1 * (proof.a_zeta * proof.b_zeta) + 
        q_l_g1 * proof.a_zeta + 
        q_r_g1 * proof.b_zeta +
        q_o_g1 * proof.c_zeta +
        q_c_g1 +
        proof.z_commitment * ((proof.a_zeta + beta * zeta + gamma) * (proof.b_zeta + beta * zeta * *K1 + gamma) * (proof.c_zeta + beta * zeta * *K2 + gamma) * alpha + l1_zeta * alpha * alpha + u) - 
        s_c_g1 * ((proof.a_zeta + beta * proof.s1_zeta + gamma) * (proof.b_zeta + beta * proof.s2_zeta + gamma) * alpha * beta * proof.z_omega_zeta) - 
        proof.t_lo_commitment * zh_zeta -
        proof.t_mid_commitment * zh_zeta * zeta.pow([N as u64]) -
        proof.t_hi_commitment * zh_zeta * zeta.pow([2*N as u64]);

    // [F]_1
    let f_1: G1Projective = 
        d_1 + proof.a_commitment * v.pow([1]) + proof.b_commitment * v.pow([2]) + proof.c_commitment * v.pow([3]) + s_a_g1 * v.pow([4]) + s_b_g1 * v.pow([5]);

    // [E]_1
    let e_1: G1Projective = 
        *G1 * (v.pow([1]) * proof.a_zeta + v.pow([2]) * proof.b_zeta + v.pow([3]) * proof.c_zeta + v.pow([4]) * proof.s1_zeta + v.pow([5]) * proof.s2_zeta + proof.z_omega_zeta * u - r_0);


    let lhs = Bn254::pairing(proof.w_zeta_commitment + proof.w_zeta_omega_commitment * u, GENERATED_SRS.ptau_g2[1]);
    let rhs = Bn254::pairing(proof.w_zeta_commitment * zeta + proof.w_zeta_omega_commitment * (u*zeta**OMEGA) + f_1 - e_1, GENERATED_SRS.ptau_g2[0]);

    lhs == rhs
}
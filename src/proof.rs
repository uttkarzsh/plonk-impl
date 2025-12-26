use ark_bn254::{Fr, G1Projective};
use crate::constants::{N};

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
    pub a_opened: Fr,
    pub b_opened: Fr,
    pub c_opened: Fr,
    pub s1_opened: Fr,
    pub s2_opened: Fr,
    pub z_omega_opened: Fr
}

impl Proof {
    pub fn generate_proof() -> Self {
        let transcript: FiatShamir = FiatShamir::new();
        let omega: Fr = Fr::get_root_of_unity(N).unwrap();
    }
}
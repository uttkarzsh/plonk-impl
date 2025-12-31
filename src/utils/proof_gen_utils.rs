use ark_bn254::{Fr};
use ark_ff::{Field, Zero, One};
use crate::utils::math::*;
use crate::constants::*;
use crate::witness::{Witness};

//z(x)for round 2 of proof generation
pub fn get_permutation_polynomial(witness: &Witness, beta: Fr, gamma: Fr) -> [Fr; N] {
    let l1x: [Fr; N] = lagrange_poly(0, &DOMAIN);

    let mut lox: [Fr; N] = [Fr::zero(); N];

    for i in 1..N{
        let lx:[Fr; N] = lagrange_poly(i, &DOMAIN);


        let mut f: Fr = Fr::one();
        for j in 0..i+1 {
            let numer: Fr = (witness.a_x[j] + beta * DOMAIN[j] + gamma) * (witness.b_x[j] + beta * DOMAIN[j] * *K1 + gamma) * (witness.c_x[j] + beta * DOMAIN[j] * *K2 + gamma);
            let denom: Fr = (witness.a_x[j] + beta * evaluate_polynomial(&S_A, DOMAIN[j]) + gamma) * (witness.b_x[j] + beta * evaluate_polynomial(&S_B, DOMAIN[j]) + gamma) * (witness.c_x[j] + beta * evaluate_polynomial(&S_C, DOMAIN[j]) + gamma);
            let denom_inv: Fr = denom.inverse().unwrap();

            let fraction: Fr = numer * denom_inv;

            f *= fraction;
        }

        lox = add(&lox, &scalar_mul(&lx, f));
    }

    add(&l1x, &lox)
}

//arithmetic constraint part of t(x)Z_H(x)
pub fn get_arithmetic_constraint_poly(ax: &[Fr; N+2], bx: &[Fr; N+2], cx: &[Fr; N+2], pi_x: &[Fr; L]) -> [Fr; 3*N + 2] {
    let mut arithmetic_constraint_poly: [Fr; 3*N + 2] = [Fr::from(0u32); 3*N + 2];

    let axbx: [Fr; 2*N + 3] = polynomial_multiplication(ax, bx);
    let axbxqm: [Fr; 3*N + 2] = polynomial_multiplication(&axbx, &Q_M);
    let axql: [Fr; 2*N + 1] = polynomial_multiplication(ax, &Q_L);
    let bxqr: [Fr; 2*N + 1] = polynomial_multiplication(bx, &Q_R);
    let cxqo: [Fr; 2*N + 1] = polynomial_multiplication(cx, &Q_O);

    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &axbxqm);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &axql);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &bxqr);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &cxqo);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &pi_x);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &Q_C);


    arithmetic_constraint_poly
}

//permutation constraint part of t(x)Z_H(x)
pub fn get_permutation_constraint_polynomial(alpha: Fr, beta: Fr, gamma: Fr, ax: &[Fr; N+2], bx: &[Fr; N+2], cx: &[Fr; N+2], zx: &[Fr; N+3]) -> [Fr; 4*N + 6] {
    let axbx: [Fr; 2*N + 3] = polynomial_multiplication(&add_three_poly(ax, &[Fr::zero(), beta], &[gamma]), &add_three_poly(bx, &[Fr::zero(), beta * *K1], &[gamma]));
    let cxzx: [Fr; 2*N + 4] = polynomial_multiplication(&add_three_poly(cx, &[Fr::zero(), beta * *K2], &[gamma]), &zx);

    let alpha_axbxcxzx: [Fr; 4*N + 6] = scalar_mul(&polynomial_multiplication(&axbx, &cxzx), alpha);

    let mut zwx = [Fr::zero(); N+3];
    for i in 0..N+3 {
        zwx[i] = zx[i] * OMEGA.pow([i as u64]);
    }

    let asxbsx: [Fr; 2*N+3] = polynomial_multiplication(&add_three_poly(ax, &scalar_mul(&S_A, beta), &[gamma]), &add_three_poly(bx, &scalar_mul(&S_B, beta), &[gamma]));
    let csxzwx: [Fr; 2*N + 4] = polynomial_multiplication(&add_three_poly(bx, &scalar_mul(&S_C, beta), &[gamma]), &zwx);

    let alpha_asxbsxcsxzwx: [Fr; 4*N + 6] = scalar_mul(&polynomial_multiplication(&asxbsx, &csxzwx), alpha);
    
    sub(&alpha_axbxcxzx, &alpha_asxbsxcsxzwx)
}

pub fn get_boundary_constraint_poly(alpha: Fr, zx: &[Fr; N+3]) -> [Fr; 2*N + 2] {
    scalar_mul(&polynomial_multiplication(&sub(zx, &[Fr::one()]), &lagrange_poly(0, &DOMAIN)), alpha * alpha)
}
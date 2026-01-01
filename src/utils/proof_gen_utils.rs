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
            let numer: Fr = 
                (witness.a_x[j] + beta * DOMAIN[j] + gamma) * 
                (witness.b_x[j] + beta * DOMAIN[j] * *K1 + gamma) * 
                (witness.c_x[j] + beta * DOMAIN[j] * *K2 + gamma);

            let denom: Fr = 
                (witness.a_x[j] + beta * evaluate_polynomial(&S_A, DOMAIN[j]) + gamma) * 
                (witness.b_x[j] + beta * evaluate_polynomial(&S_B, DOMAIN[j]) + gamma) * 
                (witness.c_x[j] + beta * evaluate_polynomial(&S_C, DOMAIN[j]) + gamma);

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

    let axbx: [Fr; 2*N + 3] = 
        polynomial_multiplication(&add_three_poly(ax, &[Fr::zero(), beta], &[gamma]), 
        &add_three_poly(bx, &[Fr::zero(), beta * *K1], &[gamma]));

    let cxzx: [Fr; 2*N + 4] = 
        polynomial_multiplication(&add_three_poly(cx, &[Fr::zero(), beta * *K2], &[gamma]), &zx);

    let alpha_axbxcxzx: [Fr; 4*N + 6] = 
        scalar_mul(&polynomial_multiplication(&axbx, &cxzx), alpha);

    let mut zwx = [Fr::zero(); N+3];
    for i in 0..N+3 {
        zwx[i] = zx[i] * OMEGA.pow([i as u64]);
    }

    let asxbsx: [Fr; 2*N+3] = 
        polynomial_multiplication(
            &add_three_poly(ax, &scalar_mul(&S_A, beta), &[gamma]), 
            &add_three_poly(bx, &scalar_mul(&S_B, beta), &[gamma])
        );

    let csxzwx: [Fr; 2*N + 4] = 
        polynomial_multiplication(
            &add_three_poly(bx, &scalar_mul(&S_C, beta), &[gamma]), 
            &zwx
        );

    let alpha_asxbsxcsxzwx: [Fr; 4*N + 6] = 
        scalar_mul(
            &polynomial_multiplication(&asxbsx, &csxzwx), 
            alpha
        );
    
    sub(&alpha_axbxcxzx, &alpha_asxbsxcsxzwx)
}

pub fn get_boundary_constraint_poly(alpha: Fr, zx: &[Fr; N+3]) -> [Fr; 2*N + 2] {
    scalar_mul(
        &polynomial_multiplication(
            &sub(zx, &[Fr::one()]), 
            &lagrange_poly(0, &DOMAIN)
        ), 
        alpha * alpha)
}

pub fn get_linearisation_poly(a_zeta: Fr, b_zeta: Fr, c_zeta: Fr, alpha: Fr, beta: Fr, gamma: Fr, zeta: Fr, z_omega_zeta: Fr, s1_zeta: Fr, s2_zeta: Fr, zx: &[Fr; N+3], pi_x: &[Fr; L], t_lo: &[Fr; N], t_mid: &[Fr; N], t_hi: &[Fr; N+6]) -> [Fr; N+6] {

    let arithmetic_part: [Fr; N] = 
        add(
            &add_three_poly(
                &scalar_mul(&Q_M, a_zeta * b_zeta), 
                &scalar_mul(&Q_L, a_zeta), 
                &scalar_mul(&Q_R, b_zeta)
            ), 
            &add_three_poly(
                &scalar_mul(&Q_O, c_zeta), 
                &Q_C, 
                &[evaluate_polynomial(&pi_x, zeta)]
            )
        );

    let permutation_part: [Fr; N+3] = 
        scalar_mul(
            &sub(
                &scalar_mul(
                    &zx, 
                    (a_zeta + beta * zeta + gamma) * (b_zeta + beta * *K1 * zeta + gamma) * (c_zeta + beta * *K2 * zeta + gamma)
                ), 
                &scalar_mul(
                    &add_three_poly(&scalar_mul(&S_C, beta), &[c_zeta], &[gamma]), 
                    (a_zeta + beta * s1_zeta + gamma) * (b_zeta + s2_zeta * beta + gamma) * z_omega_zeta
                )
            ), 
        alpha);

    let boundary_part: [Fr; N+3] = 
        scalar_mul(
            &sub(&zx, &[Fr::one()]), 
            alpha * alpha * evaluate_polynomial(&lagrange_poly(0, &DOMAIN), zeta)
        );

    let t_part: [Fr; N+6] = 
        scalar_mul(
            &add_three_poly(
                &scalar_mul(&t_hi, zeta.pow([2*N as u64])), 
                &scalar_mul(&t_mid, zeta.pow([N as u64])), 
                &t_lo
            ), 
            Fr::from(-1i32) * evaluate_polynomial(&ZH_X, zeta));

    let rx: [Fr; N+6] = 
        add(
            &add(&t_part, &boundary_part), 
            &add(&permutation_part, &arithmetic_part)
        );

    rx
}

pub fn get_opening_proof_poly_wz(rx: &[Fr; N+6], ax: &[Fr; N+2], bx: &[Fr; N+2], cx: &[Fr; N+2], zeta: Fr, a_zeta: Fr, b_zeta: Fr, c_zeta: Fr, s1_zeta: Fr, s2_zeta: Fr, v: Fr) -> [Fr; N+5] {

    let mut numerator: [Fr; N+6] = *rx;
    numerator = add(&numerator, &scalar_mul(&sub(&ax, &[a_zeta]), v.pow([1])));
    numerator = add(&numerator, &scalar_mul(&sub(&bx, &[b_zeta]), v.pow([2])));
    numerator = add(&numerator, &scalar_mul(&sub(&cx, &[c_zeta]), v.pow([3])));
    numerator = add(&numerator, &scalar_mul(&sub(&S_A, &[s1_zeta]), v.pow([4])));
    numerator = add(&numerator, &scalar_mul(&sub(&S_B, &[s2_zeta]), v.pow([5])));

    let denom: [Fr; 2] = [Fr::from(-1i32) * zeta, Fr::one()];

    let w_z: [Fr; N+5] = polynomial_division(&numerator, &denom);

    w_z

}

pub fn get_opening_proof_poly_wzomega(zx: &[Fr; N+3], zeta: Fr, z_omega_zeta: Fr) -> [Fr; N+2] {
    let numerator: [Fr; N+3] = sub(&zx, &[z_omega_zeta]);
    let denom: [Fr; 2] = [Fr::from(-1i32) * zeta * *OMEGA, Fr::one()];

    let w_z_omega : [Fr; N+2] = polynomial_division(&numerator, &denom);
    w_z_omega
}
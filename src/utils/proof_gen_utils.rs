use ark_ff::{Field as IField, Zero, One};
use crate::types::*;
use crate::utils::math::*;
use crate::constants::*;
use crate::witness::{Witness};

//z(x)for round 2 of proof generation
pub fn get_permutation_polynomial(witness: &Witness, beta: Field, gamma: Field) -> [Field; N] {
    let mut zx: [Field; N] = [Field::zero(); N];
    let mut f: [Field; N+1] = [Field::zero(); N+1];
    f[0] = Field::one();
    for j in 0..N {
        let numerator: Field = 
            (witness.a_x[j] + beta * DOMAIN[j] + gamma) * 
            (witness.b_x[j] + beta * DOMAIN[j] * *K1 + gamma) * 
            (witness.c_x[j] + beta * DOMAIN[j] * *K2 + gamma);

        let denominator: Field = 
            (witness.a_x[j] + beta * evaluate_polynomial(&S_A, DOMAIN[j]) + gamma) * 
            (witness.b_x[j] + beta * evaluate_polynomial(&S_B, DOMAIN[j]) + gamma) * 
            (witness.c_x[j] + beta * evaluate_polynomial(&S_C, DOMAIN[j]) + gamma);

        let denom_inv: Field = denominator.inverse().unwrap();

        let fraction: Field = numerator * denom_inv;

        f[j+1] = fraction * f[j];
    }

    for i in 0..N{
        zx = add(&zx, &scalar_mul(&lagrange_poly(i, &DOMAIN), f[i]));
    }

    zx
}

//arithmetic constraint part of t(x)Z_H(x)
pub fn get_arithmetic_constraint_poly(ax: &[Field; N+2], bx: &[Field; N+2], cx: &[Field; N+2], pi_x: &[Field; N]) -> [Field; 3*N + 2] {
    let mut arithmetic_constraint_poly: [Field; 3*N + 2] = [Field::zero(); 3*N + 2];

    let axbx: [Field; 2*N + 3] = polynomial_multiplication(ax, bx);
    let axbxqm: [Field; 3*N + 2] = polynomial_multiplication(&axbx, &Q_M);
    let axql: [Field; 2*N + 1] = polynomial_multiplication(ax, &Q_L);
    let bxqr: [Field; 2*N + 1] = polynomial_multiplication(bx, &Q_R);
    let cxqo: [Field; 2*N + 1] = polynomial_multiplication(cx, &Q_O);

    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &axbxqm);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &axql);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &bxqr);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &cxqo);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &pi_x);
    arithmetic_constraint_poly = add(&arithmetic_constraint_poly, &Q_C);


    arithmetic_constraint_poly
}

//permutation constraint part of t(x)Z_H(x)
pub fn get_permutation_constraint_polynomial(alpha: Field, beta: Field, gamma: Field, ax: &[Field; N+2], bx: &[Field; N+2], cx: &[Field; N+2], zx: &[Field; N+3]) -> [Field; 4*N + 6] {

    let axbx: [Field; 2*N + 3] = 
        polynomial_multiplication(&add_three_poly(ax, &[Field::zero(), beta], &[gamma]), 
        &add_three_poly(bx, &[Field::zero(), beta * *K1], &[gamma]));

    let cxzx: [Field; 2*N + 4] = 
        polynomial_multiplication(&add_three_poly(cx, &[Field::zero(), beta * *K2], &[gamma]), &zx);

    let alpha_axbxcxzx: [Field; 4*N + 6] = 
        scalar_mul(&polynomial_multiplication(&axbx, &cxzx), alpha);

    let mut zwx = [Field::zero(); N+3];
    for i in 0..N+3 {
        zwx[i] = zx[i] * OMEGA.pow([i as u64]);
    }

    let asxbsx: [Field; 2*N+3] = 
        polynomial_multiplication(
            &add_three_poly(ax, &scalar_mul(&S_A, beta), &[gamma]), 
            &add_three_poly(bx, &scalar_mul(&S_B, beta), &[gamma])
        );

    let csxzwx: [Field; 2*N + 4] = 
        polynomial_multiplication(
            &add_three_poly(cx, &scalar_mul(&S_C, beta), &[gamma]), 
            &zwx
        );

    let alpha_asxbsxcsxzwx: [Field; 4*N + 6] = 
        scalar_mul(
            &polynomial_multiplication(&asxbsx, &csxzwx), 
            alpha
        );
    
    sub(&alpha_axbxcxzx, &alpha_asxbsxcsxzwx)
}

pub fn get_boundary_constraint_poly(alpha: Field, zx: &[Field; N+3]) -> [Field; 2*N + 2] {
    scalar_mul(
        &polynomial_multiplication(
            &sub(zx, &[Field::one()]), 
            &lagrange_poly(0, &DOMAIN)
        ), 
        alpha * alpha)
}

pub fn get_linearisation_poly(a_zeta: Field, b_zeta: Field, c_zeta: Field, alpha: Field, beta: Field, gamma: Field, zeta: Field, z_omega_zeta: Field, s1_zeta: Field, s2_zeta: Field, zx: &[Field; N+3], pi_x: &[Field; N], t_lo: &[Field; N], t_mid: &[Field; N], t_hi: &[Field; N+6]) -> [Field; N+6] {

    let arithmetic_part: [Field; N] = 
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

    let permutation_part: [Field; N+3] = 
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

    let boundary_part: [Field; N+3] = 
        scalar_mul(
            &sub(&zx, &[Field::one()]), 
            alpha * alpha * evaluate_polynomial(&lagrange_poly(0, &DOMAIN), zeta)
        );

    let t_part: [Field; N+6] = 
        scalar_mul(
            &add_three_poly(
                &scalar_mul(&t_hi, zeta.pow([2*N as u64])), 
                &scalar_mul(&t_mid, zeta.pow([N as u64])), 
                &t_lo
            ), 
            Field::from(-1i32) * evaluate_polynomial(&ZH_X, zeta));

    let rx: [Field; N+6] = 
        add(
            &add(&t_part, &boundary_part), 
            &add(&permutation_part, &arithmetic_part)
        );

    rx
}

pub fn get_opening_proof_poly_wz(rx: &[Field; N+6], ax: &[Field; N+2], bx: &[Field; N+2], cx: &[Field; N+2], zeta: Field, a_zeta: Field, b_zeta: Field, c_zeta: Field, s1_zeta: Field, s2_zeta: Field, v: Field) -> [Field; N+5] {

    let mut numerator: [Field; N+6] = *rx;
    numerator = add(&numerator, &scalar_mul(&sub(&ax, &[a_zeta]), v.pow([1])));
    numerator = add(&numerator, &scalar_mul(&sub(&bx, &[b_zeta]), v.pow([2])));
    numerator = add(&numerator, &scalar_mul(&sub(&cx, &[c_zeta]), v.pow([3])));
    numerator = add(&numerator, &scalar_mul(&sub(&S_A, &[s1_zeta]), v.pow([4])));
    numerator = add(&numerator, &scalar_mul(&sub(&S_B, &[s2_zeta]), v.pow([5])));

    let denom: [Field; 2] = [Field::from(-1i32) * zeta, Field::one()];

    let w_z: [Field; N+5] = polynomial_division(&numerator, &denom);

    w_z

}

pub fn get_opening_proof_poly_wzomega(zx: &[Field; N+3], zeta: Field, z_omega_zeta: Field) -> [Field; N+2] {
    let numerator: [Field; N+3] = sub(&zx, &[z_omega_zeta]);
    let denom: [Field; 2] = [Field::from(-1i32) * zeta * *OMEGA, Field::one()];

    let w_z_omega : [Field; N+2] = polynomial_division(&numerator, &denom);
    w_z_omega
}
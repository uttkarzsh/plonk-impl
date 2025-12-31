use ark_bn254::{Fr};
use ark_ff::{Field, Zero, One};
use crate::utils::math::*;
use crate::constants::*;
use crate::witness::{Witness};

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
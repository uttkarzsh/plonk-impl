use ark_bn254::{Fr};
use ark_ff::Field;
use crate::utils::math::*;
use crate::constants::*;

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
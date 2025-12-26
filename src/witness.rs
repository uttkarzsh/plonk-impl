use ark_bn254::{Fr};
use std::sync::LazyLock;
use crate::constants::{N};

static A_VALUES: [i64; N] = [-4, 1, 1, -3];
static B_VALUES: [i64; N] = [1, 1, -4, 0];
static C_VALUES: [i64; N] = [-4, 1, -3, 0];

fn generate_witness(a: &[i64; N], b: &[i64; N], c: &[i64; N]) -> [Fr; 3*N] {
    let mut witness: [Fr; 3*N] = [Fr::from(0u64); 3*N];

    for i in 0..N {
        witness[i] = Fr::from(a[i]);
        witness[i + N] = Fr::from(b[i]);
        witness[i + 2*N] = Fr::from(c[i]);
    }
    witness
}

pub static WITNESS: LazyLock<[Fr; 3*N]> = LazyLock::new(|| generate_witness(&A_VALUES, &B_VALUES, &C_VALUES));
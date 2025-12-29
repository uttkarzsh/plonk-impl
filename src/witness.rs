use ark_bn254::{Fr};
use std::sync::LazyLock;
use crate::constants::{N};

pub struct Witness {
    pub a_x: [Fr; N],
    pub b_x: [Fr; N],
    pub c_x: [Fr; N],
}
static A_VALUES: [i64; N] = [-4, 1, 1, -3];
static B_VALUES: [i64; N] = [1, 1, -4, 0];
static C_VALUES: [i64; N] = [-4, 1, -3, 0];

impl Witness{
    fn generate_witness() -> Self {
        let a_x: [Fr; N] = [Fr::from(-4i64), Fr::from(1i64), Fr::from(1i64), Fr::from(-3i64)];
        let b_x: [Fr; N] = [Fr::from(1i64), Fr::from(1i64), Fr::from(-4i64), Fr::from(0i64)];
        let c_x: [Fr; N] = [Fr::from(-4i64), Fr::from(1i64), Fr::from(-3i64), Fr::from(0i64)];

        Self { a_x, b_x, c_x }
    }
}


pub static WITNESS: LazyLock<Witness> = LazyLock::new(|| Witness::generate_witness());
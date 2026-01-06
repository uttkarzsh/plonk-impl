use ark_bn254::{Fr};
use std::sync::LazyLock;
use crate::constants::{N};

pub struct Witness {
    pub a_x: [Fr; N],
    pub b_x: [Fr; N],
    pub c_x: [Fr; N],
}

impl Witness{
    fn generate_witness() -> Self {
        let a_x: [Fr; N] = [Fr::from(4i64), Fr::from(3i64), Fr::from(4i64), Fr::from(7i64)];
        let b_x: [Fr; N] = [Fr::from(0i64), Fr::from(0i64), Fr::from(3i64), Fr::from(5i64)];
        let c_x: [Fr; N] = [Fr::from(0i64), Fr::from(0i64), Fr::from(7i64), Fr::from(35i64)];

        Self { a_x, b_x, c_x }
    }
}


pub static WITNESS: LazyLock<Witness> = LazyLock::new(|| Witness::generate_witness());
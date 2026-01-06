use std::sync::LazyLock;
use crate::types::*;
use crate::constants::{N};

pub struct Witness {
    pub a_x: [Field; N],
    pub b_x: [Field; N],
    pub c_x: [Field; N],
}

impl Witness{
    fn generate_witness() -> Self {
        let a_x: [Field; N] = 
            [
                Field::from(4i64), 
                Field::from(3i64), 
                Field::from(4i64), 
                Field::from(7i64)
            ];

        let b_x: [Field; N] = 
            [
                Field::from(0i64), 
                Field::from(0i64), 
                Field::from(3i64), 
                Field::from(5i64)
            ];

        let c_x: [Field; N] = 
            [
                Field::from(0i64), 
                Field::from(0i64), 
                Field::from(7i64), 
                Field::from(35i64)
            ];

        Self { a_x, b_x, c_x }
    }
}


pub static WITNESS: LazyLock<Witness> = LazyLock::new(|| Witness::generate_witness());
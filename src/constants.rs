use ark_ff::{Field as IField, FftField};
use crate::utils::math::*;
use crate::types::*;
use std::sync::LazyLock;

pub static N: usize = 4;    //number of rows
pub static L: usize = 2;    //number of public inputs
pub static OMEGA: LazyLock<Field> = LazyLock::new(|| Field::get_root_of_unity(N as u64).unwrap());
pub static K1: LazyLock<Field> = LazyLock::new(|| Field::from(10u32));
pub static K2: LazyLock<Field> = LazyLock::new(|| Field::from(100u32));

fn calculate_zh_x () -> [Field; N+1]{
    let mut zh_x: [Field; N+1] = [Field::from(0u64); N+1];

    zh_x[0] = Field::from(-1i32);
    zh_x[N] = Field::from(1i32);

    zh_x
}

fn get_domain() -> [Field; N] {
    let omega: Field = Field::get_root_of_unity(N as u64).unwrap();
    let mut domain: [Field; N] = [Field::from(1u64); N];

    for i in 0..N{
        domain[i] = omega.pow([i as u64]);
    }

    domain
}

pub static Q_L: LazyLock<[Field; N]> = LazyLock::new(||lagrange_interpolation(&DOMAIN, 
    &[
        Field::from(-1i64), 
        Field::from(-1i64), 
        Field::from(1i64), 
        Field::from(0i64)
    ]));

pub static Q_R: LazyLock<[Field; N]> = LazyLock::new(||lagrange_interpolation(&DOMAIN, 
    &[
        Field::from(0i64), 
        Field::from(0i64), 
        Field::from(1i64), 
        Field::from(0i64)
    ]));

pub static Q_O: LazyLock<[Field; N]> = LazyLock::new(||lagrange_interpolation(&DOMAIN, 
    &[
        Field::from(0i64), 
        Field::from(0i64), 
        Field::from(-1i64), 
        Field::from(-1i64)
    ]));

pub static Q_M: LazyLock<[Field; N]> = LazyLock::new(||lagrange_interpolation(&DOMAIN, 
    &[
        Field::from(0i64), 
        Field::from(0i64), 
        Field::from(0i64), 
        Field::from(1i64)
    ]));

pub static Q_C: LazyLock<[Field; N]> = LazyLock::new(||lagrange_interpolation(&DOMAIN, 
    &[
        Field::from(0i64), 
        Field::from(0i64), 
        Field::from(0i64), 
        Field::from(0i64)
    ]));


pub static ZH_X: LazyLock<[Field; N+1]> = LazyLock::new(|| calculate_zh_x());
pub static DOMAIN: LazyLock<[Field; N]> = LazyLock::new(|| get_domain());

static SIGMA_A: LazyLock<[Field; N]> = LazyLock::new(|| 
    [
        DOMAIN[2], 
        DOMAIN[2] * *K1, 
        DOMAIN[0], 
        DOMAIN[2] * *K2
    ]);

static SIGMA_B: LazyLock<[Field; N]> = LazyLock::new(|| 
    [
        DOMAIN[0] * *K2, 
        DOMAIN[0] * *K1, 
        DOMAIN[1], 
        DOMAIN[3] * *K1
    ]);

static SIGMA_C: LazyLock<[Field; N]> = LazyLock::new(|| 
    [
        DOMAIN[1] * *K2, 
        DOMAIN[1] * *K1, 
        DOMAIN[3], 
        DOMAIN[3] * *K2
    ]);

pub static S_A: LazyLock<[Field; N]> = LazyLock::new(|| lagrange_interpolation(&DOMAIN, &SIGMA_A));
pub static S_B: LazyLock<[Field; N]> = LazyLock::new(|| lagrange_interpolation(&DOMAIN, &SIGMA_B));
pub static S_C: LazyLock<[Field; N]> = LazyLock::new(|| lagrange_interpolation(&DOMAIN, &SIGMA_C));




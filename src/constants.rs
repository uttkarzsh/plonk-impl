use ark_bn254::{Fr};
use ark_ff::{Field, FftField};
use crate::utils::math::*;
use std::sync::LazyLock;

pub static N: usize = 4;    //number of rows
pub static L: usize = 1;    //number of public inputs
pub static OMEGA: LazyLock<Fr> = LazyLock::new(|| Fr::get_root_of_unity(N as u64).unwrap());
pub static K1: LazyLock<Fr> = LazyLock::new(|| Fr::from(10u32));
pub static K2: LazyLock<Fr> = LazyLock::new(|| Fr::from(100u32));

fn calculate_zh_x () -> [Fr; N+1]{
    let mut zh_x: [Fr; N+1] = [Fr::from(0u64); N+1];

    zh_x[0] = Fr::from(-1i32);
    zh_x[N] = Fr::from(1i32);

    zh_x
}

fn get_domain() -> [Fr; N] {
    let omega: Fr = Fr::get_root_of_unity(N as u64).unwrap();
    let mut domain: [Fr; N] = [Fr::from(1u64); N];

    for i in 0..N{
        domain[i] = omega.pow([i as u64]);
    }

    domain
}

pub static Q_L: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(1i64), Fr::from(1i64)]);
pub static Q_R: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(1i64), Fr::from(0i64)]);
pub static Q_O: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(-1i64), Fr::from(0i64)]);
pub static Q_M: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(1i64), Fr::from(1i64), Fr::from(0i64), Fr::from(0i64)]);
pub static Q_C: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(0i64), Fr::from(3i64)]);


pub static ZH_X: LazyLock<[Fr; N+1]> = LazyLock::new(|| calculate_zh_x());
pub static DOMAIN: LazyLock<[Fr; N]> = LazyLock::new(|| get_domain());

static SIGMA_A: LazyLock<[Fr; N]> = LazyLock::new(|| [DOMAIN[1], DOMAIN[2] * *K1, DOMAIN[2] * *K2, DOMAIN[3] * *K2]);
static SIGMA_B: LazyLock<[Fr; N]> = LazyLock::new(|| [DOMAIN[2], DOMAIN[1] * *K1, DOMAIN[1] * *K2, DOMAIN[0] * *K1]);
static SIGMA_C: LazyLock<[Fr; N]> = LazyLock::new(|| [DOMAIN[3] * *K1, DOMAIN[3], DOMAIN[0], DOMAIN[0] * *K2]);

pub static S_A: LazyLock<[Fr; N]> = LazyLock::new(|| lagrange_interpolation(&DOMAIN, &SIGMA_A));
pub static S_B: LazyLock<[Fr; N]> = LazyLock::new(|| lagrange_interpolation(&DOMAIN, &SIGMA_B));
pub static S_C: LazyLock<[Fr; N]> = LazyLock::new(|| lagrange_interpolation(&DOMAIN, &SIGMA_C));




use ark_bn254::{Fr};
use ark_ff::{Field, FftField};
use std::sync::LazyLock;

pub static N: usize = 4;    //number of rows
pub static L: usize = 1;    //number of public inputs

fn calculate_zh_x () -> [Fr; N]{
    let mut zh_x: [Fr; N] = [Fr::from(0u64); N];

    zh_x[0] = Fr::from(-1i32);
    zh_x[N-1] = Fr::from(1i32);

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

static Q_L: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(1i64), Fr::from(1i64)]);
static Q_R: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(1i64), Fr::from(0i64)]);
static Q_O: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(-1i64), Fr::from(0i64)]);
static Q_M: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(1i64), Fr::from(1i64), Fr::from(0i64), Fr::from(0i64)]);
static Q_C: LazyLock<[Fr; N]> = LazyLock::new(||[Fr::from(0i64), Fr::from(0i64), Fr::from(0i64), Fr::from(3i64)]);

static ZH_X: LazyLock<[Fr; N]> = LazyLock::new(|| calculate_zh_x());

static DOMAIN: LazyLock<[Fr; N]> = LazyLock::new(|| get_domain());



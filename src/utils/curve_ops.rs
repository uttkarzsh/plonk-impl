use ark_ec::{CurveGroup, PrimeGroup};
use crate::types::*;
use std::sync::LazyLock;

pub static G1: LazyLock<G1Point> = LazyLock::new(|| G1Point::generator());
pub static G2: LazyLock<G2Point> = LazyLock::new(|| G2Point::generator());

pub fn multiply<G: CurveGroup, const N: usize, const M: usize>(
    a: &[G; N], 
    b: &[G::ScalarField; M]
) -> [G; M] 
where
    G: Copy,
{
    assert!(M <= N, "M must be less than or equal to N");
    let mut arr: [G; M] = [a[0]; M];
    for i in 0..M {
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn sum_group_elements<G: CurveGroup, const N: usize>(arr: &[G; N])->G where G: Copy, {
    let mut sum: G = G::zero();
    for i in 0..N {
        sum += arr[i];
    }
    sum
}

pub fn evaluate_commitment<const N: usize, const M: usize>(a: &[G1Point; N], b: &[Field; M]) -> G1Point {
    sum_group_elements(&multiply(a, b))
}
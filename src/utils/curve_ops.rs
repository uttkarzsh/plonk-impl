use ark_bn254::{Fr, G1Projective, G2Projective, Bn254};
use ark_ec::{CurveGroup, PrimeGroup, pairing::Pairing};
use std::sync::LazyLock;

pub static G1: LazyLock<G1Projective> = LazyLock::new(|| G1Projective::generator());
pub static G2: LazyLock<G2Projective> = LazyLock::new(|| G2Projective::generator());

pub fn hadamard_g1<const N: usize, const M: usize>(a: &[G1Projective; N], b: &[Fr; M]) -> [G1Projective; M]{
    let mut arr: [G1Projective; M] = [*G1; M];
    for i in 0..M{
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn hadamard_g2<const N: usize, const M: usize>(a: &[G2Projective; N], b: &[Fr; M]) -> [G2Projective; M]{
    let mut arr: [G2Projective; M] = [*G2; M];
    for i in 0..M{
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn sum_g1_array<const N: usize>(arr: &[G1Projective; N])->G1Projective{
    let mut sum: G1Projective = *G1 * Fr::from(0u64);
    for i in 0..N {
        sum += arr[i];
    }
    sum
}

pub fn sum_g2_array<const N: usize>(arr: &[G2Projective; N])->G2Projective{
    let mut sum: G2Projective = *G2 * Fr::from(0u64);
    for i in 0..N {
        sum += arr[i];
    }
    sum
}
use ark_bn254::{Fr};

pub fn polynomial<const N: usize>(a: &[i64; N]) -> [Fr; N]{
    let mut poly: [Fr; N] = [Fr::from(0u32); N];
    for i in 0..N{
        poly[i] = Fr::from(a[i]);
    }
    poly
}
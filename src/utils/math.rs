use ark_ff::{Field as IField, Zero, One};
use crate::types::*;

pub fn add<const N: usize, const M: usize>(a: &[Field; N], b: &[Field; M]) -> [Field; N]{
    assert!(N >= M);
    let mut arr: [Field; N] = *a;
    for i in 0..M{
        arr[i] += b[i];
    }
    arr
}

pub fn add_three_poly<const N: usize, const M: usize, const L: usize>(a: &[Field; N], b: &[Field; M], c: &[Field; L]) -> [Field; N]{
    assert!(N >= M && N >= L);
    let mut arr: [Field; N] = *a;
    for i in 0..M{
        arr[i] += b[i];
    }
    for i in 0..L {
        arr[i] += c[i];
    }

    arr
}

pub fn sub<const N: usize, const M: usize>(a: &[Field; N], b: &[Field; M]) -> [Field; N]{
    assert!(N >= M);
    let mut arr: [Field; N] = *a;
    for i in 0..M{
        arr[i] -= b[i];
    }
    arr
}

pub fn scalar_mul<const N: usize>(matrix: &[Field; N], scalar: Field) -> [Field; N] {
    let mut arr: [Field; N] = [Field::zero(); N];
    for i in 0..N{
        arr[i] = scalar * matrix[i];
    }
    arr
}

pub fn polynomial_multiplication<const N: usize, const M: usize, const L: usize>(a: &[Field; N], b: &[Field; M]) -> [Field; L]{
    assert!(L == N + M - 1);
    let mut product: [Field; L] = [Field::zero(); L];
    for i in 0..N {
        for j in 0..M{
            product[j + i] += a[i] * b[j];
        }
    }
    product
}

pub fn lagrange_poly<const N: usize>(i: usize, xs: &[Field; N]) -> [Field; N] {
    assert!(i <= N);

    let mut poly = [Field::zero(); N];
    poly[0] = Field::one();

    let mut denom = Field::one();
    for j in 0..N {
        if i != j {
            denom *= xs[i] - xs[j];
        }
    }
    let denom_inv = denom.inverse().unwrap();

    for j in 0..N {
        if i != j {
            let mut next = [Field::zero(); N];
            for k in 0..N {
                if k > 0 {
                    next[k] += poly[k - 1];
                }
                next[k] -= poly[k] * xs[j];
            }
            poly = next;
        }
    }

    for k in 0..N {
        poly[k] *= denom_inv;
    }

    poly
}


pub fn lagrange_interpolation<const N: usize>(xs: &[Field; N], ys: &[Field; N]) -> [Field; N] {
    let mut result = [Field::zero(); N];
    
    for i in 0..N {
        let mut basis = [Field::zero(); N];
        basis[0] = Field::one();
        
        let mut denom = Field::one();
        for j in 0..N {
            if i != j {
                denom *= xs[i] - xs[j];
            }
        }
        let denom_inv = denom.inverse().unwrap();
        
        for j in 0..N {
            if i != j {
                let mut new_basis = [Field::zero(); N];
                for k in 0..N {
                    if k > 0 {
                        new_basis[k] += basis[k - 1]; 
                    }
                    new_basis[k] -= basis[k] * xs[j]; 
                }
                basis = new_basis;
            }
        }
        for k in 0..N {
            result[k] += basis[k] * ys[i] * denom_inv;
        }
    }
    
    result
}


pub fn polynomial_division<const N: usize, const M: usize, const L: usize>(px: &[Field; N], qx: &[Field; M]) -> [Field; L] {
    assert!(N >= M && L == N - M + 1);
    let mut res: [Field; L] = [Field::zero(); L];
    let mut zx: [Field; N] = *px;

    let deg_p: usize = N - 1;
    let deg_q: usize = M - 1;

    let mut deg_z: usize = deg_p;
    let iterations: usize = deg_p - deg_q + 1;

    for i in 0..iterations{
        let factor: Field = zx[deg_z] / qx[deg_q];
        res[iterations - 1 - i] = factor;

        for j in 0..=deg_q{
            zx[deg_z - j] -= factor * qx[deg_q - j];
        }
        deg_z -= 1;
    }

    res
}

pub fn evaluate_polynomial<const N: usize>(polynomial: &[Field; N], var: Field) -> Field {
    let mut value: Field = Field::zero();
    for i in 0..N{
        value += polynomial[i] * var.pow([i as u64]);
    }
    value
}


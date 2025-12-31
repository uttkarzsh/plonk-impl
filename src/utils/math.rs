use ark_bn254::{Fr};
use ark_ff::Field;


pub fn add<const N: usize, const M: usize>(a: &[Fr; N], b: &[Fr; M]) -> [Fr; N]{
    let mut arr: [Fr; N] = *a;
    for i in 0..M{
        arr[i] += b[i];
    }
    arr
}

pub fn add_three_arr<const N: usize>(a: &[Fr; N], b: &[Fr; N], c: &[Fr; N]) -> [Fr; N]{
    let mut arr: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N{
        arr[i] = a[i] + b[i] + c[i];
    }
    arr
}

pub fn sub<const N: usize>(a: &[Fr; N], b: &[Fr; N]) -> [Fr; N]{
    let mut arr: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N{
        arr[i] = a[i] - b[i];
    }
    arr
}

pub fn scalar_mul<const N: usize>(matrix: &[Fr; N], scalar: Fr) -> [Fr; N] {
    let mut arr: [Fr; N] = [Fr::from(0u64); N];
    for i in 0..N{
        arr[i] = scalar * matrix[i];
    }
    arr
}

pub fn hadamard_product<const N: usize, const M: usize>(a: &[Fr; N], b: &[Fr; M]) -> [Fr; M]{
    let mut arr: [Fr; M] = [Fr::from(0u64); M];
    for i in 0..M{
        arr[i] = a[i] * b[i];
    }
    arr
}

pub fn arr_sum<const N: usize>(arr: &[Fr; N]) -> Fr{
    let mut sum: Fr = Fr::from(0u64);
    for i in 0..N{
        sum += arr[i];
    }
    sum
}

pub fn polynomial_multiplication<const N: usize, const M: usize, const L: usize>(a: &[Fr; N], b: &[Fr; M]) -> [Fr; L]{
    let mut product: [Fr; L] = [Fr::from(0u64); L];
    for i in 0..N {
        for j in 0..M{
            product[j + i] += a[i] * b[j];
        }
    }
    product
}

pub fn lagrange_interpolation<const N: usize>(xs: &[Fr; N], ys: &[Fr; N]) -> [Fr; N] {
    let mut result = [Fr::from(0u64); N];
    
    for i in 0..N {
        let mut basis = [Fr::from(0u64); N];
        basis[0] = Fr::from(1u64);
        
        let mut denom = Fr::from(1u64);
        for j in 0..N {
            if i != j {
                denom *= xs[i] - xs[j];
            }
        }
        let denom_inv = denom.inverse().unwrap();
        
        for j in 0..N {
            if i != j {
                let mut new_basis = [Fr::from(0u64); N];
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


pub fn polynomial_division<const N: usize>(px: &[Fr; N], qx: &[Fr; N], deg_p: usize, deg_q: usize) -> [Fr; N] {
    let mut res: [Fr; N] = [Fr::from(0u64); N];
    let mut zx: [Fr; N] = *px;
    let mut deg_z: usize = deg_p;
    let iterations: usize = deg_p - deg_q + 1;

    for i in 0..iterations{
        let factor: Fr = zx[deg_z] / qx[deg_q];
        res[iterations - 1 - i] = factor;

        for j in 0..=deg_q{
            zx[deg_z - j] -= factor * qx[deg_q - j];
        }
        deg_z -= 1;
    }

    res
}

pub fn domain_pub_input<const N: usize, const M: usize>(domain:&[Fr; N]) -> [Fr; M] {
    let mut p_domain: [Fr; M] = [Fr::from(0u32); M];
    for i in 0..M{
        p_domain[i] = domain[i];
    }
    p_domain
}

pub fn evaluate_polynomial<const N: usize>(polynomial: &[Fr; N], var: Fr) -> Fr {
    let mut value: Fr = Fr::from(0u64);
    for i in 0..N{
        value += polynomial[i] * var.pow([i as u64]);
    }
    value
}


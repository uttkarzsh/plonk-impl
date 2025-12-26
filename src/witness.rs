use ark_bn254::{Fr};
use crate::constants::{N};

static A_VALUES: [Fr; N] = [Fr::from(-4i64), Fr::from(1i64), Fr::from(1i64), Fr::from(-3i64)];
static B_VALUES: [Fr; N] = [Fr::from(1i64), Fr::from(1i64), Fr::from(-4i64), Fr::from(0i64)];
static C_VALUES: [Fr; N] = [Fr::from(-4i64), Fr::from(1i64), Fr::from(-3i64), Fr::from(0i64)];
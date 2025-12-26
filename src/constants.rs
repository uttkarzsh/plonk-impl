use ark_bn254::{Fr};

static N: usize = 4;    //number of rows
static L: usize = 1;    //number of public inputs



static Q_L: [Fr; N] = [Fr::from(0i64), Fr::from(0i64), Fr::from(1i64), Fr::from(1i64)];
static Q_R: [Fr; N] = [Fr::from(0i64), Fr::from(0i64), Fr::from(1i64), Fr::from(0i64)];
static Q_O: [Fr; N] = [Fr::from(0i64), Fr::from(0i64), Fr::from(-1i64), Fr::from(0i64)];
static Q_M: [Fr; N] = [Fr::from(1i64), Fr::from(1i64), Fr::from(0i64), Fr::from(0i64)];
static Q_C: [Fr; N] = [Fr::from(0i64), Fr::from(0i64), Fr::from(0i64), Fr::from(3i64)];



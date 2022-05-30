use ark_bls12_381::Fq;
use ark_ff::{Field, UniformRand};
// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
#[derive(Debug)]
pub struct Freivald {
    x: Vec<Fq>, // Array/Vec of Fq,
}
impl Freivald {
    // TODO: Create constructor for object
    pub fn new(array_size: usize) -> Self {
        // todo!()
        // Generate random number
        // Populate vector with values r^i for i=0..matrix_size
        // Return freivald value with this vector as its x value
        let mut rng = rand::thread_rng();
        let random_number = Fq::rand(&mut rng);

        let x = (0..array_size)
            .map(|i| random_number.pow([(i) as u64]))
            .collect();

        Self { x }
    }

    // TODO: Add proper types to input matrices. Remember matrices should hold Fq values
    pub fn verify(
        &self,
        matrix_a: Vec<Vec<Fq>>,
        matrix_b: Vec<Vec<Fq>>,
        supposed_ab: Vec<Vec<Fq>>,
    ) -> bool {
        assert!(check_matrix_dimensions(&matrix_a, &matrix_b, &supposed_ab));
        // todo!()
        // TODO: check if a * (b * x) == c * x. Check algorithm to make sure order of operations are
        let b_x = mat_vec_mul(&matrix_b, &self.x);
        let a_b_x = mat_vec_mul(&matrix_a, &b_x);
        let c_x = mat_vec_mul(&supposed_ab, &self.x);
        a_b_x == c_x
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // TODO: Add types for arguments
    pub fn verify_once(
        matrix_a: Vec<Vec<Fq>>,
        matrix_b: Vec<Vec<Fq>>,
        supposed_ab: Vec<Vec<Fq>>,
    ) -> bool {
        let freivald = Freivald::new(supposed_ab.len());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}

// TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(
    mat_a: &Vec<Vec<Fq>>,
    mat_b: &Vec<Vec<Fq>>,
    mat_res: &Vec<Vec<Fq>>,
) -> bool {
    // [x y] . [y z] = [x z]
    // assume that all vecs in the vec have the same length
    let a_length_x = mat_a.len();
    let a_length_y = mat_a[0].len();

    let b_length_y = mat_b.len();
    let b_length_z = mat_b[0].len();

    let res_length_x = mat_res.len();
    let res_length_z = mat_res[0].len();

    let eq_x:bool = a_length_x == res_length_x;
    let eq_y:bool = a_length_y == b_length_y;
    let eq_z:bool = b_length_z == res_length_z;

    eq_x && eq_y && eq_z
}

pub fn mat_vec_mul(mat: &Vec<Vec<Fq>>, vec: &Vec<Fq>) -> Vec<Fq> {
    // [a b] . [b 1] = [a 1]
    let mut result: Vec<Fq> = Vec::new();
    for i in 0..mat.len() {
        let mut sum = Fq::from(0);
        for j in 0..vec.len() {
            let mul_res = mat[i][j] * vec[j];
            sum = sum + mul_res;
        }
        result.push(sum);
    }

    result
}


// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct but that can change if you want to)
pub struct RepFreivald {
    x: Vec<Freivald>,
}

impl RepFreivald {
    pub fn new(round: u32, array_size: usize) -> Self {
        let x = (0..round).map(|_| Freivald::new(array_size)).collect();
        Self { x }
    }

    pub fn multi_verify(
        &self,
        mat_a: Vec<Vec<Fq>>,
        mat_b: Vec<Vec<Fq>>,
        supposed_ab: Vec<Vec<Fq>>,
    ) -> bool {
        for x in self.x.iter() {
            let res = x.verify(mat_a.clone(), mat_b.clone(), supposed_ab.clone());
            if !res {
                return false;
            }
        }

        true
    }
}

// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    let v = Freivald::new(2);
    let mat_a: Vec<Vec<Fq>> = vec![
        vec![Fq::from(1), Fq::from(2)],
        vec![Fq::from(3), Fq::from(4)],
    ];
    let mat_b: Vec<Vec<Fq>> = vec![
        vec![Fq::from(1), Fq::from(1)],
        vec![Fq::from(1), Fq::from(1)],
    ];
    let mat_solution: Vec<Vec<Fq>> = vec![
        vec![Fq::from(3), Fq::from(3)],
        vec![Fq::from(7), Fq::from(7)],
    ];

    let result = v.verify(mat_a, mat_b, mat_solution);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        static ref MATRIX_A: Vec<Vec<Fq>> = vec![
            vec![Fq::from(1), Fq::from(2)],
            vec![Fq::from(3), Fq::from(4)],
        ];
        static ref MATRIX_A_DOT_A: Vec<Vec<Fq>> = vec![
            vec![Fq::from(7), Fq::from(10)],
            vec![Fq::from(15), Fq::from(22)],
        ];
        static ref MATRIX_B: Vec<Vec<Fq>> = vec![
            vec![Fq::from(2), Fq::from(4)],
            vec![Fq::from(6), Fq::from(8)],
        ];
        static ref MATRIX_B_DOT_B: Vec<Vec<Fq>> = vec![
            vec![Fq::from(28), Fq::from(40)],
            vec![Fq::from(60), Fq::from(88)],
        ];
        static ref MATRIX_C: Vec<Vec<Fq>> = vec![
            vec![Fq::from(3), Fq::from(6)],
            vec![Fq::from(9), Fq::from(12)],
        ];
        static ref MATRIX_C_DOT_C: Vec<Vec<Fq>> = vec![
            vec![Fq::from(63), Fq::from(90)],
            vec![Fq::from(135), Fq::from(198)],
        ];
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Vec<Vec<Fq>>,
        #[case] matrix_b: &Vec<Vec<Fq>>,
        #[case] supposed_ab: &Vec<Vec<Fq>>,
    ) {
        let freivald = Freivald::new(supposed_ab.len());
        assert!(freivald.verify(matrix_a.clone(), matrix_b.clone(), supposed_ab.clone()));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Vec<Vec<Fq>>,
        #[case] b: &Vec<Vec<Fq>>,
        #[case] c: &Vec<Vec<Fq>>,
    ) {
        let freivald = Freivald::new(c.len());
        assert!(!freivald.verify(a.clone(), b.clone(), c.clone()));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn multiple_freivald_verify_success_test(
        #[case] matrix_a: &Vec<Vec<Fq>>,
        #[case] matrix_b: &Vec<Vec<Fq>>,
        #[case] supposed_ab: &Vec<Vec<Fq>>,
    ) {
        //repeat test for 10 rounds
        let round = 10;
        let multi = RepFreivald::new(round, supposed_ab.len());
        assert!(multi.multi_verify(
            matrix_a.clone(),
            matrix_b.clone(),
            supposed_ab.clone()
        ))
    }
}
extern crate num_complex;

use cmatrix;
use gates;
use qasm;

/// The `T` gate
///
/// The `T` gate rotates the state over π/4 radians around the `z` axis of
/// the Bloch sphere. It is the square root of the `S` gate.
pub struct T
{
}

impl T
{
    /// Create a new `T` gate.
    pub fn new() -> Self
    {
        T { }
    }
}

impl gates::Gate for T
{
    fn cost(&self) -> f64
    {
        gates::U1::cost()
    }

    fn description(&self) -> &str
    {
        "T"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let i = cmatrix::COMPLEX_I;
        array![[o, z], [z, x+x*i]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        let mut slice = state.slice_mut(s![n..]);
        slice *= num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        assert!(state.rows() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        let mut slice = state.slice_mut(s![n.., ..]);
        slice *= num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);
    }
}

impl qasm::OpenQasm for T
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("t {}", bit_names[bits[0]])
    }
}

impl qasm::CQasm for T
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("t {}", bit_names[bits[0]])
    }
}

/// Conjugate of `T` gate
///
/// The `T`<sup>`†`</sup> gate rotates the state over -π/4 radians around the
/// `z` axis of the Bloch sphere. It is the conjugate of the `T` gate.
pub struct Tdg
{
}

impl Tdg
{
    /// Create a new `T`<sup>`†`</sup> gate.
    pub fn new() -> Self
    {
        Tdg { }
    }
}

impl gates::Gate for Tdg
{
    fn cost(&self) -> f64
    {
        gates::U1::cost()
    }

    fn description(&self) -> &str
    {
        "T†"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let i = cmatrix::COMPLEX_I;
        array![[o, z], [z, x-x*i]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        let mut slice = state.slice_mut(s![n..]);
        slice *= num_complex::Complex::from_polar(&1.0, &-::std::f64::consts::FRAC_PI_4);
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        assert!(state.rows() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        let mut slice = state.slice_mut(s![n.., ..]);
        slice *= num_complex::Complex::from_polar(&1.0, &-::std::f64::consts::FRAC_PI_4);
    }
}

impl qasm::OpenQasm for Tdg
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("tdg {}", bit_names[bits[0]])
    }
}

impl qasm::CQasm for Tdg
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("tdag {}", bit_names[bits[0]])
    }
}

#[cfg(test)]
mod tests
{
    extern crate num_complex;

    use super::{T, Tdg};
    use gates::Gate;
    use cmatrix;

    #[test]
    fn test_description()
    {
        let gate = T::new();
        assert_eq!(gate.description(), "T");
        let gate = Tdg::new();
        assert_eq!(gate.description(), "T†");
    }

    #[test]
    fn test_matrix()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let t = num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);

        let gate = T::new();
        assert_complex_matrix_eq!(gate.matrix(), array![[o, z], [z, t]]);

        let gate = Tdg::new();
        assert_complex_matrix_eq!(gate.matrix(), array![[o, z], [z, t.conj()]]);
    }

    #[test]
    fn test_apply_mat()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let h = 0.5 * o;
        let x = cmatrix::COMPLEX_HSQRT2;
        let t = num_complex::Complex::from_polar(&1.0, &::std::f64::consts::FRAC_PI_4);
        let td = t.conj();

        let mut state = array![
            [o, z, x,  h, z],
            [z, o, z, -h, z],
            [z, z, x,  h, z],
            [z, z, z, -h, o]
        ];
        T::new().apply_mat(&mut state);
        assert_complex_matrix_eq!(&state, &array![
            [o, z,   x,    h, z],
            [z, o,   z,   -h, z],
            [z, z, t*x,  t*h, z],
            [z, z,   z, -t*h, t]
        ]);

        let mut state = array![
            [o, z, x,  h, z],
            [z, o, z, -h, z],
            [z, z, x,  h, z],
            [z, z, z, -h, o]
        ];
        Tdg::new().apply_mat(&mut state);
        assert_complex_matrix_eq!(&state, &array![
            [o, z,    x,     h,  z],
            [z, o,    z,    -h,  z],
            [z, z, td*x,  td*h,  z],
            [z, z,    z, -td*h, td]
        ]);
    }

    #[test]
    fn test_open_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = T::new().open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "t qb");
        let qasm = Tdg::new().open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "tdg qb");
    }

    #[test]
    fn test_c_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = T::new().c_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "t qb");
        let qasm = Tdg::new().c_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "tdag qb");
    }
}
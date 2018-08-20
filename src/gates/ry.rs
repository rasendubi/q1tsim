extern crate num_complex;

use cmatrix;
use gates;

/// Rotation around `y` axis.
///
/// The `R`<sub>`Y`</sub>`(λ)` gate rotates the qubit around the `y` axis of the
/// Bloch sphere over an angle `theta`.
pub struct RY
{
    theta: f64,
    desc: String
}

impl RY
{
    /// Create a new `R`<sub>`Y`</sub> gate.
    pub fn new(theta: f64) -> Self
    {
        RY { theta: theta, desc: format!("RY({:.4})", theta) }
    }
}

impl gates::Gate for RY
{
    fn cost(&self) -> f64
    {
        gates::U3::cost()
    }

    fn description(&self) -> &str
    {
        &self.desc
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let c = num_complex::Complex::new((0.5 * self.theta).cos(), 0.0);
        let s = num_complex::Complex::new((0.5 * self.theta).sin(), 0.0);
        array![[c, -s], [s, c]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        let cos_t = num_complex::Complex::new((0.5 * self.theta).cos(), 0.0);
        let sin_t = num_complex::Complex::new((0.5 * self.theta).sin(), 0.0);

        let mut s = state.to_owned();
        s *= sin_t;
        *state *= cos_t;

        let n = state.len() / 2;
        {
            let mut slice = state.slice_mut(s![..n]);
            slice -= &s.slice(s![n..]);
        }
        {
            let mut slice = state.slice_mut(s![n..]);
            slice += &s.slice(s![..n]);
        }
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        let cos_t = num_complex::Complex::new((0.5 * self.theta).cos(), 0.0);
        let sin_t = num_complex::Complex::new((0.5 * self.theta).sin(), 0.0);

        let mut s = state.to_owned();
        s *= sin_t;
        *state *= cos_t;

        let n = state.rows() / 2;
        {
            let mut slice = state.slice_mut(s![..n, ..]);
            slice -= &s.slice(s![n.., ..]);
        }
        {
            let mut slice = state.slice_mut(s![n.., ..]);
            slice += &s.slice(s![..n, ..]);
        }
    }

    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("ry({}) {}", self.theta, bit_names[bits[0]])
    }
}

#[cfg(test)]
mod tests
{
    use gates::{gate_test, Gate, RY};
    use cmatrix;

    #[test]
    fn test_description()
    {
        let gate = RY::new(0.21675627161);
        assert_eq!(gate.description(), "RY(0.2168)");
    }

    #[test]
    fn test_matrix()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;

        let gate = RY::new(::std::f64::consts::FRAC_PI_2);
        assert_complex_matrix_eq!(gate.matrix(), array![[x, -x], [x, x]]);

        let gate = RY::new(::std::f64::consts::PI);
        assert_complex_matrix_eq!(gate.matrix(), array![[z, -o], [o, z]]);
    }

    #[test]
    fn test_apply()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let mut state = array![
            [o, z, x,  x],
            [z, o, x, -x]
        ];
        let result = array![
            [x, -x, z, o],
            [x,  x, o, z]
        ];
        let gate = RY::new(::std::f64::consts::FRAC_PI_2);
        gate_test(gate, &mut state, &result);
    }

    #[test]
    fn test_open_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = RY::new(2.25).open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "ry(2.25) qb");
    }
}

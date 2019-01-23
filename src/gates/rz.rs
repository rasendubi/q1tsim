// Copyright 2019 Q1t BV
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate num_complex;

use cmatrix;
use gates;
use qasm;

/// Rotation around `z` axis.
///
/// The `R`<sub>`Z`</sub>`(λ)` gate rotates the qubit around the `z` axis of the
/// Bloch sphere over an angle `λ`. It is equivalent to the `U`<sub>`1`</sub>
/// gate, up to an overall phase.
pub struct RZ
{
    lambda: f64,
    desc: String
}

impl RZ
{
    /// Create a new `R`<sub>`Z`</sub> gate.
    pub fn new(lambda: f64) -> Self
    {
        RZ { lambda: lambda, desc: format!("RZ({:.4})", lambda) }
    }
}

impl gates::Gate for RZ
{
    fn cost(&self) -> f64
    {
        gates::U1::cost()
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
        let z = cmatrix::COMPLEX_ZERO;
        let p = num_complex::Complex::from_polar(&1.0, &(0.5 * self.lambda));
        array![[p.conj(), z], [z, p]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        {
            let mut slice = state.slice_mut(s![..n]);
            slice *= num_complex::Complex::from_polar(&1.0, &(-0.5*self.lambda));
        }
        {
            let mut slice = state.slice_mut(s![n..]);
            slice *= num_complex::Complex::from_polar(&1.0, &( 0.5*self.lambda));
        }
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        {
            let mut slice = state.slice_mut(s![..n, ..]);
            slice *= num_complex::Complex::from_polar(&1.0, &(-0.5*self.lambda));
        }
        {
            let mut slice = state.slice_mut(s![n.., ..]);
            slice *= num_complex::Complex::from_polar(&1.0, &( 0.5*self.lambda));
        }
    }
}

impl qasm::OpenQasm for RZ
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("rz({}) {}", self.lambda, bit_names[bits[0]])
    }
}

impl qasm::CQasm for RZ
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("rz {}, {}", bit_names[bits[0]], self.lambda)
    }
}

#[cfg(test)]
mod tests
{
    use gates::{gate_test, Gate, RZ};
    use qasm::{OpenQasm, CQasm};
    use cmatrix;

    #[test]
    fn test_description()
    {
        let gate = RZ::new(::std::f64::consts::FRAC_PI_4);
        assert_eq!(gate.description(), "RZ(0.7854)");
    }

    #[test]
    fn test_matrix()
    {
        let gate = RZ::new(::std::f64::consts::PI);
        let z = cmatrix::COMPLEX_ZERO;
        let i = cmatrix::COMPLEX_I;
        assert_complex_matrix_eq!(gate.matrix(), array![[-i, z], [z, i]]);
    }

    #[test]
    fn test_apply()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let i = cmatrix::COMPLEX_I;
        let mut state = array![
            [o, z, x,  x],
            [z, o, x, -x]
        ];
        let result = array![
            [x*(o-i), z,       0.5*(o-i),  0.5*(o-i)],
            [z      , x*(o+i), 0.5*(o+i), -0.5*(o+i)]
        ];
        let gate = RZ::new(::std::f64::consts::FRAC_PI_2);
        gate_test(gate, &mut state, &result);
    }

    #[test]
    fn test_open_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = RZ::new(2.25).open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "rz(2.25) qb");
    }

    #[test]
    fn test_c_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = RZ::new(2.25).c_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "rz qb, 2.25");
    }
}

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


use cmatrix;
use gates;
use export;

use gates::Gate;

/// Static loop gate
///
/// The `Loop` gate represents a static loop, i.e. a set of instructions that
/// is executed a fixed number of times.
pub struct Loop
{
    /// The loop label, naming the loop
    label: String,
    /// The number of times to execute the loop body
    nr_iterations: usize,
    /// The instructions to loop
    body: gates::Composite,
    /// A descriptions string, describing the loop
    desc: String
}

impl Loop
{
    /// Create a new static loop.
    ///
    /// Initialize a new static loop executing the instructions in `body`,
    /// `nr_iterations` times.
    pub fn new(label: &str, nr_iterations: usize, body: gates::Composite) -> Self
    {
        let desc = format!("{}({})", nr_iterations, body.description());
        Loop
        {
            label: String::from(label),
            nr_iterations: nr_iterations,
            body: body,
            desc: desc
        }
    }
}

impl gates::Gate for Loop
{
    fn cost(&self) -> f64
    {
        self.nr_iterations as f64 * self.body.cost()
    }

    fn description(&self) -> &str
    {
        &self.desc
    }

    fn nr_affected_bits(&self) -> usize
    {
        self.body.nr_affected_bits()
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let mut res = cmatrix::CMatrix::eye(1 << self.nr_affected_bits());
        for _ in 0..self.nr_iterations
        {
            self.body.apply_mat(&mut res);
        }
        res
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        for _ in 0..self.nr_iterations
        {
            self.body.apply_slice(state);
        }
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        for _ in 0..self.nr_iterations
        {
            self.body.apply_mat_slice(state);
        }
    }
}

impl export::OpenQasm for Loop
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        if self.nr_iterations == 0
        {
            String::new()
        }
        else
        {
            let qasm_body = self.body.open_qasm(bit_names, bits);
            let mut res = qasm_body.clone();
            for _ in 1..self.nr_iterations
            {
                res += ";\n";
                res += &qasm_body;
            }
            res
        }
    }

    fn conditional_open_qasm(&self, _condition: &str, _bit_names: &[String],
        _bits: &[usize]) -> Result<String, String>
    {
        Err(String::from("Classical conditions cannot be used in conjunction with a static loop"))
    }
}

impl export::CQasm for Loop
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!(".{}({})\n{}\n.end", self.label, self.nr_iterations,
            self.body.c_qasm(bit_names, bits))
    }

    fn conditional_c_qasm(&self, _condition: &str, _bit_names: &[String],
        _bits: &[usize]) -> Result<String, String>
    {
        Err(String::from("Classical conditions cannot be used in conjunction with a static loop"))
    }
}

impl export::Latex for Loop
{
    fn latex(&self, bits: &[usize], state: &mut export::LatexExportState)
    {
        if self.nr_iterations == 1
        {
            self.body.latex(bits, state);
        }
        else if self.nr_iterations == 2
        {
            self.body.latex(bits, state);
            self.body.latex_checked(bits, state);
        }
        else if self.nr_iterations > 2
        {
            let min = *bits.iter().min().unwrap();
            let max = *bits.iter().max().unwrap();

            state.start_loop(self.nr_iterations);
            self.body.latex(bits, state);
            state.add_cds(min, max - min, r"\cdots");
            self.body.latex_checked(bits, state);
            state.end_loop();
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::Loop;
    use gates::{gate_test, Composite, Gate};
    use export::{OpenQasm, CQasm};
    use cmatrix;

    #[test]
    fn test_description()
    {
        let body = Composite::from_string("body", "RX(1.0471975511965976) 0").unwrap();
        let gate = Loop::new("myloop", 3, body);
        assert_eq!(gate.description(), "3(body)");
    }

    #[test]
    fn test_matrix()
    {
        let body = Composite::from_string("body", "RX(1.0471975511965976) 0").unwrap();
        let gate = Loop::new("myloop", 3, body);
        let z = cmatrix::COMPLEX_ZERO;
        let i = cmatrix::COMPLEX_I;
        assert_complex_matrix_eq!(gate.matrix(), array![[z, -i], [-i, z]]);
    }

    #[test]
    fn test_apply()
    {
        let body = Composite::from_string("body", "RX(1.0471975511965976) 0; RX(-1.0471975511965976) 1").unwrap();
        let gate = Loop::new("myloop", 3, body);
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;

        let mut state = array![
            [o, z, x, x],
            [z, o, z, x],
            [z, z, z, z],
            [z, z, x, z],
        ];
        let result = array![
            [ z, z, x, z ],
            [ z, z, z, z ],
            [ z, o, z, x ],
            [ o, z, x, x ]
        ];
        gate_test(gate, &mut state, &result);
    }

    #[test]
    fn test_open_qasm()
    {
        let body = Composite::from_string("body", "H 0; H 1; CX 0 1").unwrap();
        let gate = Loop::new("myloop", 3, body);
        let bit_names = [String::from("qb0"), String::from("qb1")];
        let qasm = gate.open_qasm(&bit_names, &[0, 1]);
        assert_eq!(qasm, "h qb0; h qb1; cx qb0, qb1;\nh qb0; h qb1; cx qb0, qb1;\nh qb0; h qb1; cx qb0, qb1");
    }

    #[test]
    fn test_c_qasm()
    {
        let body = Composite::from_string("body", "H 0; H 1; CX 0 1").unwrap();
        let gate = Loop::new("myloop", 3, body);
        let bit_names = [String::from("qb0"), String::from("qb1")];
        let qasm = gate.c_qasm(&bit_names, &[0, 1]);
        assert_eq!(qasm, ".myloop(3)\nh qb0\nh qb1\ncnot qb0, qb1\n.end");
    }
}

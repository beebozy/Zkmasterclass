#[warn(dead_code)]
use ark_ff::PrimeField;
use ark_bn254::Fq;
use  ark_ff::{self, Field};
use std::result;
use ark_std::rand::rngs::StdRng;

use rand::{Rng, SeedableRng};

pub struct Polynomial<Fq:PrimeField> {
    pub coefficient: Vec<Fq>,
}

pub struct Point<Fq:PrimeField> {
    pub value: Vec<(Fq, Fq)>,
}

impl Polynomial<Fq> {

    pub fn new(coefficient:Vec<Fq>)->Self{
        Self{coefficient}
    }
    pub fn evaluate(&self, x: Fq) -> Fq {
        self.coefficient
            .iter()
            .enumerate()
            .map(|(index, coeff)| *coeff * x.pow(&[index as u64]))
            .sum()
    }

    fn degree(&self) -> usize {
        self.coefficient.len()-1
    }
}

impl Point<Fq> {

    pub fn new(value:Vec<(Fq, Fq)>)->Self{
        Self{value}
    }

    pub fn interpolate(&self) -> Vec<Fq> {
        let n: usize = self.value.len();

        // Start with an empty coefficient vector of size `n`.
        let mut coefficient = vec![Fq::from(0); n];

        // Iterate over each point `(x_i, y_i)`.
        for (i, &(x_i, y_i)) in self.value.iter().enumerate() {
            let mut denominator = Fq::from(1);
            let mut basis_poly = vec![Fq::from(1)];

            
            for (j, &(x_j, _)) in self.value.iter().enumerate() {
                if i != j {
                    denominator *= x_i - x_j;
                    basis_poly = multiply(&basis_poly, vec![-x_j, Fq::from(1)]);
                }
            }

            // Scale the basis polynomial by `y_i / denominator`.
            let scaled_poly = scale(&basis_poly, y_i / denominator);

            // Add the scaled polynomial to the result.
            coefficient = add_poly(&coefficient, &scaled_poly);
        }

        coefficient
    }
}

/// Multiply two polynomials.
fn multiply(poly1: &[Fq], poly2: Vec<Fq>) -> Vec<Fq> {
    let mut result: Vec<Fq> = vec![Fq::from(0); poly1.len() + poly2.len() - 1];
    for (i, &coeff1) in poly1.iter().enumerate() {
        for (j, &coeff2) in poly2.iter().enumerate() {
            result[i + j] =result[i+j] +coeff1 * coeff2;
        }
    }
    result
}

/// Scale a polynomial by a constant.
fn scale(poly: &[Fq], x: Fq) -> Vec<Fq> {


    poly.iter().map(|&coefficient| coefficient * x).collect()
}

/// Add two polynomials.
fn add_poly(poly1: &[Fq], poly2: &[Fq]) -> Vec<Fq> {
    let len = usize::max(poly1.len(), poly2.len());
    let mut result = vec![Fq::from(0); len];
    for i in 0..len {
        let coeff1 = if i < poly1.len() { poly1[i] } else { Fq::from(0) };
        let coeff2 = if i < poly2.len() { poly2[i] } else { Fq::from(0) };
        result[i] = coeff1 + coeff2;
    }
    result
}


#[warn(dead_code)]

pub struct Polynomial {
    pub coefficient: Vec<usize>,
}

pub struct Point {
    pub value: Vec<(f64, f64)>,
}

impl Polynomial {
    pub fn evaluate(&self, x: usize) -> usize {
        self.coefficient
            .iter()
            .enumerate()
            .map(|(index, coeff)| coeff * x.pow(index.try_into().unwrap()))
            .sum()
    }

    fn degree(&self) -> usize {
        self.coefficient.len()
    }
}

impl Point {
    pub fn lagrange_interpolation(&self) -> Vec<f64> {
        let n: usize = self.value.len();
        let mut coefficient = vec![0.0; n];

        for (_i, &(x_i, _y_i)) in self.value.iter().enumerate() {
            let mut denominator = 1.0;
            let mut basis_poly = vec![1.0];

            for (_j, &(x_j, _)) in self.value.iter().enumerate() {
                if x_i != x_j {
                    denominator *= (x_i - x_j) as f64;
                    basis_poly = multiply(&basis_poly, vec![-x_j as f64, 1.0]);
                }
            }

            // Scale the basis polynomial by y_i / denom
            let scaled_poly = scale(&basis_poly, (_y_i as f64) / denominator);

            // Add the scaled polynomial to the result
            coefficient = add_poly(&coefficient, &scaled_poly);
        }

        coefficient
    }
}

fn multiply(poly1: &[f64], poly2: Vec<f64>) -> Vec<f64> {
    let mut result = vec![0.0; poly1.len() + poly2.len() - 1];
    for (i, &coeff1) in poly1.iter().enumerate() {
        for (j, &coeff2) in poly2.iter().enumerate() {
            result[i + j] += coeff1 * coeff2;
        }
    }
    result
}

fn scale(poly1: &[f64], x: f64) -> Vec<f64> {
    poly1.iter().map(|&coefficient| coefficient * x).collect()
}

fn add_poly(poly1: &[f64], poly2: &[f64]) -> Vec<f64> {
    let len = usize::max(poly1.len(), poly2.len());
    let mut result = vec![0.0; len];
    for i in 0..len {
        let coeff1 = if i < poly1.len() { poly1[i] } else { 0.0 };
        let coeff2 = if i < poly2.len() { poly2[i] } else { 0.0 };
        result[i] = coeff1 + coeff2;
    }
    result
}

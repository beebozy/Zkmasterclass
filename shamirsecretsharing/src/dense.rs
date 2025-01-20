#[warn(dead_code)]


pub struct Polynomial {
    pub coefficient: Vec<f64>,
}

pub struct Point {
    pub value: Vec<(f64, f64)>,
}

impl Polynomial {

    pub fn new(coefficient:Vec<f64>)->Self{
        Self{coefficient}
    }
    pub fn evaluate(&self, x: f64) -> f64 {
        self.coefficient
            .iter()
            .enumerate()
            .map(|(index, coeff)| coeff * x.powf(index as f64))
            .sum()
    }

    fn degree(&self) -> usize {
        self.coefficient.len()-1
    }
}

impl Point {

    pub fn new(value:Vec<(f64, f64)>)->Self{
        Self{value}
    }

    pub fn interpolate(&self) -> Vec<f64> {
        let n: usize = self.value.len();

        // Start with an empty coefficient vector of size `n`.
        let mut coefficient = vec![0.0; n];

        // Iterate over each point `(x_i, y_i)`.
        for (i, &(x_i, y_i)) in self.value.iter().enumerate() {
            let mut denominator = 1.0;
            let mut basis_poly = vec![1.0];

            
            for (j, &(x_j, _)) in self.value.iter().enumerate() {
                if i != j {
                    denominator *= x_i - x_j;
                    basis_poly = multiply(&basis_poly, vec![-x_j, 1.0]);
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
fn multiply(poly1: &[f64], poly2: Vec<f64>) -> Vec<f64> {
    let mut result = vec![0.0; poly1.len() + poly2.len() - 1];
    for (i, &coeff1) in poly1.iter().enumerate() {
        for (j, &coeff2) in poly2.iter().enumerate() {
            result[i + j] += coeff1 * coeff2;
        }
    }
    result
}

/// Scale a polynomial by a constant.
fn scale(poly: &[f64], x: f64) -> Vec<f64> {


    poly.iter().map(|&coefficient| coefficient * x).collect()
}

/// Add two polynomials.
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


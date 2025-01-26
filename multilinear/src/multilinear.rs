use ark_ff::Field;
use ark_ff::PrimeField;

struct Multilinear<Fq: PrimeField> {
    values: Vec<Fq>,
}

impl<Fq: PrimeField> Multilinear<Fq> {
    pub fn new(values: &Vec<Fq>) -> Self {
        if !values.len().is_power_of_two() {
            panic!("Invalid input: the number of values must be a power of two");
        }
        Self {
            values: values.to_vec(),
        }
    }

    fn get_flipped_or_bit(&self, position_to_flip: usize, number_to_flip: usize) -> usize {
        let power = self.values.len().ilog2() as usize - 1 - position_to_flip;
        number_to_flip | (1 << power)
    }

    pub fn partial_evaluate(&self, values: (Fq, usize)) -> Self {
        // The formula to partially evaluate is: y1 + (y2 - y1) * r
        let n = self.values.len();
        let m = n / 2;

        let mut new_values = Vec::with_capacity(m);

        let target = 1 << (self.values.len().ilog2() as usize - 1 - values.1);

        let mut first_index = 0;

        for _ in 0..m {
            let y1 = self.values[first_index];
            let last_index = self.get_flipped_or_bit(values.1, first_index);
            let y2 = self.values[last_index];

            // Interpolate and evaluate
            let result = y1 + (y2 - y1) * values.0;
            new_values.push(result);

            first_index = if (first_index + 1) % target == 0 {
                last_index + 1
            } else {
                first_index + 1
            };
        }

        Self::new(&new_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq; // Using BN254 scalar field as an example
    use ark_ff::UniformRand;

    #[test]
#[should_panic(expected = "Invalid input: the number of values must be a power of two")]
fn test_multilinear_new_invalid_input() {
    // Input length is not a power of two (should panic)
    let coeff = vec![Fq::from(1), Fq::from(2), Fq::from(3)]; // Length = 3 (not a power of two)
    let _ = Multilinear::new(&coeff);
}
}
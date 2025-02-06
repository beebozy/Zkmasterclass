use ark_ff::PrimeField;
use ark_bn254::Fr; // Example finite field
use std::marker::PhantomData;

enum Operations {
    Add,
    Mul,
}

struct Gate<F: PrimeField> {
    left_input: usize,
    right_input: usize,
    output: usize,
    ops: Operations,
    _field: PhantomData<F>, // This prevents the unused type parameter error
}

impl<F: PrimeField> Gate<F> {
    fn new(left_input: usize, right_input: usize, output: usize, ops: Operations) -> Self {
        Self {
            left_input,
            right_input,
            output,
            ops,
            _field: PhantomData, // Required to satisfy the generic parameter
        }
    }

    fn evaluate(&self, inputs: &mut Vec<F>) {
        let result = match self.ops {
            Operations::Add => inputs[self.left_input] + inputs[self.right_input],
            Operations::Mul => inputs[self.left_input] * inputs[self.right_input],
        };

        if self.output >= inputs.len() {
            inputs.push(result);
        } else {
            inputs[self.output] = result;
        }
    }
}

struct Layer<F: PrimeField> {
    gates: Vec<Gate<F>>,
}

impl<F: PrimeField> Layer<F> {
    fn new(gates: Vec<Gate<F>>) -> Self {
        Self { gates }
    }

    fn evaluate(&self, inputs: &mut Vec<F>) {
        for gate in &self.gates {
            gate.evaluate(inputs);
        }
    }
}

struct Circuit<F: PrimeField> {
    layers: Vec<Layer<F>>,
}

impl<F: PrimeField> Circuit<F> {
    fn new(layers: Vec<Layer<F>>) -> Self {
        Self { layers }
    }

    fn evaluate(&self, mut input_elements: Vec<F>) -> Vec<F> {
        for layer in &self.layers {
            layer.evaluate(&mut input_elements);
        }
        input_elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let a = Fr::from(1u64);
        let b = Fr::from(2u64);
        let c = Fr::from(3u64);
        let d = Fr::from(4u64);
        let  input_elements = vec![a, b, c, d];

        // First layer: Multiplication and Addition
        let gate1 = Gate::<Fr>::new(0, 1, 4, Operations::Mul); // 1 * 2 = 2 → index 4
        let gate2 = Gate::<Fr>::new(2, 3, 5, Operations::Add); // 3 + 4 = 7 → index 5

        let layer1 = Layer::new(vec![gate1, gate2]);

        // Second layer: Addition
        let gate3 = Gate::<Fr>::new(4, 5, 6, Operations::Add); // 2 + 7 = 9 → index 6

        let layer2 = Layer::new(vec![gate3]);

        let circuit = Circuit::new(vec![layer1, layer2]);

        let result = circuit.evaluate(input_elements);

        assert_eq!(result[6], Fr::from(9u64));
    }
}

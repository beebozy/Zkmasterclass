use ark_ff::PrimeField;
use ark_bn254::Fr; // Example finite field
use std::{collections::HashSet, marker::PhantomData};

#[derive(Debug)]
enum Operations {
    Add,
    Mul,
}
#[derive(Debug)]
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

    fn evaluate(&self, inputs: &mut Vec<F>) ->Vec<F> {
        let result = match self.ops {
            Operations::Add => inputs[self.left_input] + inputs[self.right_input],
            Operations::Mul => inputs[self.left_input] * inputs[self.right_input],
        };

        if self.output >= inputs.len() {
            inputs.push(result);
        } else {
            inputs[self.output] = result;
        }
        inputs.clone()
    }
}
#[derive(Debug)]
struct Layer<F: PrimeField> {
    gates: Vec<Gate<F>>,
}

//struct Polynomial <F:PrimeField>;
/*{
    evaluation: Vec<F>
}*/


impl<F: PrimeField> Layer<F> {
    fn new(gates: Vec<Gate<F>>) -> Self {
        Self { gates }
    }

    fn evaluate(&self, inputs: &mut Vec<F>)->Vec<F> {
       
        for gate in &self.gates {
           gate.evaluate(inputs);
        }

        inputs.clone()
    }
}

struct Circuit<F: PrimeField> {
    layers: Vec<Layer<F>>,
    add_gates: HashSet<(usize, usize, usize)>,
    mul_gates: HashSet<(usize,usize,usize)>
}

impl<F: PrimeField> Circuit<F> {
    fn new(layers: Vec<Layer<F>>) -> Self {
        let mut add_gates = HashSet::new();
        let mut mul_gates = HashSet::new();

        // Populate gate sets
        for layer in &layers {
            for gate in &layer.gates {
                let key = (gate.output, gate.left_input, gate.right_input);
                match gate.ops {
                    Operations::Add => add_gates.insert(key),
                    Operations::Mul => mul_gates.insert(key),
                };
            }
        }

        Self {
            layers,
            add_gates,
            mul_gates,
        }
    }

    fn add_i(&self, output: usize, left: usize, right: usize) -> F {
        if self.add_gates.contains(&(output, left, right)) {
            F::one()
        } else {
            F::zero()
        }
    }

    
    fn mul_i(&self, output: usize, left: usize, right: usize) -> F {
        if self.mul_gates.contains(&(output, left, right)) {
            F::one()
        } else {
            F::zero()
        }
    }


    fn evaluate(&self, mut input_elements: Vec<F>) -> Vec<Vec<Vec<F>>> {
        let mut layer_outputs = Vec::new();

        // Store initial inputs as the first entry
        layer_outputs.push(vec![input_elements.clone()]);

        for (_, layer) in self.layers.iter().enumerate() {
            let prev_len = input_elements.len(); // Track the size before evaluation

            layer.evaluate(&mut input_elements);

            // Extract only newly computed values
            let new_values = input_elements[prev_len..].to_vec();
        

            layer_outputs.push(vec![new_values]); // Store as a separate layer output
        }

        layer_outputs // Return structured output
    }


    // fn add_polynomial_or_mulpolynomial(&self)-bool{
    //     let n = self.len();


    // }
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
        let gate1 = Gate::<Fr>::new(0, 1, 0, Operations::Mul); // 1 * 2 = 2 → index 4
        let gate2 = Gate::<Fr>::new(2, 3, 1, Operations::Add); // 3 + 4 = 7 → index 5

        let layer1 = Layer::new(vec![gate1, gate2]);

        println!("{:?}",&layer1);

        // Second layer: Addition
        let gate3 = Gate::<Fr>::new(0, 1, 0, Operations::Add); // 2 + 7 = 9 → 

        let layer2 = Layer::new(vec![gate3]);

        let circuit = Circuit::new(vec![layer1, layer2]);

        let result = circuit.evaluate(input_elements);
        println!("{:?}", &result);
        
    }

    #[test]
    fn test_addi_and_muli(){

        let a = Fr::from(1u64);
        let b = Fr::from(2u64);
        let c = Fr::from(3u64);
        let d = Fr::from(4u64);
        let  input_elements = vec![a, b, c, d];

        // First layer: Multiplication and Addition
        let gate1 = Gate::<Fr>::new(0, 1, 0, Operations::Mul); // 1 * 2 = 2 → index 4
        let gate2 = Gate::<Fr>::new(2, 3, 1, Operations::Add); // 3 + 4 = 7 → index 5

        let layer1 = Layer::new(vec![gate1, gate2]);

        println!("{:?}",&layer1);

        // Second layer: Addition
        let gate3 = Gate::<Fr>::new(0, 1, 0, Operations::Add); // 2 + 7 = 9 → 

        let layer2 = Layer::new(vec![gate3]);

        let circuit = Circuit::new(vec![layer1, layer2]);

        assert_eq!(circuit.mul_i(0, 1, 0),Fr::from(1));
        assert_eq!(circuit.add_i(1, 2, 3),Fr::from(1));
        let result = circuit.evaluate(input_elements);
        println!("{:?}", &result);


    }
}

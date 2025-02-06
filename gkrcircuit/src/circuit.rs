
use ark_ff::PrimeField::{Zero, One};

use ark_bn254;

//
enum Operations{
    Add, 
    Mul
}

struct Gate<F:PrimeField>{

    leftInput: u64,
    rightInput: u64, 
    output:u64,
    ops: Operations
}

struct Layers<F:PrimeField>{
    gates: Vec<Gate>, 
    
}

struct Circuit<F:PrimeField>{
    layers: Vec<Layers>,
    //output: Vec<F>
}

impl Gate<F: PrimeField>{
    fn new(leftInput:u64, rightInput:u64, output:F, ops: Operations)->Self{
        Self{leftInput, rightInput, output, ops}
    }

    fn evaluate(&self, inputs: Vec<F>)->F{
        match self.ops{
            Operations::Add =>inputs[self.0] + inputs[self.1],
            Operations::Mul =>inputs[self.0] *inputs[self.1]
            
        }
    }
}

impl Layers<F:PrimeField>{

    fn new(gates: Vec<Gate<f>>)->Self{
        
       Self{gates}
    }

    fn evaluate(& mut self, input_element: Vec<F>)->Vec<F>{
       // self::new();
             self.gates().iter().map(|gate| gate.evaluate(input_element)).collect();
// this should actually step by 2
// pick any two indexes


}

impl Circuit<F:PrimeField>{
    fn new(layers: Vec<Layers>)->Self{
        Self{layers}
    }

    fn evaluate(&self, input_element:Vec<F>)->Vec<F>{

        //let new_circuit= Circuit::new();

        let values= input_element;
        
        for layers in self.layers.iter(){
       
            let result = Vec::new();

            for new_gate in layers.gates.iter(){
               result.push(values[new_gate].evaluate());
                
            }

           // println!("The result for this layer is {}", result) 
            values= result;

        }

        values


        /*layers.gate([input element]).map(|| {


        }) */
    }

}
}



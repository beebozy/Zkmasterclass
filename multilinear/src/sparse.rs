
use ark_ff::Field;
use ark_ff::PrimeField;
struct Polynomial<F:PrimeField >{
    coefficeint: Vec<F>,
    exponent:Vec<(F,F)>
}


impl Polynomial<F:PrimeField>{
    fn new(coefficeint:Vec<F>, exponent:Vec<(F,F)>)->Self{

        Self{coefficeint, exponent};
    }

    fn complete_evaluation(&self, values: Vec<F,F>)->F{

       // exponent.iter().next().map(||)
        // I need to iterate each values in the exponennt and multiply it with a corresponding value and at the end sum all together
        
        // let result= 0;
        // for &(a,b) in self.exponent.len(){

        //     self.coefficeint[i]*= (a * values[i] + b*values[i])

        // }

        // //let result
        // let sum= 0;

        // self.iter().next().enumerate().map((|i, &()|))
        
        // for i in 0 ..exponent.len(){
             
        //     sum+= values[i] * exponent[i];


        // }

    }

}
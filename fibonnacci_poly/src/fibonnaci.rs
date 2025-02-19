use crate::dense::{Point,Polynomial};
use ark_ff::PrimeField;
use ark_bn254::Fq;
use  ark_ff::{self, Field};

fn interpolate(point: crate::dense::Point<Fq>)->Vec<Fq>{
    point.interpolate()
}

fn evaluate(polynomial:crate::dense::Polynomial<Fq>, x:Fq)->Fq{
    polynomial.evaluate(x)
}


#[cfg(test)]
mod tests{

use super::*;

use crate::dense::{Point, Polynomial};
#[test]
fn test_fibonacci_recurrence_relation() {
    // Define Fibonacci points as tuples (x, y) over the field Fq.
    let points = vec![
        (Fq::from(0), Fq::from(1)), // F(0) = 1
        (Fq::from(1), Fq::from(1)), // F(1) = 1
        (Fq::from(2), Fq::from(2)), // F(2) = 2
        (Fq::from(3), Fq::from(3)), // F(3) = 3
        (Fq::from(4), Fq::from(5)), // F(4) = 5
    ];


    
    let point = Point::new(points);

    
    let interpolated_polynomial = point.interpolate();

    let evaluated_polynomial= Polynomial{
        coefficient: interpolated_polynomial
    };
    
   assert_eq!(evaluated_polynomial.evaluate(Fq::from(4)), evaluated_polynomial.evaluate(Fq::from(2))+ evaluated_polynomial.evaluate(Fq::from(3)));
   
    }
}


fn main(){

}
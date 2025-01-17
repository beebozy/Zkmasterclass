use std::ops::Index;

pub mod dense;
pub mod sparse;
#[warn(dead_code)]


// struct Polynomial{
// coefficient :Vec<usize>
// }

//  fn multiply(x: usize, length:usize)->usize{

// let mut result=1;

// for i in 0..length{
//     result*=x;
// }

// result
//  }
// // }

// impl Polynomial{
// fn evaluate(&self, x:usize)->usize{
//     self.coefficient.iter().enumerate().map(|(index, coeff)| coeff*x.pow(index.try_into().unwrap())).sum()

    
// }

// fn degree(&self)->usize{

//     let n: usize= self.coefficient.len();
    
//     n
// }
// }




fn main() {
   


    // let polynomial  =dense::Polynomial{
    //     coefficient: vec![3,5,2]
    // };

    let point= dense::Point{
        value: vec![(1.0,1.0),(2.0,4.0),(3.0,9.0)]
    };
    let poly= sparse::Univariate{
        polynomial: vec![(2,3),(1,5)]
    };
    // let result= polynomial.evaluate(5);

    // let degree:usize =polynomial.degree();

        let lagrange_reps= point.lagrange_interpolation();
    let eval= poly.evaluate(5);
    let degree:usize = poly.degree();
    print!("The evaluation is{} and the degree {} ", eval, degree);

    print!("The langrange interpolation is {:?}", lagrange_reps);


}

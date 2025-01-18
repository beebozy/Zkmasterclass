/* use std::ops::Index;

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
        value: vec![(1.0, 2.0), (2.0, 4.0)],
        
    };
    // let poly= sparse::Univariate{
    //     polynomial: vec![(2,3),(1,5)]
    // };

    let polynomial= dense::Polynomial{
        coefficient:vec![1,2,3]
    };
    // let result= polynomial.evaluate(5);

    // let degree:usize =polynomial.degree();

        let lagrange_reps= point.interpolate();
    // let eval= poly.evaluate(5);
    // let degree:usize = poly.degree();
    let eval= polynomial.evaluate(5);
    print!("The evaluation is{}  ", eval);

    print!("The langrange interpolation is {:?}", lagrange_reps);


}
 */


 use std::ops::{Add, Mul, Sub, Div};
use num_traits::{FromPrimitive, Pow};

#[derive(Debug, PartialEq)]
struct Polynomial<T> {
    coefficients: Vec<T>,
}

#[derive(Debug, PartialEq)]
struct Points<T> {
    values: Vec<(T, T)>,
}

impl<T> Polynomial<T>
where
     T: Copy + Add<Output = T> + Mul<Output = T> + From<u32> + Pow<u32, Output = T> + Default,
     {
    fn new(coefficients: Vec<T>) -> Self {
        Self { coefficients }
    }

    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    fn evaluate(&self, x: T) -> T {
        self.coefficients
            .iter()
            .enumerate()
            .fold(T::default(), |acc, (i, &coeff)| acc + coeff * x.pow(i as u32))
    }
}

impl<T> Points<T>
where
    T: Copy
        + Add<Output = T>
        + Mul<Output = T>
        +Sub<Output = T>
        +Div<Output = T>
        + Pow<u32, Output = T>
        +From<u32>
        + Default
        + PartialEq
        + std::ops::Neg<Output = T>,
{
    fn new(values: Vec<(T, T)>) -> Self {
        Self { values }
    }

    fn interpolate(&self) -> Polynomial<T> {
        let n = self.values.len();
        let mut result = Polynomial::new(vec![T::default()]);

        for i in 0..n {
            let (xi, yi) = self.values[i];
            let mut numerator = Polynomial::new(vec![T::from(0)]);
            let mut denominator = T::from(1);

            for j in 0..n {
                if i != j {
                    let (xj, _) = self.values[j];
                    numerator = numerator * Polynomial::new(vec![-xj, T::from(1)]);
                    denominator = denominator * (xi - xj);
                }
            }

            let term = numerator * Polynomial::new(vec![yi / denominator]);
            result = result + term;
        }

        result
    }
}

// Polynomial addition
impl<T> Add for Polynomial<T>
where
T: Copy + Add<Output = T> + Mul<Output = T> + Default + std::convert::From<u32> + num_traits::Pow<u32, Output = T>
{
    type Output = Polynomial<T>;

    fn add(self, other: Self) -> Self::Output {
        let max_len = usize::max(self.coefficients.len(), other.coefficients.len());
        let mut result = vec![T::default(); max_len];

        for (i, &coeff) in self.coefficients.iter().enumerate() {
            result[i] = result[i] + coeff;
        }

        for (i, &coeff) in other.coefficients.iter().enumerate() {
            result[i] = result[i] + coeff;
        }

        Polynomial::new(result)
    }
}

// Polynomial multiplication
impl<T> Mul for Polynomial<T>
where
T: Copy + Add<Output = T> + Mul<Output = T> + Default + num_traits::Pow<u32, Output = T> + std::convert::From<u32>
{
    type Output = Polynomial<T>;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = vec![T::default(); self.coefficients.len() + other.coefficients.len() - 1];

        for (i, &a) in self.coefficients.iter().enumerate() {
            for (j, &b) in other.coefficients.iter().enumerate() {
                result[i + j] = result[i + j] + a * b;
            }
        }

        Polynomial::new(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_degree() {
        let poly: Polynomial<u32> = Polynomial::new(vec![2, 3, 5]);
        assert_eq!(poly.degree(), 2);
    }

    #[test]
    fn test_polynomial_evaluation() {
        let poly: Polynomial<u32> = Polynomial::new(vec![2, 3, 5]);
        assert_eq!(poly.evaluate(1), 10);
        assert_eq!(poly.evaluate(2), 28);
    }

    #[test]
    fn test_polynomial_interpolation() {
        let points: Points<u32> = Points{
            values: vec![(1,2),(2,4),(8,9)]
        };

    }
        /*let interpolated= points.interpolate();
        // let interpolated = points.interpolate();

        // assert_eq!(interpolated,Polynomial::new(vec![0,0,2]));
        // You can add an expected Polynomial result here for verification.
    }*/
} 
fn main(){}

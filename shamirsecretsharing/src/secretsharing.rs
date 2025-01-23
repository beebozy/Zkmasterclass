


use crate::dense::{Polynomial, Point};
use ark_bn254::Fq;
use ark_ff::Field;
use  ark_ff;
use std::result;
use ark_std::rand::rngs::StdRng;

use rand::{Rng, SeedableRng};





fn generate_secret_polynomial(secret: Fq, threshold: usize, x: Fq) -> Vec<Fq> {
    // Create a vector to hold the points
    let mut result: Vec<(Fq, Fq)> = Vec::new();

    // Add the secret as the first point
    result.push((x, secret));

    let mut rng =StdRng::from_entropy();
    // Generate random points
    for _ in 0..threshold -1 {
        let random_x: Fq = rng.gen();
        let random_y: Fq= rng.gen();
        result.push((random_x, random_y));
    }

    // Create a Point instance and interpolate
    let point = crate::dense::Point::new(result);
    let answer: Vec<Fq> = point.interpolate();

    // Return the interpolated coefficients
    answer
}

fn generate_sharing_point(coefficient: crate::dense::Polynomial<Fq>, threshold: usize, total_no_of_point:usize)->crate::dense::Point<Fq>{
    let mut answer: Vec<(Fq,Fq)>= Vec::new();

    let mut rng:StdRng= StdRng::from_entropy();
    for i in 0 ..total_no_of_point{
        let random_x: Fq= rng.gen();
        let  y:Fq= coefficient.evaluate(random_x );
        answer.push((random_x , y));

    }

    crate::dense::Point::new(answer)
}

fn recreate_polynomial(threshold: usize, points: crate::dense::Point<Fq>) -> Vec<Fq> {
    // Collect the points into a vector of tuples (x, y)
    let result: Vec<(Fq, Fq)> = points
        .value
        .iter()
        .take(threshold) // Limit to the threshold number of points
        .map(|&(x, y)| (x, y))
        .collect();

    
    
    let point_result = crate::dense::Point::new(result);


    point_result.interpolate()
}


fn generate_secret(coefficient: crate::dense::Polynomial<Fq>, x:Fq)->Fq{
    coefficient.evaluate(Fq::from(x)) //Generate the secret at a point
}


#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dense::{Point, Polynomial}; // Use the actual `dense` module from the main codebase
    use rand::Rng;

    #[test]
    fn test_generate_secret_polynomial() {
        let secret = Fq::from (42);
        let threshold = 3;
        let x = Fq::from(0);

        let coefficients = generate_secret_polynomial(secret, threshold, x);

        // Ensure the result is a valid polynomial of the expected degree
        assert_eq!(coefficients.len(), threshold);
        assert_eq!(coefficients[0], secret); // The constant term (secret)
    }

    #[test]
    fn test_generate_sharing_point() {
        let coefficients = Polynomial::new(vec![Fq::from(42),Fq::from( 5),Fq::from(3)]);
        let threshold = 3;
        let total_no_of_points = 5;

        let points = generate_sharing_point(coefficients, threshold, total_no_of_points);

        // Verify the number of points generated
        assert_eq!(points.value.len(), total_no_of_points);

        // Validate the points
        // for (x, y) in points.value {
        //     let expected_y = coefficients.evaluate(x);
        //     assert_eq!(y, expected_y);
        // }
    }

    #[test]
    fn test_recreate_polynomial() {
        let threshold = 3;
        let points = Point::new(vec![(Fq::from(0), Fq::from(42)), (Fq::from(1), Fq::from(50)), (Fq::from(2), Fq::from(68))]);

        let coefficients = recreate_polynomial(threshold, points);

        // Verify the result contains the correct number of coefficients
        assert_eq!(coefficients.len(), threshold);

        // Example check for interpolation correctness (replace with actual checks based on logic)
        assert_eq!(coefficients[0], Fq::from(42));
    }

    #[test]
    fn test_generate_secret() {
        let coefficients = Polynomial::new(vec![Fq::from(42),Fq::from(5), Fq::from(3)]);
        let x = Fq::from(2);

        let secret = generate_secret(coefficients, x);

        // Verify the result matches the polynomial evaluation
        let expected = Fq::from(42) + Fq::from(5) * x  + Fq::from(3) * x .pow(&[2 as u64]);
        assert_eq!(secret, expected);
    }
}




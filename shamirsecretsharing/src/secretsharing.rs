


use crate::dense::{Polynomial, Point};
use std::result;

use rand::{thread_rng, Rng};





fn generate_secret_polynomial(secret: f64, threshold: usize, x: f64) -> Vec<f64> {
    // Create a vector to hold the points
    let mut result: Vec<(f64, f64)> = Vec::new();

    // Add the secret as the first point
    result.push((x, secret));

    // Generate random points
    for _ in 0..threshold -1 {
        let random_x = rand::thread_rng().gen_range(0..100) as f64;
        let random_y = rand::thread_rng().gen_range(0..100) as f64;
        result.push((random_x, random_y));
    }

    // Create a Point instance and interpolate
    let point = crate::dense::Point::new(result);
    let answer = point.interpolate();

    // Return the interpolated coefficients
    answer
}

fn generate_sharing_point(coefficient: crate::dense::Polynomial, threshold: usize, total_no_of_point:usize)->crate::dense::Point{
let mut answer: Vec<(f64,f64)>= Vec::new();
    for i in 0 ..total_no_of_point{
        let random_x= rand::thread_rng().gen_range(0..100);
        let y= coefficient.evaluate(random_x as f64);
        answer.push((random_x as f64, y));

    }

    crate::dense::Point::new(answer)
}

fn recreate_polynomial(threshold: usize, points: crate::dense::Point) -> Vec<f64> {
    // Collect the points into a vector of tuples (x, y)
    let result: Vec<(f64, f64)> = points
        .value
        .iter()
        .take(threshold) // Limit to the threshold number of points
        .map(|&(x, y)| (x, y))
        .collect();

    
    
    let point_result = crate::dense::Point::new(result);


    point_result.interpolate()
}


fn generate_secret(coefficient: crate::dense::Polynomial, x:usize)->f64{
    coefficient.evaluate(x as f64) //Generate the secret at a point
}


#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dense::{Point, Polynomial}; // Use the actual `dense` module from the main codebase
    use rand::Rng;

    #[test]
    fn test_generate_secret_polynomial() {
        let secret = 42.0;
        let threshold = 3;
        let x = 0.0;

        let coefficients = generate_secret_polynomial(secret, threshold, x);

        // Ensure the result is a valid polynomial of the expected degree
        assert_eq!(coefficients.len(), threshold);
        assert_eq!(coefficients[0], secret); // The constant term (secret)
    }

    #[test]
    fn test_generate_sharing_point() {
        let coefficients = Polynomial::new(vec![42.0, 5.0, 3.0]);
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
        let points = Point::new(vec![(0.0, 42.0), (1.0, 50.0), (2.0, 68.0)]);

        let coefficients = recreate_polynomial(threshold, points);

        // Verify the result contains the correct number of coefficients
        assert_eq!(coefficients.len(), threshold);

        // Example check for interpolation correctness (replace with actual checks based on logic)
        assert_eq!(coefficients[0], 42.0);
    }

    #[test]
    fn test_generate_secret() {
        let coefficients = Polynomial::new(vec![42.0, 5.0, 3.0]);
        let x = 2;

        let secret = generate_secret(coefficients, x);

        // Verify the result matches the polynomial evaluation
        let expected = 42.0 + 5.0 * x as f64 + 3.0 * (x as f64).powi(2);
        assert_eq!(secret, expected);
    }
}




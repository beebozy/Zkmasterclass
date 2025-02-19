use ark_ff::PrimeField;
use ark_bn254::Fq; // Assuming you use Fq as your field element.
use crate::transcript::Transcript;
use std::ops::Add;
use multilinear::multilinear::Multilinear;

/// A proof consists of the claimed sum and a vector of round polynomials (each round a pair).

#[warn(dead_code)]
struct Proof<F: PrimeField> {
    claimed_sum: F,
    round_poly: Vec<[F; 2]>,
}


fn prove<F: PrimeField>(poly: &Multilinear<F>, claimed_sum: F) -> Proof<F> {
    let mut transcript = Transcript::new();

    // Append the initial polynomial representation.
    let poly_bytes: Vec<u8> = poly
        .iter()
        .flat_map(|f| f.into_big_int().to_byte_be())
        .collect();
    transcript.append(poly_bytes.as_slice());
    transcript.append(&claimed_sum.into_big_int().to_byte_be());

    let mut current_poly = poly.clone();
    let mut round_polys = Vec::new();

    // Assume number of rounds is log2 of the evaluation vector length.
    let rounds = (current_poly.evaluate.len() as f64).log2() as usize;
    for _ in 0..rounds {
        // Compute the round polynomial as a pair:
        let round_poly = [
            current_poly.partial_evaluate(0, F::zero()),
            current_poly.partial_evaluate(0, F::one()),
        ];
        round_polys.push(round_poly);

        // Append round polynomial to transcript.
        let round_poly_bytes: Vec<u8> = round_poly
            .iter()
            .flat_map(|f| f.into_big_int().to_byte_be())
            .collect();
        transcript.append(round_poly_bytes.as_slice());

        // Derive the next challenge.
        let challenge = transcript.squeeze();
        
        
        current_poly = current_poly.evaluate([Some(challenge)]);
    }

    Proof {
        claimed_sum,
        round_poly: round_polys,
    }
}


fn verifier<F: PrimeField>(poly: &Multilinear<F>, proof: &Proof<F>) -> bool {
    let mut transcript = Transcript::new();
    let mut challenges: Vec<F> = Vec::new();
    let mut current_poly= poly.clone();
    // Append the initial polynomial representation.
    let poly_bytes: Vec<u8> = poly
        .iter()
        .flat_map(|f| f.into_big_int().to_byte_be())
        .collect();
    transcript.append(poly_bytes.as_slice());
    transcript.append(&proof.claimed_sum.into_big_int().to_byte_be());

    for round_poly in &proof.round_poly {
        
        let round_poly_bytes: Vec<u8> = round_poly
            .iter()
            .flat_map(|f| f.into_big_int().to_byte_be())
            .collect();
        transcript.append(round_poly_bytes.as_slice());

        // Get the challenge for this round.
        let challenge = transcript.squeeze();
        challenges.push(challenge);
        poly = current_poly.evaluate(Some[challenge]);

       // let new_claimed_sum = round_poly[0] + challenge * (round_poly[1] - round_poly[0]);
        // Here you might check that `new_claimed_sum` is consistent with your protocol.
        // (This check is left as a placeholder.)
    }

    // Final check: the evaluation of `poly` on the vector of challenges should equal the final claimed sum.
    if proof.claimed_sum != poly.evaluate(Some[challenges.iter()]) {
        return false;
    }
    true
}



#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use ark_ff::Zero;
    use multilinear::multilinear::Multilinear;
    use transcript::Transcript;

    
    #[test]
    fn test_challenges_and_evaluation_consistency() {
        
        let evaluations = vec![
            Fq::from(1u64),
            Fq::from(2u64),
            Fq::from(3u64),
            Fq::from(4u64),
        ];
        // We assume that Multilinear::new constructs the polynomial from its evaluations.
        let poly = Multilinear::new(evaluations);

    
      //  let claimed_sum = poly.iter().fold(Fq::zero(), |acc, f| acc + *f);

      //  let claimed_sum = Fq::from(99); This should fail 
    let claimed_sum = Fq::from(10u64);
        // this should fail but it is not .. 
        let mut transcript = Transcript::new();
        let poly_bytes: Vec<u8> = poly
            .iter()
            .flat_map(|f| f.into_big_int().to_byte_be())
            .collect();
        transcript.append(poly_bytes.as_slice());
        transcript.append(&claimed_sum.into_big_int().to_byte_be());

        
        let mut expected_challenges = Vec::new();
        for round_poly in proof.round_poly.iter() {
            let round_poly_bytes: Vec<u8> = round_poly
                .iter()
                .flat_map(|f| f.into_big_int().to_byte_be())
                .collect();
            transcript.append(round_poly_bytes.as_slice());
            let challenge = transcript.squeeze();
            expected_challenges.push(challenge);
        }

       
        let mut current_poly = poly.clone();
        for challenge in expected_challenges.iter() {

            current_poly = current_poly.evaluate([Some(*challenge)]);
        }

       
        let final_value = current_poly.evaluate(Vec::<Option<Fq>>::new());
        assert_eq!(
            proof.claimed_sum, final_value
        );

        // --- Finally, check that the verifier accepts the proof ---
        assert!(
            verifier(&poly, &proof),
            "Verifier did not accept a valid proof."
        );
    }
}

use sha3::{Digest, Keccak256};
use ark_ff::PrimeField;

#[warn(dead_code)]
pub struct Transcript<K: HashTrait, F: PrimeField> {
    hash_function: K,
    _field: std::marker::PhantomData<F>,
}

impl<K: HashTrait, F: PrimeField> Transcript<K, F> {
  pub   fn new(hash_function: K) -> Self {
        Self {
            hash_function,
            _field: std::marker::PhantomData,
        }
    }

   pub  fn absorb(&mut self, data: &[u8]) {
        self.hash_function.append(data);
    }

   pub  fn squeeze(&mut self) -> F {
        let new_hash = self.hash_function.absorb();
        F::from_be_bytes_mod_order(&new_hash)
    }
}

 pub trait HashTrait {
    fn append(&mut self, data: &[u8]);
    fn absorb(&mut self) -> Vec<u8>;
}

impl HashTrait for Keccak256 {
    fn append(&mut self, data: &[u8]) {
        self.update(data);
    }

    fn absorb(&mut self) -> Vec<u8> {
        self.clone().finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use ark_ff::PrimeField;

    #[test]
    fn test_keccak256() {
        let hasher = Keccak256::default();
        let mut transcript: Transcript<Keccak256, Fq> = Transcript::new(hasher);

        transcript.absorb(b"hello, world!");
        let result = transcript.squeeze();

        // Ensure that the result is a valid field element

        // assert!(result.is_valid());

        println!("The hash result is {}" , result);
        // Debug print the result
        dbg!(result);
    }
}

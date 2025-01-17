use std::u16::MAX;


pub struct Univariate{
   pub  polynomial: Vec<(usize, usize)>
}

impl Univariate{
  pub   fn evaluate(&self, x:usize)->usize{
        self.polynomial.iter().map(|(degree, coeff)| coeff*x.pow(*degree as u32)).sum()
    }

  pub  fn degree(&self)->usize{
   
    self.polynomial.iter().map(|(degree,_coeff)| *degree).max().unwrap_or(0)
   
  
   }
}


// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::ops::Add;
use std::cmp::PartialEq;

// #[derive(Copy, Clone)]
#[derive(Copy, Clone, Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
   type Output = Self;
   
   fn add(self, other: Self) -> Self {
     let op1: u64 = self.value as u64;
     let op2: u64 = other.value as u64;
     let max: u64 = u32::MAX as u64;
     let mut result: u64 = op1 + op2;
     if result > max {
          result = result - max - 1;
     }
     Self {
       value: result as u32
     }
   }
}

impl PartialEq for WrappingU32 {
  fn eq(&self, other:&Self) -> bool {
    self.value == other.value
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}

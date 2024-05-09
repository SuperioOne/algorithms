#![allow(non_camel_case_types)]
pub mod hash_fn;

use self::hash_fn::{pearson_hash_u16, pearson_hash_u32, pearson_hash_u64, pearson_hash_u8};
use super::{HashFunc, HashStreamFunc};

pub struct PearsonHash_8 {
  state: u8,
}

pub struct PearsonHash_16 {
  state: u16,
}

pub struct PearsonHash_32 {
  state: u32,
}

pub struct PearsonHash_64 {
  state: u64,
}

pub struct PearsonHash_Usize {
  state: usize,
}

impl HashFunc for PearsonHash_8 {
  type Output = u8;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    pearson_hash_u8(self.state, bytes)
  }
}

impl HashStreamFunc for PearsonHash_8 {
  type Output = u8;

  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u8(self.state, input);
  }

  fn complete(&self) -> Self::Output {
    self.state
  }
}

impl HashFunc for PearsonHash_16 {
  type Output = u16;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    pearson_hash_u16(self.state, bytes)
  }
}

impl HashStreamFunc for PearsonHash_16 {
  type Output = u16;

  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u16(self.state, input);
  }

  fn complete(&self) -> Self::Output {
    self.state
  }
}

impl HashFunc for PearsonHash_32 {
  type Output = u32;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    pearson_hash_u32(self.state, bytes)
  }
}

impl HashStreamFunc for PearsonHash_32 {
  type Output = u32;

  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u32(self.state, input);
  }

  fn complete(&self) -> Self::Output {
    self.state
  }
}

impl HashFunc for PearsonHash_64 {
  type Output = u64;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    pearson_hash_u64(self.state, bytes)
  }
}

impl HashStreamFunc for PearsonHash_64 {
  type Output = u64;

  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u64(self.state, input);
  }

  fn complete(&self) -> Self::Output {
    self.state
  }
}

impl HashFunc for PearsonHash_Usize {
  type Output = usize;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    #[cfg(target_pointer_width = "64")]
    {
      pearson_hash_u64(self.state as u64, bytes) as Self::Output
    }

    #[cfg(target_pointer_width = "32")]
    {
      pearson_hash_u32(self.state as u32, bytes) as Self::Output
    }
  }
}

impl HashStreamFunc for PearsonHash_Usize {
  type Output = usize;

  fn update(&mut self, input: &[u8]) {
    #[cfg(target_pointer_width = "64")]
    {
      self.state = pearson_hash_u64(self.state as u64, input) as Self::Output;
    };

    #[cfg(target_pointer_width = "32")]
    {
      self.state = pearson_hash_u32(self.state as u32, input) as Self::Output;
    };
  }

  fn complete(&self) -> Self::Output {
    self.state
  }
}

impl std::hash::Hasher for PearsonHash_64 {
  fn finish(&self) -> u64 {
    self.complete()
  }

  fn write(&mut self, bytes: &[u8]) {
    self.update(bytes);
  }
}

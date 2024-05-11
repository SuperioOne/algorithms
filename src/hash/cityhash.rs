#![allow(non_camel_case_types)]

use self::hash_fn::{cityhash_32, cityhash_64, cityhash_64_with_seed, K2};
use super::HashFunc;

pub mod hash_fn;

pub struct CityHash_32;

impl CityHash_32 {
  pub fn new() -> Self {
    Self
  }
}

impl Default for CityHash_32 {
  fn default() -> Self {
    Self
  }
}

impl HashFunc for CityHash_32 {
  type Output = u32;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    cityhash_32(bytes)
  }
}

pub enum CityHashSeed {
  Default,
  Custom(u64, u64),
}

impl Default for CityHashSeed {
  fn default() -> Self {
    Self::Default
  }
}

impl From<u64> for CityHashSeed {
  fn from(value: u64) -> Self {
    Self::Custom(K2, value)
  }
}

impl From<u128> for CityHashSeed {
  fn from(value: u128) -> Self {
    Self::Custom((value >> 64) as u64, value as u64)
  }
}

impl From<(u64, u64)> for CityHashSeed {
  fn from(value: (u64, u64)) -> Self {
    Self::Custom(value.0, value.1)
  }
}

#[cfg(target_pointer_width = "64")]
impl From<usize> for CityHashSeed {
  fn from(value: usize) -> Self {
    Self::Custom(K2, value as u64)
  }
}

pub struct CityHash_64 {
  seed: CityHashSeed,
}

impl CityHash_64 {
  pub fn new() -> Self {
    Self {
      seed: CityHashSeed::default(),
    }
  }

  pub fn new_with_seed(seed: CityHashSeed) -> Self {
    Self { seed }
  }
}

impl Default for CityHash_64 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for CityHash_64 {
  type Output = u64;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    match self.seed {
      CityHashSeed::Default => cityhash_64(bytes),
      CityHashSeed::Custom(seed0, seed1) => cityhash_64_with_seed(bytes, seed0, seed1),
    }
  }
}

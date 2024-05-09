#![allow(non_camel_case_types)]

use self::hash_fn::city_hash_32;
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
    city_hash_32(bytes)
  }
}

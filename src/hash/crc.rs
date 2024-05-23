use self::hash_fn::crc32c;
use crate::hash::HashFunc;

pub mod hash_fn;

pub struct Crc32C;

impl Crc32C {
  pub const fn new() -> Self {
    Self
  }
}

impl Default for Crc32C {
  fn default() -> Self {
    Self
  }
}

impl HashFunc for Crc32C {
  type Output = u32;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    crc32c(bytes)
  }
}

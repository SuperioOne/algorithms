use self::hash_fn::{murmurhash3_128, murmurhash3_32};
use super::HashFunc;

pub mod hash_fn;

pub struct Murmur3Hash32 {
  seed: u32,
}

pub struct Murmur3Hash128 {
  seed: u64,
}

impl Murmur3Hash32 {
  pub const fn new() -> Self {
    Self { seed: 0 }
  }
  pub const fn new_with_seed(seed: u32) -> Self {
    Self { seed }
  }
}

impl Murmur3Hash128 {
  pub const fn new() -> Self {
    Self { seed: 0 }
  }

  pub const fn new_with_seed(seed: u64) -> Self {
    Self { seed }
  }
}

impl Default for Murmur3Hash32 {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for Murmur3Hash128 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for Murmur3Hash32 {
  type Output = u32;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    murmurhash3_32(self.seed, bytes)
  }
}

impl HashFunc for Murmur3Hash128 {
  type Output = u128;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    murmurhash3_128(self.seed, bytes)
  }
}

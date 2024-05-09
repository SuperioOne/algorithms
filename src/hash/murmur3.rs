use self::hash_fn::{murmurhash3_128, murmurhash3_32};
use super::HashFunc;

pub mod hash_fn;

pub struct MurmurHash3_32 {
  seed: u32,
}

pub struct MurmurHash3_128 {
  seed: u64,
}

impl MurmurHash3_32 {
  pub fn new() -> Self {
    Self { seed: 0 }
  }
  pub fn new_with_seed(seed: u32) -> Self {
    Self { seed }
  }
}

impl MurmurHash3_128 {
  pub fn new() -> Self {
    Self { seed: 0 }
  }

  pub fn new_with_seed(seed: u64) -> Self {
    Self { seed }
  }
}

impl Default for MurmurHash3_32 {
  fn default() -> Self {
    Self::new()
  }
}

impl Default for MurmurHash3_128 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for MurmurHash3_32 {
  type Output = u32;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    murmurhash3_32(self.seed, bytes)
  }
}

impl HashFunc for MurmurHash3_128 {
  type Output = u128;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    murmurhash3_128(self.seed, bytes)
  }
}

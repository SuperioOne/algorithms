use self::hash_fn::crc32c_with_initial;
use crate::hash::HashFunc;

#[cfg(not(target_feature = "sse4.2"))]
mod crc32c_table;

pub mod hash_fn;

pub struct Crc32C {
  value: u32,
}

impl Crc32C {
  pub const fn new() -> Self {
    Self { value: 0 }
  }

  pub const fn new_with_initial(value: u32) -> Self {
    Self { value }
  }
}

impl Default for Crc32C {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for Crc32C {
  type Output = u32;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    crc32c_with_initial(bytes, self.value)
  }
}

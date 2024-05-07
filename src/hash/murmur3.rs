use self::hash_fn::{murmurhash3_128, murmurhash3_32};
use super::HashFunc;

pub mod hash_fn;

pub struct MurmurHash3_32;
pub struct MurmurHash3_128;

impl HashFunc for MurmurHash3_32 {
  type Seed = u32;
  type Output = u32;

  fn get_hash(seed: Self::Seed, bytes: &[u8]) -> Self::Output {
    murmurhash3_32(seed, bytes)
  }
}

impl HashFunc for MurmurHash3_128 {
  type Seed = u64;
  type Output = u128;

  fn get_hash(seed: Self::Seed, bytes: &[u8]) -> Self::Output {
    murmurhash3_128(seed, bytes)
  }
}

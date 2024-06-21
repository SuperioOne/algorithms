use self::hash_fn::{
  cityhash_128, cityhash_128_with_seed, cityhash_32, cityhash_64, cityhash_64_with_seed, K2,
};
use super::HashFunc;

pub mod hash_fn;

pub struct CityHash32 {
  seed: u32,
}

impl CityHash32 {
  pub fn new() -> Self {
    Self { seed: 0 }
  }

  pub fn new_with_seed(seed: u32) -> Self {
    Self { seed }
  }
}

impl Default for CityHash32 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for CityHash32 {
  type Output = u32;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    cityhash_32(bytes, self.seed)
  }
}

pub enum CityHashSeed {
  Default,
  CustomSeed64(u64),
  CustomSeed128(u64, u64),
}

impl Default for CityHashSeed {
  fn default() -> Self {
    Self::Default
  }
}

impl From<u64> for CityHashSeed {
  fn from(value: u64) -> Self {
    Self::CustomSeed64(value)
  }
}

impl From<u32> for CityHashSeed {
  fn from(value: u32) -> Self {
    Self::CustomSeed64(value as u64)
  }
}

impl From<u128> for CityHashSeed {
  fn from(value: u128) -> Self {
    Self::CustomSeed128(value as u64, (value >> 64) as u64)
  }
}

impl From<(u64, u64)> for CityHashSeed {
  fn from(value: (u64, u64)) -> Self {
    Self::CustomSeed128(value.0, value.1)
  }
}

impl From<usize> for CityHashSeed {
  fn from(value: usize) -> Self {
    Self::CustomSeed64(value as u64)
  }
}

pub struct CityHash64 {
  seed: CityHashSeed,
}

impl CityHash64 {
  pub fn new() -> Self {
    Self {
      seed: CityHashSeed::default(),
    }
  }

  pub fn new_with_seed(seed: CityHashSeed) -> Self {
    Self { seed }
  }
}

impl Default for CityHash64 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for CityHash64 {
  type Output = u64;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    match self.seed {
      CityHashSeed::Default => cityhash_64(bytes),
      CityHashSeed::CustomSeed128(seed0, seed1) => cityhash_64_with_seed(bytes, seed0, seed1),
      CityHashSeed::CustomSeed64(seed1) => cityhash_64_with_seed(bytes, K2, seed1),
    }
  }
}

pub struct CityHash128 {
  seed: CityHashSeed,
}

impl CityHash128 {
  pub fn new() -> Self {
    Self {
      seed: CityHashSeed::default(),
    }
  }

  pub fn new_with_seed(seed: CityHashSeed) -> Self {
    Self { seed }
  }
}

impl Default for CityHash128 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for CityHash128 {
  type Output = u128;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    match self.seed {
      CityHashSeed::Default => cityhash_128(bytes),
      CityHashSeed::CustomSeed64(seed0) => cityhash_128_with_seed(bytes, seed0, 0),
      CityHashSeed::CustomSeed128(seed0, seed1) => cityhash_128_with_seed(bytes, seed0, seed1),
    }
  }
}

pub struct CityHashCrc128 {
  seed: CityHashSeed,
}

impl CityHashCrc128 {
  pub fn new() -> Self {
    Self {
      seed: CityHashSeed::Default,
    }
  }

  pub fn new_with_seed(seed: CityHashSeed) -> Self {
    Self { seed }
  }
}

impl Default for CityHashCrc128 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for CityHashCrc128 {
  type Output = u128;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    match self.seed {
      CityHashSeed::Default => hash_fn::cityhash_crc128(bytes),
      CityHashSeed::CustomSeed64(seed0) => hash_fn::cityhash_crc128_with_seed(bytes, seed0, 0),
      CityHashSeed::CustomSeed128(seed0, seed1) => {
        hash_fn::cityhash_crc128_with_seed(bytes, seed0, seed1)
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct U256(pub u64, pub u64, pub u64, pub u64);

impl From<(u64, u64, u64, u64)> for U256 {
  fn from(value: (u64, u64, u64, u64)) -> Self {
    Self(value.0, value.1, value.2, value.3)
  }
}

impl From<U256> for (u64, u64, u64, u64) {
  fn from(value: U256) -> Self {
    (value.0, value.1, value.2, value.3)
  }
}

impl From<(u128, u128)> for U256 {
  fn from(value: (u128, u128)) -> Self {
    Self(
      value.0 as u64,
      (value.0 >> 64) as u64,
      value.1 as u64,
      (value.1 >> 64) as u64,
    )
  }
}

impl Default for U256 {
  fn default() -> Self {
    Self(0, 0, 0, 0)
  }
}

pub struct CityHashCrc256;

impl CityHashCrc256 {
  pub fn new() -> Self {
    Self
  }
}

impl Default for CityHashCrc256 {
  fn default() -> Self {
    Self::new()
  }
}

impl HashFunc for CityHashCrc256 {
  type Output = U256;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output {
    hash_fn::cityhash_crc256(bytes)
  }
}

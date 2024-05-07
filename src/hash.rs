pub mod murmur3;
pub mod pearson;

pub trait HashStreamFunc {
  type Output;

  fn update(&mut self, bytes: &[u8]);
  fn complete(&self) -> Self::Output;
  fn complete_and_reset(&mut self) -> Self::Output;
}

pub trait HashFunc {
  type Seed;
  type Output;

  fn get_hash(seed: Self::Seed, bytes: &[u8]) -> Self::Output;
}

// Re-export high level structs
pub use murmur3::{MurmurHash3_128, MurmurHash3_32};
pub use pearson::{
  PearsonHash_16, PearsonHash_32, PearsonHash_64, PearsonHash_8, PearsonHash_Usize,
};

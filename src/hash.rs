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

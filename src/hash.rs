pub mod cityhash;
pub mod murmur3;
pub mod pearson;

pub trait HashStreamFunc {
  type Output;

  fn update(&mut self, bytes: &[u8]);
  fn complete(&self) -> Self::Output;
}

pub trait HashFunc {
  type Output;

  fn get_hash(&self, bytes: &[u8]) -> Self::Output;
}

pub mod murmur3;
pub mod pearson;

pub trait HashFunc<TOut> {
  fn update(&mut self, bytes: &[u8]);
  fn complete(&self) -> TOut;
  fn complete_and_reset(&mut self) -> TOut;
}

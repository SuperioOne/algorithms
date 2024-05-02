mod hash_fn;

use self::hash_fn::{pearson_hash_u16, pearson_hash_u32, pearson_hash_u64, pearson_hash_u8};
use super::HashFunc;

pub struct PearsonHash<T> {
  state: T,
}

impl<T> PearsonHash<T>
where
  T: Default,
{
  pub fn new() -> Self {
    Self {
      state: T::default(),
    }
  }
}

impl HashFunc<u8> for PearsonHash<u8> {
  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u8(self.state, input);
  }

  fn complete(&self) -> u8 {
    self.state
  }

  fn complete_and_reset(&mut self) -> u8 {
    let result = self.state;
    self.state = u8::default();

    result
  }
}

impl HashFunc<u16> for PearsonHash<u16> {
  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u16(self.state, input);
  }

  fn complete(&self) -> u16 {
    self.state
  }

  fn complete_and_reset(&mut self) -> u16 {
    let result = self.state;
    self.state = u16::default();

    result
  }
}

impl HashFunc<u32> for PearsonHash<u32> {
  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u32(self.state, input);
  }

  fn complete(&self) -> u32 {
    self.state
  }

  fn complete_and_reset(&mut self) -> u32 {
    let result = self.state;
    self.state = u32::default();

    result
  }
}

impl HashFunc<u64> for PearsonHash<u64> {
  fn update(&mut self, input: &[u8]) {
    self.state = pearson_hash_u64(self.state, input);
  }

  fn complete(&self) -> u64 {
    self.state
  }

  fn complete_and_reset(&mut self) -> u64 {
    let result = self.state;
    self.state = u64::default();

    result
  }
}

impl HashFunc<usize> for PearsonHash<usize> {
  fn update(&mut self, input: &[u8]) {
    #[cfg(target_pointer_width = "64")]
    {
      self.state = pearson_hash_u64(self.state as u64, input) as usize;
    };

    #[cfg(target_pointer_width = "32")]
    {
      self.state = pearson_hash_u32(self.state as u32, input) as usize;
    };
  }

  fn complete(&self) -> usize {
    self.state
  }

  fn complete_and_reset(&mut self) -> usize {
    let result = self.state;
    self.state = usize::default();

    result
  }
}

impl std::hash::Hasher for PearsonHash<u64> {
  fn finish(&self) -> u64 {
    self.complete()
  }

  fn write(&mut self, bytes: &[u8]) {
    self.update(bytes);
  }
}

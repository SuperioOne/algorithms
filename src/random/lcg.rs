use super::NumberGenerator;

const A_8: i8 = 55;
const C_8: i8 = 13;
const A_U8: u8 = A_8 as u8;
const C_U8: u8 = C_8 as u8;
const A_16: i16 = 219;
const C_16: i16 = 37;
const A_U16: u16 = A_16 as u16;
const C_U16: u16 = C_16 as u16;
const A_32: i32 = 3467255;
const C_32: i32 = 69;
const A_U32: u32 = A_32 as u32;
const C_U32: u32 = C_32 as u32;
const A_64: i64 = 2806196910506780709;
const C_64: i64 = 797;
const A_U64: u64 = A_64 as u64;
const C_U64: u64 = C_64 as u64;
#[cfg(target_pointer_width = "64")]
const A_USIZE: usize = 2806196910506780709;
#[cfg(target_pointer_width = "64")]
const C_USIZE: usize = 797;
#[cfg(target_pointer_width = "32")]
const A_USIZE: usize = 3467255;
#[cfg(target_pointer_width = "32")]
const C_USIZE: usize = 69;

pub struct Lcg<T> {
  seed: T,
}

impl<T> Lcg<T> {
  pub fn new(seed: T) -> Self {
    Self { seed }
  }

  pub fn set_seed(&mut self, new_seed: T) -> &mut Self {
    self.seed = new_seed;
    self
  }

  pub fn get_seed(&self) -> &T {
    &self.seed
  }
}

impl<T> Default for Lcg<T>
where
  T: Default,
{
  fn default() -> Self {
    Self::new(T::default())
  }
}

impl NumberGenerator<i16> for Lcg<i16> {
  fn get_next(&mut self) -> i16 {
    let next: i16 = self.seed.wrapping_mul(A_16).wrapping_add(C_16);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<i16> {
  type Item = i16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u16> for Lcg<u16> {
  fn get_next(&mut self) -> u16 {
    let next: u16 = self.seed.wrapping_mul(A_U16).wrapping_add(C_U16);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<u16> {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<i8> for Lcg<i8> {
  fn get_next(&mut self) -> i8 {
    let next: i8 = self.seed.wrapping_mul(A_8).wrapping_add(C_8);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<i8> {
  type Item = i8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u8> for Lcg<u8> {
  fn get_next(&mut self) -> u8 {
    let next: u8 = self.seed.wrapping_mul(A_U8).wrapping_add(C_U8);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<u8> {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<i32> for Lcg<i32> {
  fn get_next(&mut self) -> i32 {
    let next: i32 = self.seed.wrapping_mul(A_32).wrapping_add(C_32);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<i32> {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for Lcg<u32> {
  fn get_next(&mut self) -> u32 {
    let next: u32 = self.seed.wrapping_mul(A_U32).wrapping_add(C_U32);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<u32> {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<i64> for Lcg<i64> {
  fn get_next(&mut self) -> i64 {
    let next: i64 = self.seed.wrapping_mul(A_64).wrapping_add(C_64);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<i64> {
  type Item = i64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for Lcg<u64> {
  fn get_next(&mut self) -> u64 {
    let next: u64 = self.seed.wrapping_mul(A_U64).wrapping_add(C_U64);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<u64> {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<usize> for Lcg<usize> {
  fn get_next(&mut self) -> usize {
    let next: usize = self.seed.wrapping_mul(A_USIZE).wrapping_add(C_USIZE);
    self.seed = next;
    next
  }
}

impl Iterator for Lcg<usize> {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

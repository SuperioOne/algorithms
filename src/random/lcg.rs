#![allow(non_camel_case_types)]

use super::NumberGenerator;

const A_U8: u8 = 55;
const C_U8: u8 = 13;
const A_U16: u16 = 219;
const C_U16: u16 = 37;
const A_U32: u32 = 3467255;
const C_U32: u32 = 69;
const A_U64: u64 = 2806196910506780709;
const C_U64: u64 = 797;

#[cfg(target_pointer_width = "64")]
const A_USIZE: usize = A_U64 as usize;
#[cfg(target_pointer_width = "64")]
const C_USIZE: usize = C_U64 as usize;
#[cfg(target_pointer_width = "32")]
const A_USIZE: usize = A_U32 as usize;
#[cfg(target_pointer_width = "32")]
const C_USIZE: usize = 69;

pub struct Lcg_8 {
  state: u8,
}

pub struct Lcg_16 {
  state: u16,
}

pub struct Lcg_32 {
  state: u32,
}

pub struct Lcg_64 {
  state: u64,
}

pub struct Lcg_Usize {
  state: usize,
}

impl Lcg_8 {
  pub fn new(seed: u8) -> Self {
    Self { state: seed }
  }

  pub fn set_seed(&mut self, new_seed: u8) -> &mut Self {
    self.state = new_seed;
    self
  }

  pub fn get_seed(&self) -> u8 {
    self.state
  }
}

impl Default for Lcg_8 {
  fn default() -> Self {
    Self::new(u8::default())
  }
}

impl Iterator for Lcg_8 {
  type Item = u8;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u8> for Lcg_8 {
  fn get_next(&mut self) -> u8 {
    let next: u8 = self.state.wrapping_mul(A_U8).wrapping_add(C_U8);
    self.state = next;
    next
  }
}

impl Lcg_16 {
  pub fn new(seed: u16) -> Self {
    Self { state: seed }
  }

  pub fn set_seed(&mut self, new_seed: u16) -> &mut Self {
    self.state = new_seed;
    self
  }

  pub fn get_seed(&self) -> u16 {
    self.state
  }
}

impl Default for Lcg_16 {
  fn default() -> Self {
    Self::new(u16::default())
  }
}

impl Iterator for Lcg_16 {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u16> for Lcg_16 {
  fn get_next(&mut self) -> u16 {
    let next: u16 = self.state.wrapping_mul(A_U16).wrapping_add(C_U16);
    self.state = next;
    next
  }
}

impl Lcg_32 {
  pub fn new(seed: u32) -> Self {
    Self { state: seed }
  }

  pub fn set_seed(&mut self, new_seed: u32) -> &mut Self {
    self.state = new_seed;
    self
  }

  pub fn get_seed(&self) -> u32 {
    self.state
  }
}

impl Default for Lcg_32 {
  fn default() -> Self {
    Self::new(u32::default())
  }
}

impl Iterator for Lcg_32 {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for Lcg_32 {
  fn get_next(&mut self) -> u32 {
    let next: u32 = self.state.wrapping_mul(A_U32).wrapping_add(C_U32);
    self.state = next;
    next
  }
}

impl Lcg_64 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }

  pub fn set_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }

  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Lcg_64 {
  fn default() -> Self {
    Self::new(u64::default())
  }
}

impl Iterator for Lcg_64 {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for Lcg_64 {
  fn get_next(&mut self) -> u64 {
    let next: u64 = self.state.wrapping_mul(A_U64).wrapping_add(C_U64);
    self.state = next;
    next
  }
}

impl Lcg_Usize {
  pub fn new(seed: usize) -> Self {
    Self { state: seed }
  }

  pub fn set_seed(&mut self, new_seed: usize) -> &mut Self {
    self.state = new_seed;
    self
  }

  pub fn get_seed(&self) -> usize {
    self.state
  }
}

impl Default for Lcg_Usize {
  fn default() -> Self {
    Self::new(usize::default())
  }
}

impl Iterator for Lcg_Usize {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<usize> for Lcg_Usize {
  fn get_next(&mut self) -> usize {
    let next: usize = self.state.wrapping_mul(A_USIZE).wrapping_add(C_USIZE);
    self.state = next;
    next
  }
}

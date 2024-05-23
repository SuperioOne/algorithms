use super::NumberGenerator;
use std::arch::x86_64::{_rdrand16_step, _rdrand32_step, _rdrand64_step};

macro_rules! panic_hw_fail {
  () => {
    panic!("Unable to get random number from hardware (CPU).");
  };
}

const MAX_HW_RETRY: u8 = 10;

pub struct HwRand16;
pub struct HwRand32;
pub struct HwRand64;

#[cfg(target_pointer_width = "64")]
pub struct HwRandUsize;

impl NumberGenerator<u16> for HwRand16 {
  fn get_next(&mut self) -> u16 {
    let mut value: u16 = u16::default();

    for _ in 0..MAX_HW_RETRY {
      match unsafe { _rdrand16_step(&mut value) } {
        1 => {
          return value;
        }
        _ => {
          continue;
        }
      }
    }

    panic_hw_fail!();
  }
}

impl Iterator for HwRand16 {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u32> for HwRand32 {
  fn get_next(&mut self) -> u32 {
    let mut value: u32 = u32::default();

    for _ in 0..MAX_HW_RETRY {
      match unsafe { _rdrand32_step(&mut value) } {
        1 => {
          return value;
        }
        _ => {
          continue;
        }
      }
    }

    panic_hw_fail!();
  }
}

impl Iterator for HwRand32 {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl NumberGenerator<u64> for HwRand64 {
  fn get_next(&mut self) -> u64 {
    let mut value: u64 = u64::default();

    for _ in 0..MAX_HW_RETRY {
      match unsafe { _rdrand64_step(&mut value) } {
        1 => {
          return value;
        }
        _ => {
          continue;
        }
      }
    }

    panic_hw_fail!();
  }
}

impl Iterator for HwRand64 {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

#[cfg(target_pointer_width = "64")]
impl NumberGenerator<usize> for HwRandUsize {
  fn get_next(&mut self) -> usize {
    let mut value: u64 = u64::default();

    for _ in 0..MAX_HW_RETRY {
      match unsafe { _rdrand64_step(&mut value) } {
        1 => {
          return value as usize;
        }
        _ => {
          continue;
        }
      }
    }

    panic_hw_fail!();
  }
}

#[cfg(target_pointer_width = "64")]
impl Iterator for HwRandUsize {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

const M_32: i32 = i32::wrapping_pow(2, 31).wrapping_sub(1);
const M_64: i64 = i64::wrapping_pow(2, 63);
const A_32: i32 = 3467255;
const A_64: i64 = 2806196910506780709;
const C_32: i32 = 69; // Nice;
const C_64: i64 = 797;

pub struct Lcg32 {
  seed: i32,
}

pub struct Lcg64 {
  seed: i64,
}

impl Lcg64 {
  pub fn reset_seed(&mut self, value: i64) {
    self.seed = value;
  }

  pub fn new(seed: i64) -> Self {
    Self { seed }
  }

  pub fn get_next(&mut self) -> i64 {
    let next: i64 = (self.seed.wrapping_mul(A_64).wrapping_add(C_64)) % M_64;
    self.seed = next;

    next
  }
}

impl Default for Lcg64 {
  fn default() -> Self {
    Self::new(0)
  }
}

impl Iterator for Lcg64 {
  type Item = i64;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

impl Lcg32 {
  pub fn reset_seed(&mut self, value: i32) {
    self.seed = value;
  }

  pub fn new(seed: i32) -> Self {
    Self { seed }
  }

  pub fn get_next(&mut self) -> i32 {
    let next: i32 = (self.seed.wrapping_mul(A_32).wrapping_add(C_32)) % M_32;
    self.seed = next;

    next
  }
}

impl Default for Lcg32 {
  fn default() -> Self {
    Self::new(0)
  }
}

impl Iterator for Lcg32 {
  type Item = i32;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

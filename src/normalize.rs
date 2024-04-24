const L8: f32 = i8::MAX as f32 - i8::MIN as f32;
const L16: f32 = i16::MAX as f32 - i16::MIN as f32;
const L32: f32 = i32::MAX as f32 - i32::MIN as f32;
const L64: f64 = i64::MAX as f64 - i64::MIN as f64;

pub trait Normalize: PartialOrd {
  fn min_max_normalize(self, min: Self, max: Self) -> Self;
}

impl Normalize for i8 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f32;
    let f_min = min as f32;
    let f_max = max as f32;

    (f32::round((((f_max - f_min) / L8) * (f_val - i8::MIN as f32)) + f_min)) as i8
  }
}

impl Normalize for u8 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f32;
    let f_min = min as f32;
    let f_max = max as f32;

    (f32::round((((f_max - f_min) / u8::MAX as f32) * f_val) + f_min)) as u8
  }
}

impl Normalize for i16 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f32;
    let f_min = min as f32;
    let f_max = max as f32;

    (f32::round((((f_max - f_min) / L16) * (f_val - i16::MIN as f32)) + f_min)) as i16
  }
}

impl Normalize for u16 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f32;
    let f_min = min as f32;
    let f_max = max as f32;

    (f32::round((((f_max - f_min) / u16::MAX as f32) * f_val) + f_min)) as u16
  }
}

impl Normalize for i32 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f32;
    let f_min = min as f32;
    let f_max = max as f32;

    (f32::round((((f_max - f_min) / L32) * (f_val - i32::MIN as f32)) + f_min)) as i32
  }
}

impl Normalize for u32 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f32;
    let f_min = min as f32;
    let f_max = max as f32;

    (f32::round(((f_max - f_min) / u32::MAX as f32) * f_val + f_min)) as u32
  }
}

impl Normalize for i64 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f64;
    let f_min = min as f64;
    let f_max = max as f64;

    (f64::round((((f_max - f_min) / L64) * (f_val - i64::MIN as f64)) + f_min)) as i64
  }
}

impl Normalize for u64 {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f64;
    let f_min = min as f64;
    let f_max = max as f64;

    (f64::round(((f_max - f_min) / u64::MAX as f64) * f_val + f_min)) as u64
  }
}

#[cfg(target_pointer_width = "64")]
impl Normalize for usize {
  #[inline]
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f64;
    let f_min = min as f64;
    let f_max = max as f64;

    (f64::round(((f_max - f_min) / usize::MAX as f64) * f_val + f_min)) as usize
  }
}

#[cfg(target_pointer_width = "32")]
impl Normalize for usize {
  #[inline]
  fn normalized_clamp(self, min: Self, max: Self) -> Self {
    assert!(min < max);

    let f_val = self as f32;
    let f_min = min as f32;
    let f_max = max as f32;

    (f32::round(((f_max - f_min) / usize::MAX as f32) * f_val + f_min)) as usize
  }
}

impl Normalize for f32 {
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    ((max - min) / (f32::MAX - f32::MIN) * self) + min
  }
}

impl Normalize for f64 {
  fn min_max_normalize(self, min: Self, max: Self) -> Self {
    ((max - min) / (f64::MAX - f64::MIN) * self) + min
  }
}

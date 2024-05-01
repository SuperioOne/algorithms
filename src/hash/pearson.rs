pub struct Pearson<T> {
  state: T,
}

// https://www.rfc-editor.org/rfc/rfc3074
// const MIXING_TABLE: &[u8] = &[
//   51, 175, 119, 215, 81, 14, 79, 191, 103, 49, 181, 143, 186, 157, 0, 232, 31, 32, 55, 60, 152, 58,
//   17, 237, 174, 70, 160, 144, 220, 90, 57, 223, 59, 3, 18, 140, 111, 166, 203, 196, 134, 243, 124,
//   95, 222, 179, 197, 65, 180, 48, 36, 15, 107, 46, 233, 130, 165, 30, 123, 161, 209, 23, 97, 16,
//   40, 91, 219, 61, 100, 10, 210, 109, 250, 127, 22, 138, 29, 108, 244, 67, 207, 9, 178, 204, 74,
//   98, 126, 249, 167, 116, 34, 77, 193, 200, 121, 5, 20, 113, 71, 35, 128, 13, 182, 94, 25, 226,
//   227, 199, 75, 27, 41, 245, 230, 224, 43, 225, 177, 26, 155, 150, 212, 142, 218, 115, 241, 73, 88,
//   105, 39, 114, 62, 255, 192, 201, 145, 214, 168, 158, 221, 148, 154, 122, 12, 84, 82, 163, 44,
//   139, 228, 236, 205, 242, 217, 11, 187, 146, 159, 64, 86, 239, 195, 42, 106, 198, 118, 112, 184,
//   172, 87, 2, 173, 117, 176, 229, 247, 253, 137, 185, 99, 164, 102, 147, 45, 66, 231, 52, 141, 211,
//   194, 206, 246, 238, 56, 110, 78, 248, 63, 240, 189, 93, 92, 51, 53, 183, 19, 171, 72, 50, 33,
//   104, 101, 69, 8, 252, 83, 120, 76, 135, 85, 54, 202, 125, 188, 213, 96, 235, 136, 208, 162, 129,
//   190, 132, 156, 38, 47, 1, 7, 254, 24, 4, 216, 131, 89, 21, 28, 133, 37, 153, 149, 80, 170, 68, 6,
//   169, 234, 151,
// ];

// Table from the "Fast Hashing of Variable-Length Text Strings" Paper.
const MIXING_TABLE: &[u8] = &[
  1, 87, 49, 12, 176, 178, 102, 166, 121, 193, 6, 84, 249, 230, 44, 163, 14, 197, 213, 181, 161,
  85, 218, 80, 64, 239, 24, 226, 236, 142, 38, 200, 110, 177, 104, 103, 141, 253, 255, 50, 77, 101,
  81, 18, 45, 96, 31, 222, 25, 107, 190, 70, 86, 237, 240, 34, 72, 242, 20, 214, 244, 227, 149,
  235, 97, 234, 57, 22, 60, 250, 82, 175, 208, 5, 127, 199, 111, 62, 135, 248, 174, 169, 211, 58,
  66, 154, 106, 195, 245, 171, 17, 187, 182, 179, 0, 243, 132, 56, 148, 75, 128, 133, 158, 100,
  130, 126, 91, 13, 153, 246, 216, 219, 119, 68, 223, 78, 83, 88, 201, 99, 122, 11, 92, 32, 136,
  114, 52, 10, 138, 30, 48, 183, 156, 35, 61, 26, 143, 74, 251, 94, 129, 162, 63, 152, 170, 7, 115,
  167, 241, 206, 3, 150, 55, 59, 151, 220, 90, 53, 23, 131, 125, 173, 15, 238, 79, 95, 89, 16, 105,
  137, 225, 224, 217, 160, 37, 123, 118, 73, 2, 157, 46, 116, 9, 145, 134, 228, 207, 212, 202, 215,
  69, 229, 27, 188, 67, 124, 168, 252, 42, 4, 29, 108, 21, 247, 19, 205, 39, 203, 233, 40, 186,
  147, 198, 192, 155, 33, 164, 191, 98, 204, 165, 180, 117, 76, 140, 36, 210, 172, 41, 54, 159, 8,
  185, 232, 113, 196, 231, 47, 146, 120, 51, 65, 28, 144, 254, 221, 93, 189, 194, 139, 112, 43, 71,
  109, 184, 209,
];

const U16_MASK: u16 = 1;
const U32_MASK: u32 = (1 << 24) + (87 << 16) + 49;
const U64_MASK: u64 =
  (1 << 48) + (87 << 40) + (49 << 32) + (12 << 24) + (176 << 16) + (178 << 8) + 102;

// Primitive versions of std::mem::transmute().
macro_rules! u16_from {
  ($high:expr, $low:expr) => {{
    let high: u16 = ($high as u16).wrapping_shl(8);
    let low: u16 = $low as u16;

    high + low
  }};
}

macro_rules! u32_from {
  ($byte_0:expr, $byte_1:expr, $byte_2:expr, $byte_3:expr) => {{
    let hh: u32 = ($byte_0 as u32).wrapping_shl(32);
    let hl: u32 = ($byte_1 as u32).wrapping_shl(16);
    let lh: u32 = ($byte_2 as u32).wrapping_shl(8);
    let ll: u32 = $byte_3 as u32;

    hh + hl + lh + ll
  }};
}

macro_rules! u64_from {
  ($byte_0:expr, $byte_1:expr, $byte_2:expr, $byte_3:expr, $byte_4:expr, $byte_5:expr, $byte_6:expr, $byte_7:expr) => {{
    let h0: u64 = ($byte_0 as u64).wrapping_shl(56);
    let h1: u64 = ($byte_1 as u64).wrapping_shl(48);
    let h2: u64 = ($byte_2 as u64).wrapping_shl(40);
    let h3: u64 = ($byte_3 as u64).wrapping_shl(32);
    let l0: u64 = ($byte_4 as u64).wrapping_shl(24);
    let l1: u64 = ($byte_5 as u64).wrapping_shl(16);
    let l2: u64 = ($byte_6 as u64).wrapping_shl(8);
    let l3: u64 = $byte_7 as u64;

    h0 + h1 + h2 + h3 + l0 + l1 + l2 + l3
  }};
}

macro_rules! bytes_from_u16 {
  ($val:expr) => {{
    let input: u16 = $val;

    ((input >> 8) as u8, (input & 0xFF) as u8)
  }};
}

macro_rules! bytes_from_u32 {
  ($val:expr) => {{
    let input: u32 = $val;

    (
      (input >> 24) as u8,
      ((input >> 16) & 0xFF) as u8,
      ((input >> 8) & 0xFF) as u8,
      (input & 0xFF) as u8,
    )
  }};
}

macro_rules! bytes_from_u64 {
  ($val:expr) => {{
    let input: u64 = $val;

    (
      (input >> 56) as u8,
      ((input >> 48) & 0xFF) as u8,
      ((input >> 40) & 0xFF) as u8,
      ((input >> 32) & 0xFF) as u8,
      ((input >> 24) & 0xFF) as u8,
      ((input >> 16) & 0xFF) as u8,
      ((input >> 8) & 0xFF) as u8,
      (input & 0xFF) as u8,
    )
  }};
}

impl<T> Pearson<T>
where
  T: Default,
{
  pub fn new() -> Self {
    Self {
      state: T::default(),
    }
  }

  pub fn complete(self) -> T {
    self.state
  }
}

impl Pearson<u8> {
  pub fn update(mut self, input: &[u8]) -> Self {
    for byte in input {
      let idx = (self.state ^ byte) as usize;
      self.state = *MIXING_TABLE.get(idx).unwrap();
    }

    self
  }
}

impl Pearson<u16> {
  pub fn update(mut self, input: &[u8]) -> Self {
    if input.is_empty() {
      return self;
    }

    for byte in input {
      let input = u16_from!(*byte, *byte) ^ U16_MASK;
      let xored = input ^ self.state;
      let (idx_high, idx_low) = bytes_from_u16!(xored);
      self.state = u16_from!(
        *MIXING_TABLE.get(idx_high as usize).unwrap(),
        *MIXING_TABLE.get(idx_low as usize).unwrap()
      );
    }

    self
  }
}

impl Pearson<u32> {
  pub fn update(mut self, input: &[u8]) -> Self {
    if input.is_empty() {
      return self;
    }

    for byte in input {
      let input = u32_from!(*byte, *byte, *byte, *byte) ^ U32_MASK;
      let xored = input ^ self.state;
      let (idx_0, idx_1, idx_2, idx_3) = bytes_from_u32!(xored);

      self.state = u32_from!(
        *MIXING_TABLE.get(idx_0 as usize).unwrap(),
        *MIXING_TABLE.get(idx_1 as usize).unwrap(),
        *MIXING_TABLE.get(idx_2 as usize).unwrap(),
        *MIXING_TABLE.get(idx_3 as usize).unwrap()
      );
    }

    self
  }
}

impl Pearson<u64> {
  pub fn update(mut self, input: &[u8]) -> Self {
    if input.is_empty() {
      return self;
    }

    for byte in input {
      let input = u64_from!(*byte, *byte, *byte, *byte, *byte, *byte, *byte, *byte) ^ U64_MASK;
      let xored = input ^ self.state;
      let (idx_0, idx_1, idx_2, idx_3, idx_4, idx_5, idx_6, idx_7) = bytes_from_u64!(xored);
      self.state = u64_from!(
        *MIXING_TABLE.get(idx_0 as usize).unwrap(),
        *MIXING_TABLE.get(idx_1 as usize).unwrap(),
        *MIXING_TABLE.get(idx_2 as usize).unwrap(),
        *MIXING_TABLE.get(idx_3 as usize).unwrap(),
        *MIXING_TABLE.get(idx_4 as usize).unwrap(),
        *MIXING_TABLE.get(idx_5 as usize).unwrap(),
        *MIXING_TABLE.get(idx_6 as usize).unwrap(),
        *MIXING_TABLE.get(idx_7 as usize).unwrap()
      );
    }

    self
  }
}

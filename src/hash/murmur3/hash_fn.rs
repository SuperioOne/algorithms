// NOTE: Not planning to implement 128_x86 version.

const C1_32: u32 = 0xCC9E_2D51;
const C2_32: u32 = 0x1B87_3593;
const C1_64: u64 = 0x87C3_7B91_1142_53D5;
const C2_64: u64 = 0x4CF5_AD43_2745_937F;

macro_rules! fmix32 {
  ($input:expr) => {{
    let mut i: u32 = $input;
    i ^= i.wrapping_shr(16);
    i = i.wrapping_mul(0x85EB_CA6B);
    i ^= i.wrapping_shr(13);
    i = i.wrapping_mul(0xC2B2_AE35);
    i ^= i.wrapping_shr(16);

    i
  }};
}

macro_rules! fmix64 {
  ($input:expr) => {{
    let mut i: u64 = $input;
    i ^= i.wrapping_shr(33);
    i = i.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    i ^= i.wrapping_shr(33);
    i = i.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
    i ^= i.wrapping_shr(33);

    i
  }};
}

macro_rules! u128_from {
  ($low:expr, $high:expr) => {{
    let h: u64 = $high;
    let l: u64 = $low;

    ((h as u128) << 64) + (l as u128)
  }};
}

pub fn murmurhash3_32(seed: u32, input: &[u8]) -> u32 {
  let mut h1 = seed;
  let blocks_len = input.len() / 4;
  let blocks_ptr: *const u32 = input.as_ptr().cast();

  for idx in 0..blocks_len {
    let mut k1 = unsafe { blocks_ptr.add(idx).read_unaligned() };

    k1 = k1.wrapping_mul(C1_32).rotate_left(15).wrapping_mul(C2_32);
    h1 ^= k1;
    h1 = h1.rotate_left(13).wrapping_mul(5).wrapping_add(0xE654_6B64);
  }

  let tail_len = input.len() & 3;

  if tail_len > 0 {
    let tail: *const u8 = unsafe { input.as_ptr().byte_add(input.len() - tail_len) };
    let mut k1: u32 = 0;
    let mut l: u32 = 0;

    for i in 0..tail_len {
      k1 ^= (unsafe { tail.add(i).read() } as u32) << l;
      l += 8;
    }

    h1 ^= k1.wrapping_mul(C1_32).rotate_left(15).wrapping_mul(C2_32);
  }

  h1 ^= input.len() as u32;
  fmix32!(h1)
}

#[cfg(target_pointer_width = "64")]
pub fn murmurhash3_128(seed: u64, input: &[u8]) -> u128 {
  let mut h1: u64 = seed;
  let mut h2: u64 = seed;
  let blocks_len = input.len() / 16;
  let blocks_ptr: *const u64 = input.as_ptr().cast();

  for idx in 0..blocks_len {
    let mut k1 = unsafe { blocks_ptr.add(idx * 2).read_unaligned() };
    let mut k2 = unsafe { blocks_ptr.add(idx * 2 + 1).read_unaligned() };

    k1 = k1.wrapping_mul(C1_64).rotate_left(31).wrapping_mul(C2_64);
    h1 ^= k1;
    h1 = h1
      .rotate_left(27)
      .wrapping_add(h2)
      .wrapping_mul(5)
      .wrapping_add(0x52DC_E729);

    k2 = k2.wrapping_mul(C2_64).rotate_left(33).wrapping_mul(C1_64);
    h2 ^= k2;
    h2 = h2
      .rotate_left(31)
      .wrapping_add(h1)
      .wrapping_mul(5)
      .wrapping_add(0x3849_5AB5);
  }

  let tail_len: usize = input.len() & 15;

  if tail_len > 0 {
    let tail: *const u8 = unsafe { input.as_ptr().byte_add(input.len() - tail_len) };
    let mut k1: u64 = 0;
    let mut k2: u64 = 0;

    match tail_len {
      1..=7 => {
        let mut l: u32 = 0;

        for i in 0..tail_len {
          k1 ^= (unsafe { tail.add(i).read() } as u64) << l;
          l += 8;
        }
      }
      8 => k1 ^= unsafe { (tail as *const u64).read_unaligned() },
      9..=15 => {
        k1 ^= unsafe { (tail as *const u64).read_unaligned() };
        k2 ^= {
          let t: u64 = unsafe { (tail as *const u64).byte_add(tail_len - 8).read_unaligned() };
          let r: u32 = 64 - ((tail_len - 8) as u32 * 8);

          t >> r
        };

        k2 = k2.wrapping_mul(C2_64).rotate_left(33).wrapping_mul(C1_64);
        h2 ^= k2;
      }
      _ => unreachable!(),
    }

    k1 = k1.wrapping_mul(C1_64).rotate_left(31).wrapping_mul(C2_64);
    h1 ^= k1;
  }

  h1 ^= input.len() as u64;
  h2 ^= input.len() as u64;

  h1 = h1.wrapping_add(h2);
  h2 = h2.wrapping_add(h1);

  h1 = fmix64!(h1);
  h2 = fmix64!(h2);

  h1 = h1.wrapping_add(h2);
  h2 = h2.wrapping_add(h1);

  u128_from!(h2, h1)
}

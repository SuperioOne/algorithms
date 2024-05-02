const C1: u32 = 0xCC9E2D51;
const C2: u32 = 0x1B873593;

macro_rules! fmix32 {
  ($input:expr) => {{
    let mut i: u32 = $input;
    i ^= i.wrapping_shr(16);
    i = i.wrapping_mul(0x85EBCA6B);
    i ^= i.wrapping_shr(13);
    i = i.wrapping_mul(0xC2B2AE35);
    i ^= i.wrapping_shr(16);

    i
  }};
}

#[inline]
pub fn murmur3_32(seed: u32, input: &[u8]) -> u32 {
  if input.is_empty() {
    return fmix32!(seed);
  }

  let mut h1 = seed;
  let blocks_len = input.len() / 4;
  let blocks_ptr: *const u32 = (&input[..(blocks_len * 4)]).as_ptr().cast();

  for idx in 0..blocks_len {
    let mut k1 = unsafe { blocks_ptr.add(idx).read_unaligned() };

    k1 = k1.wrapping_mul(C1);
    k1 = k1.rotate_left(15);
    k1 = k1.wrapping_mul(C2);

    h1 ^= k1;
    h1 = h1.rotate_left(13);
    h1 = h1.wrapping_mul(5).wrapping_add(0xE6546B64);
  }

  let tail = &input[(blocks_len * 4)..];

  if !tail.is_empty() {
    let mut k1 = 0_u32;

    match tail.len() & 3 {
      1 => {
        k1 ^= *tail.get(0).unwrap() as u32;
      }
      2 => {
        k1 ^= *tail.get(0).unwrap() as u32;
        k1 ^= (*tail.get(1).unwrap() as u32) << 8;
      }
      3 => {
        k1 ^= *tail.get(0).unwrap() as u32;
        k1 ^= (*tail.get(1).unwrap() as u32) << 8;
        k1 ^= (*tail.get(2).unwrap() as u32) << 16;
      }
      _ => unreachable!(),
    }

    h1 ^= k1.wrapping_mul(C1).rotate_left(15).wrapping_mul(C2);
  }

  h1 ^= input.len() as u32;
  fmix32!(h1)
}

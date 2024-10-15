macro_rules! u128_from {
  ($low:expr, $high:expr) => {
    (($high as u128) << 64) + ($low as u128)
  };
}

pub(super) const K0: u64 = 0xC3A5_C85C_97CB_3127;
pub(super) const K1: u64 = 0xB492_B66F_BE98_F273;
pub(super) const K2: u64 = 0x9AE1_6A3B_2F90_404F;
pub(super) const K_MUL: u64 = 0x9DDF_EA08_EB38_2D69;

const C1: u32 = 0xCC9E_2D51;
const C2: u32 = 0x1B87_3593;

/**
* When to use .wrapping_ops() vs operator.
*
* Can operator chained?
*  - If yes, use function.
*  - If no;
*    - Is there any possibility for overflow and it's expected behavior?
*      - If yes, use wrapping_ops() function.
*      - If no, use operator.
*/

macro_rules! read {
  ($ptr:expr, $offset:expr) => {
    unsafe { $ptr.byte_add($offset).read_unaligned() }
  };

  ($ptr:expr) => {
    unsafe { $ptr.read_unaligned() }
  };
}

// 32-bit

macro_rules! fmix32 {
  ($input:expr) => {{
    let mut i: u32 = $input;
    i ^= i >> 16;
    i = i.wrapping_mul(0x85EB_CA6B);
    i ^= i >> 13;
    i = i.wrapping_mul(0xC2B2_AE35);
    i ^= i >> 16;

    i
  }};
}

macro_rules! permute3 {
  ($a:expr, $b:expr, $c:expr) => {
    core::mem::swap($a, $b);
    core::mem::swap($a, $c);
  };
}

fn mur3(mut a: u32, mut h: u32) -> u32 {
  a = a.wrapping_mul(C1).rotate_right(17).wrapping_mul(C2);
  h ^= a;
  h = h.rotate_right(19).wrapping_mul(5).wrapping_add(0xE654_6B64);
  h
}

#[inline]
fn hash_32_len_0_to_4(input: &[u8], seed: u32) -> u32 {
  let mut b: u32 = seed;
  let mut c: u32 = 9;

  for byte in input {
    b = b.wrapping_mul(C1).wrapping_add(*byte as u32);
    c ^= b;
  }

  fmix32!(mur3(b, mur3(input.len() as u32, c)))
}

#[inline]
fn hash_32_len_5_to_12(input: &[u8], seed: u32) -> u32 {
  let mut a: u32 = (input.len() as u32).wrapping_add(seed);
  let mut b: u32 = a * 5;
  let mut c: u32 = 9;
  let d = b;
  let input_ptr: *const u32 = input.as_ptr().cast();

  a = a.wrapping_add(read!(input_ptr));
  b = b.wrapping_add(read!(input_ptr, input.len() - 4));
  c = c.wrapping_add(read!(input_ptr, (input.len() >> 1) & 4));

  fmix32!(mur3(c, mur3(b, mur3(a, d))))
}

#[inline]
fn hash_32_len_13_to_24(input: &[u8], seed: u32) -> u32 {
  let input_ptr: *const u32 = input.as_ptr().cast();

  let a: u32 = read!(input_ptr, (input.len() >> 1) - 4);
  let b: u32 = read!(input_ptr, 4);
  let c: u32 = read!(input_ptr, input.len() - 8);
  let d: u32 = read!(input_ptr, input.len() >> 1);
  let e: u32 = read!(input_ptr);
  let f: u32 = read!(input_ptr, input.len() - 4);
  let h: u32 = (input.len() as u32).wrapping_add(seed);

  fmix32!(mur3(f, mur3(e, mur3(d, mur3(c, mur3(b, mur3(a, h)))))))
}

pub fn cityhash_32(input: &[u8], seed: u32) -> u32 {
  match input.len() {
    0..=4 => hash_32_len_0_to_4(input, seed),
    5..=12 => hash_32_len_5_to_12(input, seed),
    13..=24 => hash_32_len_13_to_24(input, seed),
    _ => {
      let len = input.len();
      let mut h: u32 = (len as u32).wrapping_add(seed);
      let mut g: u32 = C1.wrapping_mul(h);
      let mut f: u32 = g;
      let input_ptr: *const u32 = input.as_ptr().cast();

      let a0 = read!(input_ptr, len - 4)
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a1 = read!(input_ptr, len - 8)
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a2 = read!(input_ptr, len - 16)
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a3 = read!(input_ptr, len - 12)
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a4 = read!(input_ptr, len - 20)
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);

      h ^= a0;
      h = h.rotate_right(19).wrapping_mul(5).wrapping_add(0xE654_6B64);

      h ^= a2;
      h = h.rotate_right(19).wrapping_mul(5).wrapping_add(0xE654_6B64);

      g ^= a1;
      g = g.rotate_right(19).wrapping_mul(5).wrapping_add(0xE654_6B64);

      g ^= a3;
      g = g.rotate_right(19).wrapping_mul(5).wrapping_add(0xE654_6B64);

      f = f
        .wrapping_add(a4)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(0xE654_6B64);

      let mut offset: usize = 0;

      for _i in 0..((len - 1) / 20) {
        let a0 = read!(input_ptr, offset)
          .wrapping_mul(C1)
          .rotate_right(17)
          .wrapping_mul(C2);
        let a1 = read!(input_ptr, offset + 4);
        let a2 = read!(input_ptr, offset + 8)
          .wrapping_mul(C1)
          .rotate_right(17)
          .wrapping_mul(C2);
        let a3 = read!(input_ptr, offset + 12)
          .wrapping_mul(C1)
          .rotate_right(17)
          .wrapping_mul(C2);
        let a4 = read!(input_ptr, offset + 16);

        h ^= a0;
        h = h.rotate_right(18).wrapping_mul(5).wrapping_add(0xE654_6B64);
        f = f.wrapping_add(a1).rotate_right(19).wrapping_mul(C1);
        g = g
          .wrapping_add(a2)
          .rotate_right(18)
          .wrapping_mul(5)
          .wrapping_add(0xE654_6B64);
        h ^= a3.wrapping_add(a1);
        h = h.rotate_right(19).wrapping_mul(5).wrapping_add(0xE654_6B64);
        g ^= a4;
        g = g.swap_bytes().wrapping_mul(5);
        h = h.wrapping_add(a4.wrapping_mul(5));
        h = h.swap_bytes();
        f = f.wrapping_add(a0);

        permute3!(&mut f, &mut h, &mut g);
        offset += 20;
      }

      g = g
        .rotate_right(11)
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C1);
      f = f
        .rotate_right(11)
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C1);
      h = h
        .wrapping_add(g)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(0xE654_6B64)
        .rotate_right(17)
        .wrapping_mul(C1);
      h = h
        .wrapping_add(f)
        .rotate_right(19)
        .wrapping_mul(5)
        .wrapping_add(0xE654_6B64)
        .rotate_right(17)
        .wrapping_mul(C1);

      h
    }
  }
}

// 64-bit

macro_rules! shift_mix64 {
  ($val:expr) => {
    $val ^ ($val >> 47)
  };
}

#[inline]
fn hash_len_16_murmur(u: u64, v: u64, mul: u64) -> u64 {
  let mut a = (u ^ v).wrapping_mul(mul);
  a ^= a >> 47;

  let mut b = (v ^ a).wrapping_mul(mul);
  b ^= b >> 47;

  b.wrapping_mul(mul)
}

#[inline]
fn hash_len_16(low: u64, high: u64) -> u64 {
  let mut a: u64 = (low ^ high).wrapping_mul(K_MUL);
  a ^= a >> 47;

  let mut b: u64 = (high ^ a).wrapping_mul(K_MUL);
  b ^= b >> 47;

  b.wrapping_mul(K_MUL)
}

#[inline]
fn hash_64_0_to_16(input: &[u8]) -> u64 {
  let input_ptr: *const u8 = input.as_ptr();

  match input.len() {
    0 => K2,
    1..=3 => {
      let a: u32 = read!(input_ptr) as u32;
      let b: u32 = read!(input_ptr, input.len() >> 1) as u32;
      let c: u32 = read!(input_ptr, input.len() - 1) as u32;

      let y: u64 = ((a + (b << 8)) as u64).wrapping_mul(K2);
      let z: u64 = ((input.len() as u32).wrapping_add(c << 2) as u64).wrapping_mul(K0);

      shift_mix64!(y ^ z).wrapping_mul(K2)
    }
    4..=7 => {
      let input_ptr_32: *const u32 = input_ptr.cast();

      let mul: u64 = K2.wrapping_add((input.len() * 2) as u64);
      let a: u64 = read!(input_ptr_32) as u64;
      let b: u64 = read!(input_ptr_32, input.len() - 4) as u64;

      hash_len_16_murmur(a.wrapping_shl(3).wrapping_add(input.len() as u64), b, mul)
    }
    8..=16 => {
      let input_ptr_64: *const u64 = input_ptr.cast();

      let mul: u64 = K2.wrapping_add(input.len().wrapping_mul(2) as u64);
      let a: u64 = read!(input_ptr_64).wrapping_add(K2);
      let b: u64 = read!(input_ptr_64, input.len() - 8);
      let c: u64 = b.rotate_right(37).wrapping_mul(mul).wrapping_add(a);
      let d: u64 = a.rotate_right(25).wrapping_add(b).wrapping_mul(mul);

      hash_len_16_murmur(c, d, mul)
    }
    _ => unreachable!(),
  }
}

#[inline]
fn hash_64_17_to_32(input: &[u8]) -> u64 {
  let input_ptr: *const u64 = input.as_ptr().cast();

  let mul: u64 = K2.wrapping_add(input.len().wrapping_mul(2) as u64);
  let a: u64 = read!(input_ptr).wrapping_mul(K1);
  let b: u64 = read!(input_ptr, 8);
  let c: u64 = read!(input_ptr, input.len() - 8).wrapping_mul(mul);
  let d: u64 = read!(input_ptr, input.len() - 16).wrapping_mul(K2);
  let u: u64 = a
    .wrapping_add(b)
    .rotate_right(43)
    .wrapping_add(d)
    .wrapping_add(c.rotate_right(30));
  let v: u64 = b
    .wrapping_add(K2)
    .rotate_right(18)
    .wrapping_add(a)
    .wrapping_add(c);

  hash_len_16_murmur(u, v, mul)
}

#[inline]
fn hash_64_33_to_64(input: &[u8]) -> u64 {
  let input_ptr: *const u64 = input.as_ptr().cast();
  let len = input.len();

  let mul: u64 = K2.wrapping_add(len.wrapping_mul(2) as u64);
  let mut a: u64 = read!(input_ptr).wrapping_mul(K2);
  let mut b: u64 = read!(input_ptr, 8);
  let c: u64 = read!(input_ptr, len - 24);
  let d: u64 = read!(input_ptr, len - 32);
  let e: u64 = read!(input_ptr, 16).wrapping_mul(K2);
  let f: u64 = read!(input_ptr, 24).wrapping_mul(9);
  let g: u64 = read!(input_ptr, len - 8);
  let h: u64 = read!(input_ptr, len - 16).wrapping_mul(mul);

  let u: u64 = b
    .rotate_right(30)
    .wrapping_add(c)
    .wrapping_mul(9)
    .wrapping_add(a.wrapping_add(g).rotate_right(43));
  let v: u64 = (a.wrapping_add(g) ^ d).wrapping_add(f).wrapping_add(1);
  let w: u64 = u
    .wrapping_add(v)
    .wrapping_mul(mul)
    .swap_bytes()
    .wrapping_add(h);
  let x: u64 = e.wrapping_add(f).rotate_right(42).wrapping_add(c);
  let y: u64 = v
    .wrapping_add(w)
    .wrapping_mul(mul)
    .swap_bytes()
    .wrapping_add(g)
    .wrapping_mul(mul);
  let z: u64 = e.wrapping_add(f).wrapping_add(c);

  a = x
    .wrapping_add(z)
    .wrapping_mul(mul)
    .wrapping_add(y)
    .swap_bytes()
    .wrapping_add(b);

  b = shift_mix64!(z
    .wrapping_add(a)
    .wrapping_mul(mul)
    .wrapping_add(d)
    .wrapping_add(h));

  b.wrapping_mul(mul).wrapping_add(x)
}

#[inline]
fn weakhash_len_32_with_seed(w: u64, x: u64, y: u64, z: u64, mut a: u64, mut b: u64) -> (u64, u64) {
  a = a.wrapping_add(w);
  b = b.wrapping_add(a).wrapping_add(z).rotate_right(21);

  let c: u64 = a;

  a = a.wrapping_add(x).wrapping_add(y);
  b = b.wrapping_add(a.rotate_right(44));

  (a.wrapping_add(z), b.wrapping_add(c))
}

macro_rules! weakhash_len_32_from {
  ($ptr:expr, $offset:expr ,$a:expr, $b:expr) => {
    $crate::cityhash::hash_fn::weakhash_len_32_with_seed(
      read!($ptr, $offset),
      read!($ptr, $offset + 8),
      read!($ptr, $offset + 16),
      read!($ptr, $offset + 24),
      $a,
      $b,
    )
  };
}

pub fn cityhash_64(input: &[u8]) -> u64 {
  match input.len() {
    0..=16 => hash_64_0_to_16(input),
    17..=32 => hash_64_17_to_32(input),
    33..=64 => hash_64_33_to_64(input),
    _ => {
      let input_ptr: *const u64 = input.as_ptr().cast();
      let mut len = input.len();

      let mut x: u64 = read!(input_ptr, len - 40);
      let mut y: u64 = read!(input_ptr, len - 16).wrapping_add(read!(input_ptr, len - 56));
      let mut z: u64 = hash_len_16(
        read!(input_ptr, len - 48).wrapping_add(len as u64),
        read!(input_ptr, len - 24),
      );
      let mut v: (u64, u64) = weakhash_len_32_from!(input_ptr, len - 64, len as u64, z);
      let mut w: (u64, u64) = weakhash_len_32_from!(input_ptr, len - 32, y.wrapping_add(K1), x);
      x = x.wrapping_mul(K1).wrapping_add(read!(input_ptr));

      let mut offset: usize = 0;
      len = (len - 1) & !63;
      loop {
        x = x
          .wrapping_add(y)
          .wrapping_add(v.0)
          .wrapping_add(read!(input_ptr, offset + 8))
          .rotate_right(37)
          .wrapping_mul(K1);
        y = y
          .wrapping_add(v.1)
          .wrapping_add(read!(input_ptr, offset + 48))
          .rotate_right(42)
          .wrapping_mul(K1);
        x ^= w.1;
        y = y
          .wrapping_add(v.0)
          .wrapping_add(read!(input_ptr, offset + 40));
        z = z.wrapping_add(w.0).rotate_right(33).wrapping_mul(K1);
        v = weakhash_len_32_from!(input_ptr, offset, v.1.wrapping_mul(K1), x.wrapping_add(w.0));
        w = weakhash_len_32_from!(
          input_ptr,
          offset + 32,
          z.wrapping_add(w.1),
          y.wrapping_add(read!(input_ptr, offset + 16))
        );

        core::mem::swap(&mut z, &mut x);

        offset += 64;
        len -= 64;

        if len == 0 {
          break;
        }
      }

      let low = shift_mix64!(y)
        .wrapping_mul(K1)
        .wrapping_add(z)
        .wrapping_add(hash_len_16(v.0, w.0));
      let high = hash_len_16(v.1, w.1).wrapping_add(x);

      hash_len_16(low, high)
    }
  }
}

pub fn cityhash_64_with_seed(input: &[u8], seed0: u64, seed1: u64) -> u64 {
  let hash: u64 = cityhash_64(input);
  hash_len_16(hash.wrapping_sub(seed0), seed1)
}

// 128-bit

#[inline]
fn city_murmur(input: &[u8], seed0: u64, seed1: u64) -> u128 {
  let input_ptr: *const u64 = input.as_ptr().cast();
  let mut a: u64 = seed0;
  let mut b: u64 = seed1;
  let mut c: u64;
  let mut d: u64;

  if input.len() <= 16 {
    a = shift_mix64!(a.wrapping_mul(K1)).wrapping_mul(K1);
    c = b.wrapping_mul(K1).wrapping_add(hash_64_0_to_16(input));

    let d1 = if input.len() >= 8 {
      read!(input_ptr)
    } else {
      c
    };

    d = shift_mix64!(a.wrapping_add(d1));
  } else {
    c = hash_len_16(read!(input_ptr, input.len() - 8).wrapping_add(K1), a);
    d = hash_len_16(
      b.wrapping_add(input.len() as u64),
      c.wrapping_add(read!(input_ptr, input.len() - 16)),
    );
    a = a.wrapping_add(d);

    let mut offset: usize = 0;
    let mut len: usize = input.len();
    loop {
      a ^= shift_mix64!(read!(input_ptr, offset).wrapping_mul(K1)).wrapping_mul(K1);
      a = a.wrapping_mul(K1);
      b ^= a;
      c ^= shift_mix64!(read!(input_ptr, offset + 8).wrapping_mul(K1)).wrapping_mul(K1);
      c = c.wrapping_mul(K1);
      d ^= c;

      len -= 16;
      offset += 16;

      if len <= 16 {
        break;
      }
    }
  }

  a = hash_len_16(a, c);
  b = hash_len_16(d, b);

  u128_from!(a ^ b, hash_len_16(b, a))
}

pub fn cityhash_128_with_seed(input: &[u8], seed0: u64, seed1: u64) -> u128 {
  if input.len() < 128 {
    return city_murmur(input, seed0, seed1);
  }

  let input_ptr: *const u64 = input.as_ptr().cast();
  let mut v: (u64, u64) = (0, 0);
  let mut w: (u64, u64) = (0, 0);
  let mut x: u64 = seed0;
  let mut y: u64 = seed1;
  let mut z: u64 = (input.len() as u64).wrapping_mul(K1);
  v.0 = (y ^ K1)
    .rotate_right(49)
    .wrapping_mul(K1)
    .wrapping_add(read!(input_ptr));
  v.1 = v
    .0
    .rotate_right(42)
    .wrapping_mul(K1)
    .wrapping_add(read!(input_ptr, 8));
  w.0 = y
    .wrapping_add(z)
    .rotate_right(35)
    .wrapping_mul(K1)
    .wrapping_add(x);
  w.1 = x
    .wrapping_add(read!(input_ptr, 88))
    .rotate_right(53)
    .wrapping_mul(K1);

  let mut len: usize = input.len();
  let mut offset: usize = 0;
  loop {
    x = x
      .wrapping_add(y)
      .wrapping_add(v.0)
      .wrapping_add(read!(input_ptr, offset + 8))
      .rotate_right(37)
      .wrapping_mul(K1);
    y = y
      .wrapping_add(v.1)
      .wrapping_add(read!(input_ptr, offset + 48))
      .rotate_right(42)
      .wrapping_mul(K1);
    x ^= w.1;
    y = y
      .wrapping_add(v.0)
      .wrapping_add(read!(input_ptr, offset + 40));
    z = z.wrapping_add(w.0).rotate_right(33).wrapping_mul(K1);
    v = weakhash_len_32_from!(input_ptr, offset, v.1.wrapping_mul(K1), x.wrapping_add(w.0));
    w = weakhash_len_32_from!(
      input_ptr,
      offset + 32,
      z.wrapping_add(w.1),
      y.wrapping_add(read!(input_ptr, offset + 16))
    );

    core::mem::swap(&mut z, &mut x);
    offset += 64;

    x = x
      .wrapping_add(y)
      .wrapping_add(v.0)
      .wrapping_add(read!(input_ptr, offset + 8))
      .rotate_right(37)
      .wrapping_mul(K1);
    y = y
      .wrapping_add(v.1)
      .wrapping_add(read!(input_ptr, offset + 48))
      .rotate_right(42)
      .wrapping_mul(K1);
    x ^= w.1;
    y = y
      .wrapping_add(v.0)
      .wrapping_add(read!(input_ptr, offset + 40));
    z = z.wrapping_add(w.0).rotate_right(33).wrapping_mul(K1);
    v = weakhash_len_32_from!(input_ptr, offset, v.1.wrapping_mul(K1), x.wrapping_add(w.0));
    w = weakhash_len_32_from!(
      input_ptr,
      offset + 32,
      z.wrapping_add(w.1),
      y.wrapping_add(read!(input_ptr, offset + 16))
    );

    core::mem::swap(&mut z, &mut x);
    offset += 64;

    len -= 128;
    if len < 128 {
      break;
    }
  }

  x = x.wrapping_add(v.0.wrapping_add(z).rotate_right(49).wrapping_mul(K0));
  y = y.wrapping_mul(K0).wrapping_add(w.1.rotate_right(37));
  z = z.wrapping_mul(K0).wrapping_add(w.0.rotate_right(27));
  w.0 = w.0.wrapping_mul(9);
  v.0 = v.0.wrapping_mul(K0);

  let mut tail_done: usize = 0;

  loop {
    if tail_done >= len {
      break;
    }

    tail_done += 32;
    y = x
      .wrapping_add(y)
      .rotate_right(42)
      .wrapping_mul(K0)
      .wrapping_add(v.1);
    w.0 = w
      .0
      .wrapping_add(read!(input_ptr, offset + len - tail_done + 16));
    x = x.wrapping_mul(K0).wrapping_add(w.0);
    z = z
      .wrapping_add(w.1)
      .wrapping_add(read!(input_ptr, offset + len - tail_done));
    w.1 = w.1.wrapping_add(v.0);
    v = weakhash_len_32_from!(
      input_ptr,
      offset + len - tail_done,
      v.0.wrapping_add(z),
      v.1
    );
    v.0 = v.0.wrapping_mul(K0);
  }

  x = hash_len_16(x, v.0);
  y = hash_len_16(y.wrapping_add(z), w.0);

  u128_from!(
    hash_len_16(x.wrapping_add(v.1), w.1).wrapping_add(y),
    hash_len_16(x.wrapping_add(w.1), y.wrapping_add(v.1))
  )
}

pub fn cityhash_128(input: &[u8]) -> u128 {
  if input.len() >= 16 {
    let ptr: *const u64 = input.as_ptr().cast();
    let low: u64 = read!(ptr);
    let high: u64 = read!(ptr, 8).wrapping_add(K0);

    cityhash_128_with_seed(&input[16..], low, high)
  } else {
    cityhash_128_with_seed(input, K0, K1)
  }
}

// CRC 256-bit

#[inline]
fn cityhash_crc256_long(input: &[u8], seed: u64) -> super::U256 {
  let input_ptr: *const u64 = input.as_ptr().cast();
  let mut result = super::U256(0, 0, 0, 0);

  let mut a: u64 = read!(input_ptr, 56).wrapping_add(K0);
  let mut b: u64 = read!(input_ptr, 96).wrapping_add(K0);
  let mut c: u64 = hash_len_16(b, input.len() as u64);
  let mut d: u64 = read!(input_ptr, 120)
    .wrapping_mul(K0)
    .wrapping_add(input.len() as u64);

  result.0 = c;
  result.1 = d;

  let mut e: u64 = read!(input_ptr, 184).wrapping_add(seed);
  let mut f: u64 = 0;
  let mut g: u64 = 0;
  let mut h: u64 = c.wrapping_add(d);
  let mut x: u64 = seed;
  let mut y: u64 = 0;
  let mut z: u64 = 0;

  let mut len: usize = input.len();
  let mut iters: usize = len / 240;
  let mut offset: usize = 0;
  len -= iters.wrapping_mul(240);

  macro_rules! chunk {
    ($r:expr) => {
      permute3!(&mut x, &mut z, &mut y);
      b = b.wrapping_add(read!(input_ptr, offset));
      c = c.wrapping_add(read!(input_ptr, offset + 8));
      d = d.wrapping_add(read!(input_ptr, offset + 16));
      e = e.wrapping_add(read!(input_ptr, offset + 24));
      f = f.wrapping_add(read!(input_ptr, offset + 32));
      a = a.wrapping_add(b);
      h = h.wrapping_add(f);
      b = b.wrapping_add(c);
      f = f.wrapping_add(d);
      g = g.wrapping_add(e);
      e = e.wrapping_add(z);
      g = g.wrapping_add(x);
      z = $crate::crc::hash_fn::crc32c_u64_step(z as u32, b.wrapping_add(g)) as u64;
      y = $crate::crc::hash_fn::crc32c_u64_step(y as u32, e.wrapping_add(h)) as u64;
      x = $crate::crc::hash_fn::crc32c_u64_step(x as u32, f.wrapping_add(a)) as u64;
      e = e.rotate_right($r);
      c = c.wrapping_add(e);
      offset += 40;
    };
  }

  loop {
    chunk!(0_u32);
    permute3!(&mut a, &mut h, &mut c);
    chunk!(33_u32);
    permute3!(&mut a, &mut h, &mut f);
    chunk!(0_u32);
    permute3!(&mut b, &mut h, &mut f);
    chunk!(42_u32);
    permute3!(&mut b, &mut h, &mut d);
    chunk!(0_u32);
    permute3!(&mut b, &mut h, &mut e);
    chunk!(33_u32);
    permute3!(&mut a, &mut h, &mut e);

    iters -= 1;
    if iters == 0 {
      break;
    }
  }

  while len >= 40 {
    chunk!(29);
    e ^= a.rotate_right(20);
    h = h.wrapping_add(b.rotate_right(30));
    g ^= c.rotate_right(40);
    f = f.wrapping_add(d.rotate_right(34));
    permute3!(&mut c, &mut h, &mut g);
    len -= 40;
  }

  if len > 0 {
    offset = (offset + len) - 40;
    chunk!(33);
    e ^= a.rotate_right(43);
    h = h.wrapping_add(b.rotate_right(42));
    g ^= c.rotate_right(41);
    f = f.wrapping_add(d.rotate_right(40));
  }

  result.0 ^= h;
  result.1 ^= g;
  g = g.wrapping_add(h);
  a = hash_len_16(a, g.wrapping_add(z));
  x = x.wrapping_add(y << 32);
  b = b.wrapping_add(x);
  c = hash_len_16(c, z).wrapping_add(h);
  d = hash_len_16(d, e.wrapping_add(result.0));
  g = g.wrapping_add(e);
  h = h.wrapping_add(hash_len_16(x, f));
  e = hash_len_16(a, d).wrapping_add(g);
  z = hash_len_16(b, c).wrapping_add(a);
  y = hash_len_16(g, h).wrapping_add(c);
  result.0 = e.wrapping_add(z).wrapping_add(y).wrapping_add(x);
  a = shift_mix64!(a.wrapping_add(y).wrapping_mul(K0))
    .wrapping_mul(K0)
    .wrapping_add(b);
  result.1 = result.1.wrapping_add(a).wrapping_add(result.0);
  a = shift_mix64!(a.wrapping_mul(K0))
    .wrapping_mul(K0)
    .wrapping_add(c);
  result.2 = a.wrapping_add(result.1);
  a = shift_mix64!(a.wrapping_add(e).wrapping_mul(K0)).wrapping_mul(K0);
  result.3 = a.wrapping_add(result.2);

  result
}

#[inline]
fn cityhash_crc256_short(input: &[u8]) -> super::U256 {
  let padded_buf: &mut [u8] = &mut [0; 240];
  unsafe {
    input
      .as_ptr()
      .copy_to_nonoverlapping(padded_buf.as_mut_ptr(), input.len());
  };

  cityhash_crc256_long(padded_buf, (!input.len() as u32) as u64)
}

pub fn cityhash_crc256(input: &[u8]) -> super::U256 {
  if input.len() >= 240 {
    cityhash_crc256_long(input, 0)
  } else {
    cityhash_crc256_short(input)
  }
}

// CRC 128-bit

pub fn cityhash_crc128_with_seed(input: &[u8], seed0: u64, seed1: u64) -> u128 {
  if input.len() <= 900 {
    cityhash_128_with_seed(input, seed0, seed1)
  } else {
    let result = cityhash_crc256(input);
    let u: u64 = seed1.wrapping_add(result.0);
    let v: u64 = seed0.wrapping_add(result.1);
    let low: u64 = hash_len_16(u, v.wrapping_add(result.2));
    let high: u64 = hash_len_16(
      v.rotate_right(32),
      u.wrapping_mul(K0).wrapping_add(result.3),
    );

    u128_from!(low, high)
  }
}

pub fn cityhash_crc128(input: &[u8]) -> u128 {
  if input.len() <= 900 {
    cityhash_128(input)
  } else {
    let result = cityhash_crc256(input);

    u128_from!(result.2, result.3)
  }
}

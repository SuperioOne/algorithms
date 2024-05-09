const K0: u64 = 0xC3A5_C85C_97CB_3127;
const K1: u64 = 0xB492_B66F_BE98_F273;
const K2: u64 = 0x9AE1_6A3B_2F90_404F;
const C1: u32 = 0xcc9e2d51;
const C2: u32 = 0x1b873593;

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

macro_rules! permute3 {
  ($a:expr, $b:expr, $c:expr) => {
    core::mem::swap($a, $b);
    std::mem::swap($a, $c);
  };
}

fn mur3(mut a: u32, mut h: u32) -> u32 {
  a = a.wrapping_mul(C1).rotate_right(17).wrapping_mul(C2);
  h ^= a;
  h = h.rotate_right(19).wrapping_mul(5).wrapping_add(0xE654_6B64);
  h
}

#[inline]
fn hash_32_len_0_to_4(input: &[u8]) -> u32 {
  let mut b: u32 = 0;
  let mut c: u32 = 9;

  for byte in input {
    b = b.wrapping_mul(C1).wrapping_add(*byte as u32);
    c ^= b;
  }

  fmix32!(mur3(b, mur3(input.len() as u32, c)))
}

#[inline]
fn hash_32_len_5_to_12(input: &[u8]) -> u32 {
  let mut a: u32 = input.len() as u32;
  let mut b: u32 = a.wrapping_mul(5);
  let mut c: u32 = 9;
  let d = b;
  let input_ptr: *const u32 = input.as_ptr().cast();

  a = a.wrapping_add(unsafe { input_ptr.read_unaligned() });
  b = b.wrapping_add(unsafe { input_ptr.byte_add(input.len() - 4).read_unaligned() });
  c = c.wrapping_add(unsafe {
    input_ptr
      .byte_add(input.len().wrapping_shr(1) & 4)
      .read_unaligned()
  });

  fmix32!(mur3(c, mur3(b, mur3(a, d))))
}

#[inline]
fn hash_32_len_13_to_24(input: &[u8]) -> u32 {
  let input_ptr: *const u32 = input.as_ptr().cast();

  let a: u32 = unsafe {
    input_ptr
      .byte_add(input.len().wrapping_shr(1) - 4)
      .read_unaligned()
  };
  let b: u32 = unsafe { input_ptr.byte_add(4).read_unaligned() };
  let c: u32 = unsafe { input_ptr.byte_add(input.len() - 8).read_unaligned() };
  let d: u32 = unsafe {
    input_ptr
      .byte_add(input.len().wrapping_shr(1))
      .read_unaligned()
  };
  let e: u32 = unsafe { input_ptr.read_unaligned() };
  let f: u32 = unsafe { input_ptr.byte_add(input.len() - 4).read_unaligned() };
  let h: u32 = input.len() as u32;

  fmix32!(mur3(f, mur3(e, mur3(d, mur3(c, mur3(b, mur3(a, h)))))))
}

pub fn city_hash_32(input: &[u8]) -> u32 {
  match input.len() {
    0..=4 => hash_32_len_0_to_4(input),
    5..=12 => hash_32_len_5_to_12(input),
    13..=24 => hash_32_len_13_to_24(input),
    _ => {
      let len = input.len();
      let mut h: u32 = len as u32;
      let mut g: u32 = C1.wrapping_mul(h);
      let mut f: u32 = g;
      let input_ptr: *const u32 = input.as_ptr().cast();

      let a0 = unsafe { input_ptr.byte_add(len - 4).read_unaligned() }
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a1 = unsafe { input_ptr.byte_add(len - 8).read_unaligned() }
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a2 = unsafe { input_ptr.byte_add(len - 16).read_unaligned() }
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a3 = unsafe { input_ptr.byte_add(len - 12).read_unaligned() }
        .wrapping_mul(C1)
        .rotate_right(17)
        .wrapping_mul(C2);
      let a4 = unsafe { input_ptr.byte_add(len - 20).read_unaligned() }
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
        let a0 = unsafe { input_ptr.byte_add(offset).read_unaligned() }
          .wrapping_mul(C1)
          .rotate_right(17)
          .wrapping_mul(C2);

        let a1 = unsafe { input_ptr.byte_add(offset + 4).read_unaligned() };

        let a2 = unsafe { input_ptr.byte_add(offset + 8).read_unaligned() }
          .wrapping_mul(C1)
          .rotate_right(17)
          .wrapping_mul(C2);

        let a3 = unsafe { input_ptr.byte_add(offset + 12).read_unaligned() }
          .wrapping_mul(C1)
          .rotate_right(17)
          .wrapping_mul(C2);

        let a4 = unsafe { input_ptr.byte_add(offset + 16).read_unaligned() };

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

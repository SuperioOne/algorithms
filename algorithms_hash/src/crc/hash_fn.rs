// TODO: Add ARM fallbacks https://developer.arm.com/documentation/dui0801/g/A64-General-Instructions/CRC32CB--CRC32CH--CRC32CW--CRC32CX

macro_rules! read {
  ($ptr:expr, $offset:expr) => {
    unsafe { $ptr.byte_add($offset).read_unaligned() }
  };

  ($ptr:expr) => {
    unsafe { $ptr.read_unaligned() }
  };
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u64_step(crc: u32, value: u64) -> u32 {
  unsafe { core::arch::x86_64::_mm_crc32_u64(crc as u64, value) as u32 }
}

#[cfg(not(target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u64_step(crc: u32, value: u64) -> u32 {
  use super::crc32c_table::CRC32C_TABLE;

  let mut rl: u32 = crc ^ (value as u32);
  let rh: u32 = (value >> 32) as u32;

  let mut t1: u32 = CRC32C_TABLE[7][(rl & 0x000000FF) as usize]
    ^ CRC32C_TABLE[6][((rl >> 8) & 0x000000FF) as usize];
  let mut t2: u32 = rl >> 16;

  rl = t1
    ^ CRC32C_TABLE[5][(t2 & 0x000000FF) as usize]
    ^ CRC32C_TABLE[4][((t2 >> 8) & 0x000000FF) as usize];

  t1 = CRC32C_TABLE[3][(rh & 0x000000FF) as usize]
    ^ CRC32C_TABLE[2][((rh >> 8) & 0x000000FF) as usize];
  t2 = rh >> 16;

  rl ^ t1
    ^ CRC32C_TABLE[1][(t2 & 0x000000FF) as usize]
    ^ CRC32C_TABLE[0][((t2 >> 8) & 0x000000FF) as usize]
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u32_step(crc: u32, value: u32) -> u32 {
  unsafe { core::arch::x86_64::_mm_crc32_u32(crc, value) }
}

#[cfg(not(target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u32_step(crc: u32, value: u32) -> u32 {
  use super::crc32c_table::CRC32C_TABLE;

  let rl: u32 = crc ^ value;
  let t1 = CRC32C_TABLE[3][(rl & 0x000000FF) as usize]
    ^ CRC32C_TABLE[2][((rl >> 8) & 0x000000FF) as usize];
  let t2 = rl >> 16;

  t1 ^ CRC32C_TABLE[1][(t2 & 0x000000FF) as usize]
    ^ CRC32C_TABLE[0][((t2 >> 8) & 0x000000FF) as usize]
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u16_step(crc: u32, value: u16) -> u32 {
  unsafe { core::arch::x86_64::_mm_crc32_u16(crc, value) }
}

#[cfg(not(target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u16_step(crc: u32, value: u16) -> u32 {
  use super::crc32c_table::CRC32C_TABLE;

  let rl: u32 = crc ^ (value as u32);

  (crc >> 16)
    ^ CRC32C_TABLE[1][(rl & 0x000000FF) as usize]
    ^ CRC32C_TABLE[0][((rl >> 8) & 0x000000FF) as usize]
}

#[cfg(all(target_arch = "x86_64", target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u8_step(crc: u32, value: u8) -> u32 {
  unsafe { core::arch::x86_64::_mm_crc32_u8(crc, value) }
}

#[cfg(not(target_feature = "sse4.2"))]
#[inline(always)]
pub fn crc32c_u8_step(crc: u32, value: u8) -> u32 {
  use super::crc32c_table::CRC32C_TABLE;

  let rl: u32 = crc ^ (value as u32);
  (crc >> 8) ^ CRC32C_TABLE[0][(rl & 0x000000FF) as usize]
}

pub fn crc32c_with_initial(input: &[u8], initial: u32) -> u32 {
  let mut crc: u32 = !initial;
  let mut offset: usize = 0;
  let len: usize = input.len() / 8;
  let input_ptr: *const u64 = input.as_ptr().cast();

  for _i in 0..len {
    crc = crc32c_u64_step(crc, read!(input_ptr, offset));
    offset += 8;
  }

  let tail_len = input.len() & 7;

  if tail_len > 0 {
    let tail_ptr: *const u8 = unsafe { input.as_ptr().byte_add(offset) };

    match tail_len {
      1 => {
        crc = crc32c_u8_step(crc, read!(tail_ptr));
      }
      2 => {
        let tail_u16: *const u16 = tail_ptr.cast();
        crc = crc32c_u16_step(crc, read!(tail_u16));
      }
      3 => {
        let tail_u16: *const u16 = tail_ptr.cast();
        crc = crc32c_u16_step(crc, read!(tail_u16));
        crc = crc32c_u8_step(crc, read!(tail_ptr, 2));
      }
      4 => {
        let tail_u32: *const u32 = tail_ptr.cast();
        crc = crc32c_u32_step(crc, read!(tail_u32));
      }
      5 => {
        let tail_u32: *const u32 = tail_ptr.cast();
        crc = crc32c_u32_step(crc, read!(tail_u32));
        crc = crc32c_u8_step(crc, read!(tail_ptr, 4));
      }
      6 => {
        let tail_u32: *const u32 = tail_ptr.cast();
        crc = crc32c_u32_step(crc, read!(tail_u32));
        let tail_u16: *const u16 = unsafe { tail_ptr.byte_add(4).cast() };
        crc = crc32c_u16_step(crc, read!(tail_u16));
      }
      7 => {
        let tail_u32: *const u32 = tail_ptr.cast();
        crc = crc32c_u32_step(crc, read!(tail_u32));
        let tail_u16: *const u16 = unsafe { tail_ptr.byte_add(4).cast() };
        crc = crc32c_u16_step(crc, read!(tail_u16));
        crc = crc32c_u8_step(crc, read!(tail_ptr, 6));
      }
      _ => unreachable!(),
    }
  }

  !crc
}

pub fn crc32c(input: &[u8]) -> u32 {
  crc32c_with_initial(input, 0)
}

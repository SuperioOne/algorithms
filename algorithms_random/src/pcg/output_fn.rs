#[inline]
pub fn pcg_output_xsh_rs_16_8(state: u16) -> u8 {
  (((state >> 7) ^ state) >> ((state >> 14).wrapping_add(3))) as u8
}

#[inline]
pub fn pcg_output_xsh_rs_32_16(state: u32) -> u16 {
  (((state >> 11) ^ state) >> ((state >> 30).wrapping_add(11))) as u16
}

#[inline]
pub fn pcg_output_xsh_rs_64_32(state: u64) -> u32 {
  (((state >> 22) ^ state) >> ((state >> 61).wrapping_add(22))) as u32
}

#[inline]
pub fn pcg_output_xsh_rs_128_64(state: u128) -> u64 {
  (((state >> 43) ^ state) >> ((state >> 124).wrapping_add(45))) as u64
}

#[inline]
pub fn pcg_output_xsh_rr_16_8(state: u16) -> u8 {
  u8::rotate_right((((state >> 5) ^ state) >> 5) as u8, (state >> 13) as u32)
}

#[inline]
pub fn pcg_output_xsh_rr_32_16(state: u32) -> u16 {
  u16::rotate_right((((state >> 10) ^ state) >> 12) as u16, state >> 28)
}

#[inline]
pub fn pcg_output_xsh_rr_64_32(state: u64) -> u32 {
  u32::rotate_right((((state >> 18) ^ state) >> 27) as u32, (state >> 59) as u32)
}

#[inline]
pub fn pcg_output_xsh_rr_128_64(state: u128) -> u64 {
  u64::rotate_right(
    (((state >> 35) ^ state) >> 58) as u64,
    (state >> 122) as u32,
  )
}

#[inline]
pub fn pcg_output_rxs_m_xs_8_8(state: u8) -> u8 {
  let word = ((state >> ((state >> 6).wrapping_add(2))) ^ state).wrapping_mul(217u8);
  (word >> 6) ^ word
}

#[inline]
pub fn pcg_output_rxs_m_xs_16_16(state: u16) -> u16 {
  let word = ((state >> ((state >> 13).wrapping_add(3))) ^ state).wrapping_mul(62169u16);
  (word >> 11) ^ word
}

#[inline]
pub fn pcg_output_rxs_m_xs_32_32(state: u32) -> u32 {
  let word = ((state >> ((state >> 28).wrapping_add(4))) ^ state).wrapping_mul(277803737u32);
  (word >> 22) ^ word
}

#[inline]
pub fn pcg_output_rxs_m_xs_64_64(state: u64) -> u64 {
  let word =
    ((state >> ((state >> 59).wrapping_add(5))) ^ state).wrapping_mul(12605985483714917081u64);
  (word >> 43) ^ word
}

#[inline]
pub fn pcg_output_rxs_m_xs_128_128(state: u128) -> u128 {
  let word = ((state >> ((state >> 122).wrapping_add(6))) ^ state)
    .wrapping_mul((17766728186571221404_u128 << 64).wrapping_add(12605985483714917081_u128));
  (word >> 86) ^ word
}

#[inline]
pub fn pcg_output_xsl_rr_64_32(state: u64) -> u32 {
  u32::rotate_right((state >> 32) as u32 ^ (state as u32), (state >> 59) as u32)
}

#[inline]
pub fn pcg_output_xsl_rr_128_64(state: u128) -> u64 {
  u64::rotate_right((state >> 64) as u64 ^ (state as u64), (state >> 122) as u32)
}

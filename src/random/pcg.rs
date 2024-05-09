#![allow(non_camel_case_types)]

mod output_fn;
mod step_fn;

use crate::random::pcg::output_fn::pcg_output_xsh_rr_16_8;
use crate::random::pcg::step_fn::pcg_mcg_16_step_r;

use self::output_fn::{
  pcg_output_rxs_m_xs_128_128, pcg_output_rxs_m_xs_16_16, pcg_output_rxs_m_xs_32_32,
  pcg_output_rxs_m_xs_64_64, pcg_output_rxs_m_xs_8_8, pcg_output_xsh_rr_128_64,
  pcg_output_xsh_rr_32_16, pcg_output_xsh_rr_64_32, pcg_output_xsh_rs_128_64,
  pcg_output_xsh_rs_16_8, pcg_output_xsh_rs_32_16, pcg_output_xsh_rs_64_32,
  pcg_output_xsl_rr_128_64, pcg_output_xsl_rr_64_32,
};
use self::step_fn::{
  pcg_mcg_128_step_r, pcg_mcg_32_step_r, pcg_mcg_64_step_r, pcg_oneseq_128_step_r,
  pcg_oneseq_16_step_r, pcg_oneseq_32_step_r, pcg_oneseq_64_step_r, pcg_oneseq_8_step_r,
};

use super::NumberGenerator;

// MCG XSH RR

pub struct Pcg_Mcg_16_XSH_RR_8 {
  state: u16,
}

impl Pcg_Mcg_16_XSH_RR_8 {
  pub fn new(seed: u16) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u16) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u16 {
    self.state
  }
}

impl Default for Pcg_Mcg_16_XSH_RR_8 {
  fn default() -> Self {
    Self::new(<u16>::default())
  }
}

impl NumberGenerator<u8> for Pcg_Mcg_16_XSH_RR_8 {
  fn get_next(&mut self) -> u8 {
    self.state = pcg_mcg_16_step_r(self.state);
    pcg_output_xsh_rr_16_8(self.state)
  }
}

impl Iterator for Pcg_Mcg_16_XSH_RR_8 {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_Mcg_32_XSH_RR_16 {
  state: u32,
}

impl Pcg_Mcg_32_XSH_RR_16 {
  pub fn new(seed: u32) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u32) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u32 {
    self.state
  }
}

impl Default for Pcg_Mcg_32_XSH_RR_16 {
  fn default() -> Self {
    Self::new(<u32>::default())
  }
}

impl NumberGenerator<u16> for Pcg_Mcg_32_XSH_RR_16 {
  fn get_next(&mut self) -> u16 {
    self.state = pcg_mcg_32_step_r(self.state);
    pcg_output_xsh_rr_32_16(self.state)
  }
}

impl Iterator for Pcg_Mcg_32_XSH_RR_16 {
  type Item = u16;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_Mcg_64_XSH_RR_32 {
  state: u64,
}

impl Pcg_Mcg_64_XSH_RR_32 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Pcg_Mcg_64_XSH_RR_32 {
  fn default() -> Self {
    Self::new(<u64>::default())
  }
}

impl NumberGenerator<u32> for Pcg_Mcg_64_XSH_RR_32 {
  fn get_next(&mut self) -> u32 {
    self.state = pcg_mcg_64_step_r(self.state);
    pcg_output_xsh_rr_64_32(self.state)
  }
}

impl Iterator for Pcg_Mcg_64_XSH_RR_32 {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_Mcg_128_XSH_RR_64 {
  state: u128,
}

impl Pcg_Mcg_128_XSH_RR_64 {
  pub fn new(seed: u128) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u128 {
    self.state
  }
}

impl Default for Pcg_Mcg_128_XSH_RR_64 {
  fn default() -> Self {
    Self::new(<u128>::default())
  }
}

impl NumberGenerator<u64> for Pcg_Mcg_128_XSH_RR_64 {
  fn get_next(&mut self) -> u64 {
    self.state = pcg_mcg_128_step_r(self.state);
    pcg_output_xsh_rr_128_64(self.state)
  }
}

impl Iterator for Pcg_Mcg_128_XSH_RR_64 {
  type Item = u64;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// MCG XSH RS

pub struct Pcg_Mcg_16_XSH_RS_8 {
  state: u16,
}

impl Pcg_Mcg_16_XSH_RS_8 {
  pub fn new(seed: u16) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u16) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u16 {
    self.state
  }
}

impl Default for Pcg_Mcg_16_XSH_RS_8 {
  fn default() -> Self {
    Self::new(<u16>::default())
  }
}

impl NumberGenerator<u8> for Pcg_Mcg_16_XSH_RS_8 {
  fn get_next(&mut self) -> u8 {
    self.state = pcg_mcg_16_step_r(self.state);
    pcg_output_xsh_rs_16_8(self.state)
  }
}

impl Iterator for Pcg_Mcg_16_XSH_RS_8 {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_Mcg_32_XSH_RS_16 {
  state: u32,
}

impl Pcg_Mcg_32_XSH_RS_16 {
  pub fn new(seed: u32) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u32) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u32 {
    self.state
  }
}

impl Default for Pcg_Mcg_32_XSH_RS_16 {
  fn default() -> Self {
    Self::new(<u32>::default())
  }
}

impl NumberGenerator<u16> for Pcg_Mcg_32_XSH_RS_16 {
  fn get_next(&mut self) -> u16 {
    self.state = pcg_mcg_32_step_r(self.state);
    pcg_output_xsh_rs_32_16(self.state)
  }
}

impl Iterator for Pcg_Mcg_32_XSH_RS_16 {
  type Item = u16;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_Mcg_64_XSH_RS_32 {
  state: u64,
}

impl Pcg_Mcg_64_XSH_RS_32 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Pcg_Mcg_64_XSH_RS_32 {
  fn default() -> Self {
    Self::new(<u64>::default())
  }
}

impl NumberGenerator<u32> for Pcg_Mcg_64_XSH_RS_32 {
  fn get_next(&mut self) -> u32 {
    self.state = pcg_mcg_64_step_r(self.state);
    pcg_output_xsh_rs_64_32(self.state)
  }
}

impl Iterator for Pcg_Mcg_64_XSH_RS_32 {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_Mcg_128_XSH_RS_64 {
  state: u128,
}

impl Pcg_Mcg_128_XSH_RS_64 {
  pub fn new(seed: u128) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u128 {
    self.state
  }
}

impl Default for Pcg_Mcg_128_XSH_RS_64 {
  fn default() -> Self {
    Self::new(<u128>::default())
  }
}

impl NumberGenerator<u64> for Pcg_Mcg_128_XSH_RS_64 {
  fn get_next(&mut self) -> u64 {
    self.state = pcg_mcg_128_step_r(self.state);
    pcg_output_xsh_rs_128_64(self.state)
  }
}

impl Iterator for Pcg_Mcg_128_XSH_RS_64 {
  type Item = u64;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ XHS RR Impls

pub struct Pcg_OneSeq_16_XSH_RR_8 {
  state: u16,
}

impl Pcg_OneSeq_16_XSH_RR_8 {
  pub fn new(seed: u16) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u16) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u16 {
    self.state
  }
}

impl Default for Pcg_OneSeq_16_XSH_RR_8 {
  fn default() -> Self {
    Self::new(<u16>::default())
  }
}

impl NumberGenerator<u8> for Pcg_OneSeq_16_XSH_RR_8 {
  fn get_next(&mut self) -> u8 {
    self.state = pcg_oneseq_16_step_r(self.state);
    pcg_output_xsh_rr_16_8(self.state)
  }
}

impl Iterator for Pcg_OneSeq_16_XSH_RR_8 {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_32_XSH_RR_16 {
  state: u32,
}

impl Pcg_OneSeq_32_XSH_RR_16 {
  pub fn new(seed: u32) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u32) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u32 {
    self.state
  }
}

impl Default for Pcg_OneSeq_32_XSH_RR_16 {
  fn default() -> Self {
    Self::new(<u32>::default())
  }
}

impl NumberGenerator<u16> for Pcg_OneSeq_32_XSH_RR_16 {
  fn get_next(&mut self) -> u16 {
    self.state = pcg_oneseq_32_step_r(self.state);
    pcg_output_xsh_rr_32_16(self.state)
  }
}

impl Iterator for Pcg_OneSeq_32_XSH_RR_16 {
  type Item = u16;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_64_XSH_RR_32 {
  state: u64,
}

impl Pcg_OneSeq_64_XSH_RR_32 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Pcg_OneSeq_64_XSH_RR_32 {
  fn default() -> Self {
    Self::new(<u64>::default())
  }
}

impl NumberGenerator<u32> for Pcg_OneSeq_64_XSH_RR_32 {
  fn get_next(&mut self) -> u32 {
    self.state = pcg_oneseq_64_step_r(self.state);
    pcg_output_xsh_rr_64_32(self.state)
  }
}

impl Iterator for Pcg_OneSeq_64_XSH_RR_32 {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_128_XSH_RR_64 {
  state: u128,
}

impl Pcg_OneSeq_128_XSH_RR_64 {
  pub fn new(seed: u128) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u128 {
    self.state
  }
}

impl Default for Pcg_OneSeq_128_XSH_RR_64 {
  fn default() -> Self {
    Self::new(<u128>::default())
  }
}

impl NumberGenerator<u64> for Pcg_OneSeq_128_XSH_RR_64 {
  fn get_next(&mut self) -> u64 {
    self.state = pcg_oneseq_128_step_r(self.state);
    pcg_output_xsh_rr_128_64(self.state)
  }
}

impl Iterator for Pcg_OneSeq_128_XSH_RR_64 {
  type Item = u64;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ XSH RS

pub struct Pcg_OneSeq_16_XSH_RS_8 {
  state: u16,
}

impl Pcg_OneSeq_16_XSH_RS_8 {
  pub fn new(seed: u16) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u16) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u16 {
    self.state
  }
}

impl Default for Pcg_OneSeq_16_XSH_RS_8 {
  fn default() -> Self {
    Self::new(<u16>::default())
  }
}

impl NumberGenerator<u8> for Pcg_OneSeq_16_XSH_RS_8 {
  fn get_next(&mut self) -> u8 {
    self.state = pcg_oneseq_16_step_r(self.state);
    pcg_output_xsh_rs_16_8(self.state)
  }
}

impl Iterator for Pcg_OneSeq_16_XSH_RS_8 {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_32_XSH_RS_16 {
  state: u32,
}

impl Pcg_OneSeq_32_XSH_RS_16 {
  pub fn new(seed: u32) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u32) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u32 {
    self.state
  }
}

impl Default for Pcg_OneSeq_32_XSH_RS_16 {
  fn default() -> Self {
    Self::new(<u32>::default())
  }
}

impl NumberGenerator<u16> for Pcg_OneSeq_32_XSH_RS_16 {
  fn get_next(&mut self) -> u16 {
    self.state = pcg_oneseq_32_step_r(self.state);
    pcg_output_xsh_rs_32_16(self.state)
  }
}

impl Iterator for Pcg_OneSeq_32_XSH_RS_16 {
  type Item = u16;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_64_XSH_RS_32 {
  state: u64,
}

impl Pcg_OneSeq_64_XSH_RS_32 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Pcg_OneSeq_64_XSH_RS_32 {
  fn default() -> Self {
    Self::new(<u64>::default())
  }
}

impl NumberGenerator<u32> for Pcg_OneSeq_64_XSH_RS_32 {
  fn get_next(&mut self) -> u32 {
    self.state = pcg_oneseq_64_step_r(self.state);
    pcg_output_xsh_rs_64_32(self.state)
  }
}

impl Iterator for Pcg_OneSeq_64_XSH_RS_32 {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_128_XSH_RS_64 {
  state: u128,
}

impl Pcg_OneSeq_128_XSH_RS_64 {
  pub fn new(seed: u128) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u128 {
    self.state
  }
}

impl Default for Pcg_OneSeq_128_XSH_RS_64 {
  fn default() -> Self {
    Self::new(<u128>::default())
  }
}

impl NumberGenerator<u64> for Pcg_OneSeq_128_XSH_RS_64 {
  fn get_next(&mut self) -> u64 {
    self.state = pcg_oneseq_128_step_r(self.state);
    pcg_output_xsh_rs_128_64(self.state)
  }
}

impl Iterator for Pcg_OneSeq_128_XSH_RS_64 {
  type Item = u64;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ RXS M XS Impls

pub struct Pcg_OneSeq_8_RXS_M_XS_8 {
  state: u8,
}

impl Pcg_OneSeq_8_RXS_M_XS_8 {
  pub fn new(seed: u8) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u8) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u8 {
    self.state
  }
}

impl Default for Pcg_OneSeq_8_RXS_M_XS_8 {
  fn default() -> Self {
    Self::new(<u8>::default())
  }
}

impl NumberGenerator<u8> for Pcg_OneSeq_8_RXS_M_XS_8 {
  fn get_next(&mut self) -> u8 {
    self.state = pcg_oneseq_8_step_r(self.state);
    pcg_output_rxs_m_xs_8_8(self.state)
  }
}

impl Iterator for Pcg_OneSeq_8_RXS_M_XS_8 {
  type Item = u8;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_16_RXS_M_XS_16 {
  state: u16,
}

impl Pcg_OneSeq_16_RXS_M_XS_16 {
  pub fn new(seed: u16) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u16) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u16 {
    self.state
  }
}

impl Default for Pcg_OneSeq_16_RXS_M_XS_16 {
  fn default() -> Self {
    Self::new(<u16>::default())
  }
}

impl NumberGenerator<u16> for Pcg_OneSeq_16_RXS_M_XS_16 {
  fn get_next(&mut self) -> u16 {
    self.state = pcg_oneseq_16_step_r(self.state);
    pcg_output_rxs_m_xs_16_16(self.state)
  }
}

impl Iterator for Pcg_OneSeq_16_RXS_M_XS_16 {
  type Item = u16;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_32_RXS_M_XS_32 {
  state: u32,
}

impl Pcg_OneSeq_32_RXS_M_XS_32 {
  pub fn new(seed: u32) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u32) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u32 {
    self.state
  }
}

impl Default for Pcg_OneSeq_32_RXS_M_XS_32 {
  fn default() -> Self {
    Self::new(<u32>::default())
  }
}

impl NumberGenerator<u32> for Pcg_OneSeq_32_RXS_M_XS_32 {
  fn get_next(&mut self) -> u32 {
    self.state = pcg_oneseq_32_step_r(self.state);
    pcg_output_rxs_m_xs_32_32(self.state)
  }
}

impl Iterator for Pcg_OneSeq_32_RXS_M_XS_32 {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_64_RXS_M_XS_64 {
  state: u64,
}

impl Pcg_OneSeq_64_RXS_M_XS_64 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Pcg_OneSeq_64_RXS_M_XS_64 {
  fn default() -> Self {
    Self::new(<u64>::default())
  }
}

impl NumberGenerator<u64> for Pcg_OneSeq_64_RXS_M_XS_64 {
  fn get_next(&mut self) -> u64 {
    self.state = pcg_oneseq_64_step_r(self.state);
    pcg_output_rxs_m_xs_64_64(self.state)
  }
}

impl Iterator for Pcg_OneSeq_64_RXS_M_XS_64 {
  type Item = u64;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_128_RXS_M_XS_128 {
  state: u128,
}

impl Pcg_OneSeq_128_RXS_M_XS_128 {
  pub fn new(seed: u128) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u128 {
    self.state
  }
}

impl Default for Pcg_OneSeq_128_RXS_M_XS_128 {
  fn default() -> Self {
    Self::new(<u128>::default())
  }
}

impl NumberGenerator<u128> for Pcg_OneSeq_128_RXS_M_XS_128 {
  fn get_next(&mut self) -> u128 {
    self.state = pcg_oneseq_128_step_r(self.state);
    pcg_output_rxs_m_xs_128_128(self.state)
  }
}

impl Iterator for Pcg_OneSeq_128_RXS_M_XS_128 {
  type Item = u128;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// ONESEQ XSL RR Impls

pub struct Pcg_OneSeq_64_XSL_RR_32 {
  state: u64,
}

impl Pcg_OneSeq_64_XSL_RR_32 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Pcg_OneSeq_64_XSL_RR_32 {
  fn default() -> Self {
    Self::new(<u64>::default())
  }
}

impl NumberGenerator<u32> for Pcg_OneSeq_64_XSL_RR_32 {
  fn get_next(&mut self) -> u32 {
    self.state = pcg_oneseq_64_step_r(self.state);
    pcg_output_xsl_rr_64_32(self.state)
  }
}

impl Iterator for Pcg_OneSeq_64_XSL_RR_32 {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_OneSeq_128_XSL_RR_64 {
  state: u128,
}

impl Pcg_OneSeq_128_XSL_RR_64 {
  pub fn new(seed: u128) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u128 {
    self.state
  }
}

impl Default for Pcg_OneSeq_128_XSL_RR_64 {
  fn default() -> Self {
    Self::new(<u128>::default())
  }
}

impl NumberGenerator<u64> for Pcg_OneSeq_128_XSL_RR_64 {
  fn get_next(&mut self) -> u64 {
    self.state = pcg_oneseq_128_step_r(self.state);
    pcg_output_xsl_rr_128_64(self.state)
  }
}

impl Iterator for Pcg_OneSeq_128_XSL_RR_64 {
  type Item = u64;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// MCG XSL RR Impls

pub struct Pcg_MCG_64_XSL_RR_32 {
  state: u64,
}

impl Pcg_MCG_64_XSL_RR_32 {
  pub fn new(seed: u64) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u64 {
    self.state
  }
}

impl Default for Pcg_MCG_64_XSL_RR_32 {
  fn default() -> Self {
    Self::new(<u64>::default())
  }
}

impl NumberGenerator<u32> for Pcg_MCG_64_XSL_RR_32 {
  fn get_next(&mut self) -> u32 {
    self.state = pcg_mcg_64_step_r(self.state);
    pcg_output_xsl_rr_64_32(self.state)
  }
}

impl Iterator for Pcg_MCG_64_XSL_RR_32 {
  type Item = u32;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub struct Pcg_MCG_128_XSL_RR_64 {
  state: u128,
}

impl Pcg_MCG_128_XSL_RR_64 {
  pub fn new(seed: u128) -> Self {
    Self { state: seed }
  }
  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.state = new_seed;
    self
  }
  pub fn get_seed(&self) -> u128 {
    self.state
  }
}

impl Default for Pcg_MCG_128_XSL_RR_64 {
  fn default() -> Self {
    Self::new(<u128>::default())
  }
}

impl NumberGenerator<u64> for Pcg_MCG_128_XSL_RR_64 {
  fn get_next(&mut self) -> u64 {
    self.state = pcg_mcg_128_step_r(self.state);
    pcg_output_xsl_rr_128_64(self.state)
  }
}

impl Iterator for Pcg_MCG_128_XSL_RR_64 {
  type Item = u64;
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

// Type Aliases

#[cfg(target_pointer_width = "64")]
pub struct Pcg_Usize {
  inner: Pcg_64,
}

#[cfg(target_pointer_width = "64")]
impl Pcg_Usize {
  pub fn new(seed: u128) -> Self {
    {
      Self {
        inner: Pcg_64::new(seed),
      }
    }
  }

  pub fn reset_seed(&mut self, new_seed: u128) -> &mut Self {
    self.inner.state = new_seed;
    self
  }

  pub fn get_seed(&self) -> u128 {
    self.inner.state
  }
}

#[cfg(target_pointer_width = "32")]
pub struct Pcg_Usize {
  inner: Pcg_32,
}

#[cfg(target_pointer_width = "32")]
impl Pcg_Usize {
  pub fn new(seed: u64) -> Self {
    {
      Self {
        inner: Pcg_32::new(seed),
      }
    }
  }

  pub fn reset_seed(&mut self, new_seed: u64) -> &mut Self {
    self.inner.state = new_seed;
    self
  }

  pub fn get_seed(&self) -> u64 {
    self.inner.state
  }
}

impl Default for Pcg_Usize {
  fn default() -> Self {
    Self {
      inner: Default::default(),
    }
  }
}
impl NumberGenerator<usize> for Pcg_Usize {
  fn get_next(&mut self) -> usize {
    self.inner.get_next() as usize
  }
}

impl Iterator for Pcg_Usize {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    Some(self.get_next())
  }
}

pub type Pcg_8 = Pcg_OneSeq_16_XSH_RR_8;
pub type Pcg_16 = Pcg_OneSeq_32_XSH_RR_16;
pub type Pcg_32 = Pcg_OneSeq_64_XSH_RR_32;
pub type Pcg_64 = Pcg_OneSeq_128_XSH_RR_64;
pub type Pcg_128 = Pcg_OneSeq_128_RXS_M_XS_128;

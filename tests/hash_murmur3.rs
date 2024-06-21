use algorithms::hash::murmur3::{Murmur3Hash128, Murmur3Hash32};
use algorithms::hash::HashFunc;

mod common;

const TEST_CASE_0: &'static [u8] = b"";
const TEST_CASE_1: &'static [u8] = b"test";
const TEST_CASE_2: &'static [u8] = b"Hello, world!";
const TEST_CASE_3: &'static [u8] = b"The quick brown fox jumps over the lazy dog";
// Copypasta for testing very long text. Reference: https://htmx.org/essays/rest-copypasta/
const TEST_CASE_4: &'static [u8] = common::HTMX_COPY_PASTA;
const TEST_CASE_5: &'static [u8] = b"0123456";
const TEST_CASE_6: &'static [u8] = b"01234567";
const TEST_CASE_7: &'static [u8] = b"012345678";
const TEST_CASE_8: &'static [u8] = b"0123456789AB";
const TEST_CASE_9: &'static [u8] = b"0123456789ABCDE";

#[test]
fn murmur3_32_verify() {
  let hasher = Murmur3Hash32::default();

  assert_eq!(0x00000000, hasher.get_hash(TEST_CASE_0));
  assert_eq!(0xBA6BD213, hasher.get_hash(TEST_CASE_1));
  assert_eq!(0xC0363E43, hasher.get_hash(TEST_CASE_2));
  assert_eq!(0x2E4FF723, hasher.get_hash(TEST_CASE_3));
  assert_eq!(0x8C16F361, hasher.get_hash(TEST_CASE_4));
  assert_eq!(0xAEF31A8, hasher.get_hash(TEST_CASE_5));
  assert_eq!(0x56831753, hasher.get_hash(TEST_CASE_6));
  assert_eq!(0x5081DA7D, hasher.get_hash(TEST_CASE_7));
  assert_eq!(0xEA51B5DC, hasher.get_hash(TEST_CASE_8));
  assert_eq!(0xD5C4EAA1, hasher.get_hash(TEST_CASE_9));
}

#[test]
fn murmur3_32_with_seed_verify() {
  let seed: u32 = 0x9747B28C;
  let hasher = Murmur3Hash32::new_with_seed(seed);

  assert_eq!(0xEBB6C228, hasher.get_hash(TEST_CASE_0));
  assert_eq!(0x704B81DC, hasher.get_hash(TEST_CASE_1));
  assert_eq!(0x24884CBA, hasher.get_hash(TEST_CASE_2));
  assert_eq!(0x2FA826CD, hasher.get_hash(TEST_CASE_3));
  assert_eq!(0x9AE5D7FA, hasher.get_hash(TEST_CASE_4));
  assert_eq!(0xAB2513B7, hasher.get_hash(TEST_CASE_5));
  assert_eq!(0x7BC64FBE, hasher.get_hash(TEST_CASE_6));
  assert_eq!(0xE0CFE8FB, hasher.get_hash(TEST_CASE_7));
  assert_eq!(0x29982342, hasher.get_hash(TEST_CASE_8));
  assert_eq!(0xE505999C, hasher.get_hash(TEST_CASE_9));
}

#[test]
fn murmur3_128_verify() {
  let hasher = Murmur3Hash128::default();

  assert_eq!(0x00000000, hasher.get_hash(TEST_CASE_0));
  assert_eq!(
    0x9A128231F9BD4D82_AC7D28CC74BDE19D,
    hasher.get_hash(TEST_CASE_1)
  );
  assert_eq!(
    0x2C326650A8F3C564_F1512DD1D2D665DF,
    hasher.get_hash(TEST_CASE_2)
  );
  assert_eq!(
    0x7A433CA9C49A9347_E34BBC7BBC071B6C,
    hasher.get_hash(TEST_CASE_3)
  );
  assert_eq!(
    0x821D0936B48B4438_1F4D7E788D8307ED,
    hasher.get_hash(TEST_CASE_4)
  );
  assert_eq!(
    0xB4EBEF492FDEF34E_13EB9FB82606F7A6,
    hasher.get_hash(TEST_CASE_5)
  );
  assert_eq!(
    0xC3369387D8964920_8236039B7387354D,
    hasher.get_hash(TEST_CASE_6)
  );
  assert_eq!(
    0x72A17AF899D597F1_4C1E87519FE738BA,
    hasher.get_hash(TEST_CASE_7)
  );
  assert_eq!(
    0x9A6096214190A0BD_46967871F3B4400A,
    hasher.get_hash(TEST_CASE_8)
  );
  assert_eq!(
    0x270FC32BD6A3F5DE_D0161EEDC28F1027,
    hasher.get_hash(TEST_CASE_9)
  );
}

#[test]
fn murmur3_128_with_seed_verify() {
  let seed: u64 = 0x9747B28C;
  let hasher = Murmur3Hash128::new_with_seed(seed);

  assert_eq!(
    0x93B0608FE302957A_392B208A1DAABBB3,
    hasher.get_hash(TEST_CASE_0)
  );
  assert_eq!(
    0x64A6E65666D07937_A066A6B76C553018,
    hasher.get_hash(TEST_CASE_1)
  );
  assert_eq!(
    0xF85E7E7631D576BA_EDC485D662A8392E,
    hasher.get_hash(TEST_CASE_2)
  );
  assert_eq!(
    0xF94573727EC016E5_738A7F3BD2633121,
    hasher.get_hash(TEST_CASE_3)
  );
  assert_eq!(
    0x60A64FCEA1001606_E94B201FDADB0A73,
    hasher.get_hash(TEST_CASE_4)
  );
  assert_eq!(
    0x8B6A978A6926EAD7_D4888F3A46F792E3,
    hasher.get_hash(TEST_CASE_5)
  );
  assert_eq!(
    0x48A9D6B322A1C07A_295373671326D416,
    hasher.get_hash(TEST_CASE_6)
  );
  assert_eq!(
    0xC37755B336F3111B_D1117CC8DEEFFB95,
    hasher.get_hash(TEST_CASE_7)
  );
  assert_eq!(
    0xE5AECDBFD78688E6_ABA229BC5FEDE8E3,
    hasher.get_hash(TEST_CASE_8)
  );
  assert_eq!(
    0x373BF069175817C5_4204D1605934BBF1,
    hasher.get_hash(TEST_CASE_9)
  );
}

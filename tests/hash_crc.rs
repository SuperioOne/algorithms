use algorithms::hash::HashFunc;
mod common;

const TEST_CASE_0: &[u8] = b"";
const TEST_CASE_1: &[u8] = b"0";
const TEST_CASE_2: &[u8] = b"01";
const TEST_CASE_3: &[u8] = b"012";
const TEST_CASE_4: &[u8] = b"0123";
const TEST_CASE_5: &[u8] = b"01234";
const TEST_CASE_6: &[u8] = b"01234567"; // 8-byte
const TEST_CASE_7: &[u8] = b"0123456789012345"; // 16-byte
const TEST_CASE_8: &[u8] = b"0123456789012345678901234567890123456789012345678901"; // 52-byte

#[test]
fn crc32c_verify() {
  let hasher = algorithms::hash::crc::Crc32C::default();

  assert_eq!(0x00000000, hasher.get_hash(TEST_CASE_0));
  assert_eq!(0x629E1AE0, hasher.get_hash(TEST_CASE_1));
  assert_eq!(0x73A7AFE3, hasher.get_hash(TEST_CASE_2));
  assert_eq!(0x73B69656, hasher.get_hash(TEST_CASE_3));
  assert_eq!(0x063962B9, hasher.get_hash(TEST_CASE_4));
  assert_eq!(0x6FA51D98, hasher.get_hash(TEST_CASE_5));
  assert_eq!(0xAC222320, hasher.get_hash(TEST_CASE_6));
  assert_eq!(0x5E1EF156, hasher.get_hash(TEST_CASE_7));
  assert_eq!(0x6BDEDC9A, hasher.get_hash(TEST_CASE_8));
}

#[test]
fn crc32c_with_initial() {
  let hasher = algorithms::hash::crc::Crc32C::new_with_initial(0x6FA51D98);
  assert_eq!(0xAC222320, hasher.get_hash(b"567"));
  assert_eq!(
    0x6BDEDC9A,
    hasher.get_hash(b"56789012345678901234567890123456789012345678901")
  );
}

mod cityhash {
  use algorithms::hash::{cityhash::CityHash_32, HashFunc};

  #[test]
  fn cityhash_32_verify() {
    let hasher = CityHash_32::default();

    assert_eq!(0x61571065, hasher.get_hash(b"test"));
    assert_eq!(0x6FE1B5B5, hasher.get_hash(b"Hello, world!"));
    assert_eq!(
      0xA339C810,
      hasher.get_hash(b"The quick brown fox jumps over the lazy dog")
    );
  }

  // Tests edge cases
  #[test]
  fn cityhash_32_from_0_to_4() {
    let hasher = CityHash_32::default();

    assert_eq!(0xDC56D17A, hasher.get_hash(b""));
    assert_eq!(0xD63C2322, hasher.get_hash(b"0"));
    assert_eq!(0xE5B54743, hasher.get_hash(b"01"));
    assert_eq!(0xC243F2A6, hasher.get_hash(b"012"));
    assert_eq!(0xA7A6F80B, hasher.get_hash(b"0123"));
  }

  #[test]
  fn cityhash_32_from_5_to_12() {
    let hasher = CityHash_32::default();

    assert_eq!(0xEC60F6A0, hasher.get_hash(b"01234"));
    assert_eq!(0x99D93E22, hasher.get_hash(b"01234567"));
    assert_eq!(0x62D2EDE3, hasher.get_hash(b"0123456789AB"));
  }

  #[test]
  fn cityhash_32_from_13_to_24() {
    let hasher = CityHash_32::default();

    assert_eq!(0xDD51E526, hasher.get_hash(b"0123456789ABC"));
    assert_eq!(0x82697553, hasher.get_hash(b"0123456789ABCDEFGH"));
    assert_eq!(0xDB3F1CF1, hasher.get_hash(b"0123456789ABCDEFGHIJKLMN"));
  }
}

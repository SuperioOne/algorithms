mod cityhash {
  use algorithms::hash::cityhash::CityHash128;
  use algorithms::hash::{cityhash::CityHash32, cityhash::CityHash64, HashFunc};

  const TEST_CASE_0: &'static [u8] = b"";
  const TEST_CASE_1: &'static [u8] = b"test";
  const TEST_CASE_2: &'static [u8] = b"Hello, world!";
  const TEST_CASE_3: &'static [u8] = b"The quick brown fox jumps over the lazy dog";
  // Copypasta for testing very long text. Reference: https://htmx.org/essays/rest-copypasta/
  const TEST_CASE_4: &'static [u8] = "I’d just like to interject for a moment. What you’re referring to as REST, is in fact, JSON/RPC, or as I’ve recently taken to calling it, REST-less. JSON is not a hypermedia unto itself, but rather a plain data format made useful by out of band information as defined by swagger documentation or similar.Many computer users work with a canonical version of REST every day, without realizing it. Through a peculiar turn of events, the version of REST which is widely used today is often called “The Web”, and many of its users are not aware that it is basically the REST-ful architecture, defined by Roy Fielding. There really is a REST, and these people are using it, but it is just a part of The Web they use. REST is the network architecture: hypermedia encodes the state of resources for hypermedia clients. JSON is an essential part of Single Page Applications, but useless by itself; it can only function in the context of a complete API specification. JSON is normally used in combination with SPA libraries: the whole system is basically RPC with JSON added, or JSON/RPC. All these so-called “REST-ful” APIs are really JSON/RPC.".as_bytes();

  #[test]
  fn cityhash_32_verify() {
    let hasher = CityHash32::default();

    assert_eq!(0x61571065, hasher.get_hash(TEST_CASE_1));
    assert_eq!(0x6FE1B5B5, hasher.get_hash(TEST_CASE_2));
    assert_eq!(0xA339C810, hasher.get_hash(TEST_CASE_3));
  }

  // Tests edge cases
  #[test]
  fn cityhash_32_from_0_to_4() {
    let hasher = CityHash32::default();

    assert_eq!(0xDC56D17A, hasher.get_hash(TEST_CASE_0));
    assert_eq!(0xD63C2322, hasher.get_hash(b"0"));
    assert_eq!(0xE5B54743, hasher.get_hash(b"01"));
    assert_eq!(0xC243F2A6, hasher.get_hash(b"012"));
    assert_eq!(0xA7A6F80B, hasher.get_hash(b"0123"));
  }

  #[test]
  fn cityhash_32_from_5_to_12() {
    let hasher = CityHash32::default();

    assert_eq!(0xEC60F6A0, hasher.get_hash(b"01234"));
    assert_eq!(0x99D93E22, hasher.get_hash(b"01234567"));
    assert_eq!(0x62D2EDE3, hasher.get_hash(b"0123456789AB"));
  }

  #[test]
  fn cityhash_32_from_13_to_24() {
    let hasher = CityHash32::default();

    assert_eq!(0xDD51E526, hasher.get_hash(b"0123456789ABC"));
    assert_eq!(0x82697553, hasher.get_hash(b"0123456789ABCDEFGH"));
    assert_eq!(0xDB3F1CF1, hasher.get_hash(b"0123456789ABCDEFGHIJKLMN"));
  }

  #[test]
  fn cityhash_64_verify() {
    let hasher = CityHash64::default();

    assert_eq!(0x7717383DAA85B5B2, hasher.get_hash(TEST_CASE_1));
    assert_eq!(0x307C26B3E0789A47, hasher.get_hash(TEST_CASE_2));
    assert_eq!(0xC268724928FECA7D, hasher.get_hash(TEST_CASE_3));
  }

  #[test]
  fn cityhash_64_from_0_to_16() {
    let hasher = CityHash64::default();

    assert_eq!(0x9AE16A3B2F90404F, hasher.get_hash(TEST_CASE_0));
    assert_eq!(0xD2ED96073B81823F, hasher.get_hash(b"0"));
    assert_eq!(0x73D7FA164971735C, hasher.get_hash(b"012"));
    assert_eq!(0x839ACC613DFB3687, hasher.get_hash(b"0123"));
    assert_eq!(0x1D096F87B7B28685, hasher.get_hash(b"0123456"));
    assert_eq!(0x6DFF3AF4CE43E8F6, hasher.get_hash(b"01234567"));
    assert_eq!(0x5C6A6DB7CFC1DF39, hasher.get_hash(b"0123456789AB"));
    assert_eq!(0x6DA7F6E5AD5A11D4, hasher.get_hash(b"0123456789ABCDEF"));
  }

  #[test]
  fn cityhash_64_from_17_to_32() {
    let hasher = CityHash64::default();

    assert_eq!(0x519EF8279FA56DA8, hasher.get_hash(b"0123456789ABCDEFG"));
    assert_eq!(
      0x79E3DE290DD64958,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLM")
    );
    assert_eq!(
      0x46B788F01C3BEF9D,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUV")
    );
  }

  #[test]
  fn cityhash_64_from_33_to_64() {
    let hasher = CityHash64::default();

    assert_eq!(
      0x3122B526C080B298,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVW")
    );
    assert_eq!(
      0x68AEEA84C323BFEE,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    );
    assert_eq!(
      0x964ECAE01F972966,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQR")
    );
  }

  #[test]
  fn cityhash_64_beyond_65() {
    let hasher = CityHash64::default();

    assert_eq!(
      0xB1CDABB8345B282F,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    );

    assert_eq!(0xD800360A5EE32429, hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
  }

  #[test]
  fn cityhash_64_with_seed() {
    let seed_u64: u64 = 65;
    let seed_u128: u128 = (256_u128) + (43_u128 << 64);
    let input = TEST_CASE_3;

    let hasher_0 = CityHash64::new_with_seed(seed_u64.into());
    let hasher_1 = CityHash64::new_with_seed(seed_u128.into());

    assert_eq!(0xE3951FDAB7579E23, hasher_0.get_hash(input));
    assert_eq!(0x9DB7790AA52E2889, hasher_1.get_hash(input));
  }

  #[test]
  fn cityhash_128_verify() {
    let hasher = CityHash128::default();

    assert_eq!(
      0xFBEFF23C90EADF081139CE35096D0BA4,
      hasher.get_hash(TEST_CASE_1)
    );
    assert_eq!(
      0x1D605B22B9DB6F5FEF39B1309BE65105,
      hasher.get_hash(TEST_CASE_2)
    );
    assert_eq!(
      0xBF1498F876DBE279A7F9A86A2D60C968,
      hasher.get_hash(TEST_CASE_3)
    );
  }

  #[test]
  fn cityhash_128_less_than_128() {
    let hasher = CityHash128::default();

    assert_eq!(
      0x3CB540C392E51E293DF09DFC64C09A2B,
      hasher.get_hash(TEST_CASE_0)
    );
    assert_eq!(0x5A3D7A45A4BC087C8523CF187B8ABBDD, hasher.get_hash(b"01"));
    assert_eq!(0x6754117E10DA46A1E3C2D49F7CD89869, hasher.get_hash(b"0123"));
    assert_eq!(
      0xE6EA5C08212D1998D942F238E317D6A8,
      hasher.get_hash(b"01234567")
    );
    assert_eq!(
      0xB146F97B33269DB4981DBA60A4C77A9B,
      hasher.get_hash(b"0123456789AB")
    );
    assert_eq!(
      0x98314E6AF6A90057EA3118A9D52C21E9,
      hasher.get_hash(b"0123456789ABCDEF")
    );
    assert_eq!(
      0x69A2100901C539D67C885C5F7B6FB54,
      hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQR")
    );
    assert_eq!(0x3058F429847758080D13719241C3BA84, hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHI"));
  }

  #[test]
  fn cityhash_128_greater_than_127() {
    let hasher = CityHash128::default();

    assert_eq!(0x2D0B83D3524FC5C2C349A6AB84C2BF28, hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJ"));
    assert_eq!(0x88F058253CBF2DB8619EC93C451ECE96, hasher.get_hash(b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
  }

  #[test]
  fn cityhash_128_with_seed() {
    let hasher = CityHash128::new_with_seed((256, 7024).into());

    assert_eq!(
      0x387C39BA03C11ECFA5498E70909AFC45,
      hasher.get_hash(TEST_CASE_0)
    );
    assert_eq!(
      0xE3A53983F6913207F31E1CBD73E5D46A,
      hasher.get_hash(TEST_CASE_1)
    );
    assert_eq!(
      0x7B0FDDA39E0FE126A53C6CD49B87CC87,
      hasher.get_hash(TEST_CASE_2)
    );
    assert_eq!(
      0xA39337FB84A414D1A42F9E80EFC249A6,
      hasher.get_hash(TEST_CASE_3)
    );
  }

  #[test]
  fn cityhash_crc256_verify() {
    use algorithms::hash::cityhash::CityHashCrc256;

    let hasher = CityHashCrc256::new();

    assert_eq!(
      (
        10742825796673861936,
        7496450439486572256,
        12948398300027047298,
        14530353005289330699
      ),
      hasher.get_hash(TEST_CASE_0).into()
    );
    assert_eq!(
      (
        11964743055457135893,
        9728737892224883946,
        17411264226841264756,
        14975265955330065747
      ),
      hasher.get_hash(TEST_CASE_1).into()
    );
    assert_eq!(
      (
        17058006416023241677,
        1145080796913737482,
        1247677997574841463,
        8187887664672644589
      ),
      hasher.get_hash(TEST_CASE_2).into()
    );
    assert_eq!(
      (
        1764417952579595218,
        3983194879807211044,
        15813673336000732099,
        6922380923046817502
      ),
      hasher.get_hash(TEST_CASE_3).into()
    );

    assert_eq!(
      (
        15683388910506547534,
        13409921696239111691,
        8029825801520537352,
        8077810071791116379
      ),
      hasher.get_hash(TEST_CASE_4).into()
    );
  }

  #[test]
  fn cityhash_crc128_verify() {
    use algorithms::hash::cityhash::CityHashCrc128;

    let hasher = CityHashCrc128::new();
    assert_eq!(
      0x3CB540C392E51E293DF09DFC64C09A2B,
      hasher.get_hash(TEST_CASE_0)
    );
    assert_eq!(
      0xFBEFF23C90EADF081139CE35096D0BA4,
      hasher.get_hash(TEST_CASE_1)
    );
    assert_eq!(
      0x1D605B22B9DB6F5FEF39B1309BE65105,
      hasher.get_hash(TEST_CASE_2)
    );
    assert_eq!(
      0xBF1498F876DBE279A7F9A86A2D60C968,
      hasher.get_hash(TEST_CASE_3)
    );
    assert_eq!(
      0x701A2576542EA05B6F6FAC05CBE5DF08,
      hasher.get_hash(TEST_CASE_4)
    );
  }

  #[test]
  fn cityhash_crc128_with_seed() {
    use algorithms::hash::cityhash::CityHashCrc128;

    let hasher = CityHashCrc128::new_with_seed((256, 7024).into());

    assert_eq!(
      0x387C39BA03C11ECFA5498E70909AFC45,
      hasher.get_hash(TEST_CASE_0)
    );
    assert_eq!(
      0xE3A53983F6913207F31E1CBD73E5D46A,
      hasher.get_hash(TEST_CASE_1)
    );
    assert_eq!(
      0x7B0FDDA39E0FE126A53C6CD49B87CC87,
      hasher.get_hash(TEST_CASE_2)
    );
    assert_eq!(
      0xA39337FB84A414D1A42F9E80EFC249A6,
      hasher.get_hash(TEST_CASE_3)
    );
    assert_eq!(
      0x3A152F9B2978C1960B3E2C5531CA9341,
      hasher.get_hash(TEST_CASE_4)
    );
  }
}

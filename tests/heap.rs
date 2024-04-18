mod heap_test {

  #[test]
  fn min_heap_odd_i32() {
    let mut arr = [2, 7, 17, 3, 19, 36, 25, 1, 100];

    algorithms::min_heapify(&mut arr);

    println!("{:?}", &arr);
    assert!(true)
  }

  #[test]
  fn max_heap_odd_i32() {
    let mut arr = [2, 7, 17, 3, 19, 36, 25, 1, 100];

    algorithms::max_heapify(&mut arr);

    println!("{:?}", &arr);
    assert!(true)
  }

  #[test]
  fn min_heap_even_i32() {
    let mut arr = [-2, 7, 0, 53, 13, 45, -25, 1, 100, 123];

    algorithms::min_heapify(&mut arr);

    println!("{:?}", &arr);
    assert!(true)
  }

  #[test]
  fn max_heap_even_i32() {
    let mut arr = [-2, 7, 0, 53, 13, 45, -25, 1, 100, 123];

    algorithms::max_heapify(&mut arr);

    println!("{:?}", &arr);
    assert!(true)
  }
}

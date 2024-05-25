use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("crc32c 16 byte", |b| {
    b.iter(|| algorithms::hash::crc::hash_fn::crc32c(black_box(b"1234567890123456")))
  });

  c.bench_function("crc32c 8 byte", |b| {
    b.iter(|| algorithms::hash::crc::hash_fn::crc32c(black_box(b"12345678")))
  });

  c.bench_function("crc32c 12 byte", |b| {
    b.iter(|| algorithms::hash::crc::hash_fn::crc32c(black_box(b"123456789012")))
  });

  c.bench_function("crc32c 38 byte", |b| {
    b.iter(|| {
      algorithms::hash::crc::hash_fn::crc32c(black_box(b"12345678901234567890123456789012345678"))
    })
  });

  c.bench_function("crc32c extreme", |b| {
      b.iter(|| algorithms::hash::crc::hash_fn::crc32c(black_box("I’d just like to interject for a moment. What you’re referring to as REST, is in fact, JSON/RPC, or as I’ve recently taken to calling it, REST-less. JSON is not a hypermedia unto itself, but rather a plain data format made useful by out of band information as defined by swagger documentation or similar.Many computer users work with a canonical version of REST every day, without realizing it. Through a peculiar turn of events, the version of REST which is widely used today is often called “The Web”, and many of its users are not aware that it is basically the REST-ful architecture, defined by Roy Fielding. There really is a REST, and these people are using it, but it is just a part of The Web they use. REST is the network architecture: hypermedia encodes the state of resources for hypermedia clients. JSON is an essential part of Single Page Applications, but useless by itself; it can only function in the context of a complete API specification. JSON is normally used in combination with SPA libraries: the whole system is basically RPC with JSON added, or JSON/RPC. All these so-called “REST-ful” APIs are really JSON/RPC.".as_bytes()
  )))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

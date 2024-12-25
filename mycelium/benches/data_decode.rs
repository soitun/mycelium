use bytes::BytesMut;
use criterion::{criterion_group, criterion_main, Criterion};
use mycelium::packet::data::{decode, decode_zero, DataPacket};
use std::net::Ipv6Addr;

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_packet_decode");

    // Setup test data
    let mut data = BytesMut::new();
    data.extend_from_slice(&[0x00, 0x20, 0x00, 0x04]); // header
    data.extend_from_slice(&[1u8; 16]); // src_ip
    data.extend_from_slice(&[2u8; 16]); // dst_ip
    data.extend_from_slice(&[42u8; 1000]); // payload

    group.bench_function("decode", |b| {
        b.iter(|| {
            let mut test_data = data.clone();
            decode(&mut test_data).unwrap()
        });
    });

    group.bench_function("decode_zero", |b| {
        b.iter(|| {
            let mut test_data = data.clone();
            decode_zero(&mut test_data).unwrap()
        });
    });

    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);

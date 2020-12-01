use huffman_comprs::{Huffman, RZFile};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let tree = Huffman::from(input.as_str());

    let data = tree.compress(&input).unwrap();

    let file = RZFile::new(tree, data);
    file.save_to_file("output.rz").unwrap();

    let bytes = std::fs::read("output.rz").unwrap();

    c.bench_function("Reading from file", |b| {
        b.iter(|| {
            RZFile::from(black_box(bytes.as_slice()));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

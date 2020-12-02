use huffman_comprs::Huffman;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn from_file(c: &mut Criterion) {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let tree = Huffman::from(input.as_str());

    let data = tree.compress(&input).unwrap();

    c.bench_function("Reconstruction", |b| {
        b.iter(|| tree.reconstruct(black_box(data.clone()), black_box(0)))
    });
}

fn generation(c: &mut Criterion) {
    let input = std::fs::read_to_string("input.txt").unwrap();

    c.bench_function("Generation", |b| {
        b.iter(|| Huffman::from(black_box(input.as_str())))
    });
}

criterion_group!(benches, from_file, generation);
criterion_main!(benches);

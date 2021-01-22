use huffman_comprs::{Huffman, RZFile};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn from_file(c: &mut Criterion) {
    let input = "Ab".to_string();

    let tree = Huffman::from(input.as_str());

    let data = tree.compress_str(&input).unwrap();

    let rz_file = RZFile::new(tree, data.clone());

    c.bench_function("Reconstruction", |b| {
        b.iter(|| {
            rz_file
                .tree
                .reconstruct(black_box(data.clone()), black_box(0))
                .unwrap()
        })
    });
}

fn generation(c: &mut Criterion) {
    let input = "This is a short input, to demonstrate how fast this can be!".to_string();

    c.bench_function("Generation", |b| {
        b.iter(|| Huffman::from(black_box(input.as_str())))
    });
}

criterion_group!(benches, from_file, generation);
criterion_main!(benches);

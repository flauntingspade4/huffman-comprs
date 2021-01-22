use std::convert::TryFrom;

use huffman_comprs::{Huffman, RZFile};

#[test]
fn from_nonsense() {
    let input = "abcdefg";
    let path = "example.rz";

    {
        let tree = Huffman::from(input);

        let data = tree.compress_str(input).unwrap();

        let file = RZFile::new(tree, data);

        let mut data = Vec::new();

        for a in file.data() {
            data.append(&mut huffman_comprs::u8_to_bits(*a));
        }

        file.save_to_file(path).unwrap();
    }

    let buf = std::fs::read(path).unwrap();

    let file = RZFile::try_from(buf.as_slice()).unwrap();

    let mut data = Vec::with_capacity(buf.len() * 8);

    for a in file.data() {
        data.append(&mut huffman_comprs::u8_to_bits(*a));
    }

    let new_input: Vec<char> = file.tree.reconstruct(data, file.zeros()).unwrap();
    let new_input: String = new_input.into_iter().collect();

    std::fs::remove_file(path).unwrap();

    assert_eq!(new_input.as_str(), input);
}

use huffman_comprs::{Huffman, RZFile};

#[test]
fn from_nonsense() {
    let input = "abcde";
    let path = "example.rz";

    {
        let tree = Huffman::from(input);

        let data = tree.compress(input).unwrap();

        let file = RZFile::new(tree, data);

        file.save_to_file(path).unwrap();
    }

    let buf = std::fs::read(path).unwrap();

    let file = RZFile::from(buf.as_slice());

    let mut data = Vec::with_capacity(buf.len() * 8);

    for a in file.data() {
        data.append(&mut Huffman::u8_to_bits(*a));
    }

    let new_input = file.tree.reconstruct(data, file.zeros).unwrap();

    assert_eq!(new_input.as_str(), input);

    std::fs::remove_file(path).unwrap();
}

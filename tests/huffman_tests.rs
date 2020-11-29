use huffman_comprs::*;

#[test]
fn from_hello_world() {
    let script = "aabc";

    let huffman = Huffman::from(script);

    assert_eq!(vec![true], huffman.get_code('a').unwrap());
    assert_eq!(vec![false, true], huffman.get_code('b').unwrap());
    assert_eq!(vec![false, false], huffman.get_code('c').unwrap());
}

#[cfg(feature = "serde_support")]
#[test]
fn serde_testing() {
    let script = "This is a testing string, and should make a somewhat interesting Huffman Tree.";

    let huffman = Huffman::from(script);

    let bv = bincode::serialize(&huffman).unwrap();

    let huffman_de: Huffman = bincode::deserialize(&bv).unwrap();

    assert_eq!(huffman, huffman_de);
}

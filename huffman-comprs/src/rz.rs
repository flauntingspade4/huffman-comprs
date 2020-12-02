use std::{convert::TryInto, path::Path};

use crate::Huffman;

/// A RZ file, with a tree, and data.
///
/// RZ files are constructed as such:
///
/// | name      | size             | usage                                                                                |
/// |-----------|------------------|--------------------------------------------------------------------------------------|
/// | tree_len  | 4 bytes          | Indicates how many bytes the tree takes up                                           |
/// | zeros     | 1 byte           | Indicates how many 0s are appended onto the end of the data, so to make full bytes   |
/// | tree      | tree_len bytes   | The actual Huffman tree                                                              |
/// | data      | rest of the file | The data, compressed with the above huffman tree                                     |
///
/// [`RZFile`](struct.RZFile.html)s can be generated from `&[u8]`
#[derive(Clone, Debug)]
pub struct RZFile {
    tree_len: u32,
    pub zeros: u8,
    pub tree: Huffman,
    data: Vec<u8>,
}

impl RZFile {
    /// Generates a RZ file from a given [`Huffman`](struct.Huffman.html) tree, and `Vec<bool>`, being the compressed data
    pub fn new(tree: Huffman, mut data: Vec<bool>) -> Self {
        let tree_len = bincode::serialize(&tree).unwrap().len() as u32;

        let mut data_new = Vec::with_capacity(data.len() / 8);

        let len = data.len();

        while !data.is_empty() {
            let mut to_add = 0;
            for i in 0..8 {
                to_add += (data.pop().unwrap_or(false) as u8) * 2u16.pow(i) as u8;
            }
            data_new.push(to_add);
        }

        let zeros = if len % 8 != 0 {
            let zeros = 8 - (len % 8);
            data.append(&mut vec![false; zeros]);
            zeros as u8
        } else {
            0
        };

        Self {
            tree_len,
            zeros,
            tree,
            data: data_new,
        }
    }
    /// Returns a reference to the RZFile's data, which is compressed
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
    /// Saves the compressed version of self to the file at `path`
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut contents = Vec::with_capacity(5 + self.tree_len as usize + self.data.len());
        contents.append(&mut self.tree_len.to_be_bytes().to_vec());
        contents.push(self.zeros);
        contents.append(&mut bincode::serialize(&self.tree).unwrap());
        contents.append(&mut self.data.clone());
        std::fs::write(path, contents)
    }
}

impl From<&[u8]> for RZFile {
    fn from(buf: &[u8]) -> Self {
        let tree_len = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        let zeros = buf[4];
        let buf = buf.split_at(5).1;
        let (tree, data) = buf.split_at(tree_len as usize);

        Self {
            tree_len,
            zeros,
            tree: bincode::deserialize(tree).unwrap(),
            data: data.to_vec(),
        }
    }
}

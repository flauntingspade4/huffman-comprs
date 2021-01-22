use std::{
    convert::{TryFrom, TryInto},
    path::Path,
};

use serde::{Deserialize, Serialize};

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
pub struct RZFile<T>
where
    T: Serialize + Ord + Clone,
{
    tree_len: u32,
    zeros: u8,
    pub tree: Huffman<T>,
    data: Vec<u8>,
}

impl<T> RZFile<T>
where
    T: Serialize + Ord + Clone,
{
    /// Generates a RZ file from a given [`Huffman`](struct.Huffman.html) tree, and `Vec<bool>`, being the compressed data
    #[must_use]
    pub fn new(tree: Huffman<T>, mut data: Vec<bool>) -> Self {
        let tree_len = bincode::serialize(&tree).unwrap().len() as u32;

        let mut data_new = Vec::with_capacity(data.len() / 8);

        let zeros = if data.len() % 8 != 0 {
            let zeros = 8 - (data.len() % 8);
            zeros as u8
        } else {
            0
        };

        while !data.is_empty() {
            let mut to_add = 0;
            for i in 0..8 {
                to_add += (data.pop().unwrap_or(false) as u8) * 2_u16.pow(i) as u8;
            }
            data_new.push(to_add);
        }

        Self {
            tree_len,
            zeros,
            tree,
            data: data_new,
        }
    }
    /// Returns a reference to the `RZFile`'s data, which is compressed
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    /// Returns how many zeros should be appended in the file
    #[must_use]
    pub fn zeros(&self) -> u8 {
        self.zeros
    }
    /// Saves the compressed version of self to the file at `path`
    ///
    /// # Errors
    /// Fails if there's any issue with writing to the file at path
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut contents = Vec::with_capacity(5 + self.tree_len as usize + self.data.len());
        contents.append(&mut self.tree_len.to_be_bytes().to_vec());
        contents.push(self.zeros);
        contents.append(&mut bincode::serialize(&self.tree).unwrap());
        contents.append(&mut self.data.to_vec());
        std::fs::write(path, contents)
    }
}

impl<'a: 'de, 'de, T> TryFrom<&'a [u8]> for RZFile<T>
where
    T: Serialize + Ord + Deserialize<'de> + Clone,
{
    type Error = bincode::Error;

    fn try_from(mut buf: &'de [u8]) -> Result<Self, Self::Error> {
        let tree_len = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        let zeros = buf[4];
        buf = &buf[5..];

        let (tree, buf) = buf.split_at(tree_len as usize);

        let tree = bincode::deserialize(tree)?;

        Ok(Self {
            tree_len,
            zeros,
            tree,
            data: buf.to_vec(),
        })
    }
}

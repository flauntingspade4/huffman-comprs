//! A small crate, defining a [Huffman tree](https://en.wikipedia.org/wiki/Huffman_coding), and easy methods to generate one
//! # Example
//! ```
//! use huffman_comprs::*;
//!
//! let input = "This is a test input";
//!
//! // A Huffman tree can be generated from anything
//! // that can be made into a &str
//! let huffman = Huffman::from(input);
//!
//! assert_eq!(vec![true, false, true], huffman.get_code('i').unwrap());
//! ```

use std::collections::BTreeMap;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rz")]
mod rz;
pub use rz::RZFile;

/// A huffman encoding metadata tree.
/// # Examples
/// ```
/// use huffman_comprs::*;
///
/// let script = "aabcd";
///
/// let huffman = Huffman::from(script);
///
/// assert_eq!(vec![false], huffman.get_code('a').unwrap());
/// ```
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct Huffman {
    freq: usize,
    left: Option<Box<Huffman>>,
    right: Option<Box<Huffman>>,
    contents: Vec<char>,
}

impl Huffman {
    const fn new(contents: Vec<char>, freq: usize) -> Self {
        Self {
            freq,
            left: None,
            right: None,
            contents,
        }
    }
    fn build_from_children(left: Self, right: Self) -> Self {
        let mut contents = left.contents.clone();
        contents.append(&mut right.contents.clone());

        Self {
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            contents,
        }
    }
    /// Gets the code of a specified character
    ///
    /// # Errors
    /// Returns `None` if no matching code
    /// is found in the tree
    pub fn get_code(&self, to_get: char) -> Option<Vec<bool>> {
        let mut code = Vec::new();
        self._get_code(to_get, &mut code);

        if !code.is_empty() {
            Some(code)
        } else {
            None
        }
    }
    fn _get_code(&self, to_get: char, code: &mut Vec<bool>) {
        if let Some(left) = &self.left {
            if left.contents.contains(&to_get) {
                code.push(false);
                left._get_code(to_get, code);
            }
        }
        if let Some(right) = &self.right {
            if right.contents.contains(&to_get) {
                code.push(true);
                right._get_code(to_get, code);
            }
        }
    }
    /// Attempts to get the `char` associated with a given code
    /// # Errors
    /// Returns `None` if no matching code
    /// is found in the tree
    pub fn get_char(&self, mut input: Vec<bool>) -> Option<char> {
        input.reverse();

        self._get_char(&mut input)
    }
    fn _get_char(&self, input: &mut Vec<bool>) -> Option<char> {
        if self.contents.len() == 1 {
            Some(self.contents[0])
        } else if input.pop().unwrap() {
            if let Some(right) = &self.right {
                right._get_char(input)
            } else {
                None
            }
        } else if let Some(left) = &self.left {
            left._get_char(input)
        } else {
            None
        }
    }
    /// Attempts to reconstruct a String from a given Vec<bool>, also taking
    /// a u8 'zeros', indicating how many '0's are appended upon the end of
    /// input. This should be the fifth byte of the .rz file
    pub fn reconstruct(&self, mut data: Vec<bool>, zeros: u8) -> Option<String> {
        let mut to_return = String::new();

        data.reverse();

        for _ in 0..zeros {
            data.pop();
        }

        data.reverse();

        while !data.is_empty() {
            let c = match self._get_char(&mut data) {
                Some(c) => c,
                None => break,
            };
            to_return.push(c);
        }

        Some(to_return)
    }
    /// The frequency of all the characters in the huffman tree.
    /// This value should be equal to the total length of the string
    /// used to generate this Huffman tree
    pub const fn freq(&self) -> usize {
        self.freq
    }
    /// Gets a reference to the current tree's contents
    pub const fn contents(&self) -> &Vec<char> {
        &self.contents
    }
    /// Converts self to a `BTreeMap<char, Vec<bool>>`, to allow for faster querying
    pub fn to_btree(&self) -> BTreeMap<char, Vec<bool>> {
        let mut b_tree = BTreeMap::new();

        if let Some(left) = &self.left {
            left._to_btree(&mut b_tree, vec![false]);
        }
        if let Some(right) = &self.right {
            right._to_btree(&mut b_tree, vec![true]);
        }

        b_tree
    }
    fn _to_btree(&self, b_tree: &mut BTreeMap<char, Vec<bool>>, path: Vec<bool>) {
        if self.contents.len() == 1 {
            b_tree.insert(self.contents[0], path);
        } else {
            if let Some(left) = &self.left {
                let mut left_path = path.clone();
                left_path.push(false);
                left._to_btree(b_tree, left_path);
            }
            if let Some(right) = &self.right {
                let mut right_path = path;
                right_path.push(true);
                right._to_btree(b_tree, right_path);
            }
        }
    }
    /// Attempts to compress a given `&str` to a `Vec<bool>`, representing it's
    /// compressed version
    ///
    /// # Errors
    /// This method will fail and return `None` if any of the characters in `input`
    /// are not contained in self's tree
    pub fn compress(&self, input: &str) -> Option<Vec<bool>> {
        let mut output = Vec::with_capacity(input.len());

        let symbols = self.to_btree();

        for character in input.chars() {
            let c = symbols.get(&character)?;
            output.reserve(c.len());
            for t in c {
                output.push(*t);
            }
        }

        Some(output)
    }
    /// A utility function, splitting up `a` into a `Vec<bool>`, representing
    /// it's bits
    pub fn u8_to_bits(a: u8) -> Vec<bool> {
        vec![
            a & 128 == 128,
            a & 64 == 64,
            a & 32 == 32,
            a & 16 == 16,
            a & 8 == 8,
            a & 4 == 4,
            a & 2 == 2,
            a & 1 == 1,
        ]
    }
}

impl PartialEq for Huffman {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for Huffman {}

impl PartialOrd for Huffman {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.freq.cmp(&self.freq))
    }
}

impl Ord for Huffman {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl From<&str> for Huffman {
    fn from(buf: &str) -> Self {
        let mut contents = Vec::new();
        for character in buf.chars() {
            if let Some(i) = contents
                .iter()
                .position(|a: &Huffman| a.contents[0] == character)
            {
                contents[i].freq += 1;
            } else {
                contents.push(Huffman::new(vec![character], 1));
            }
        }

        while contents.len() > 1 {
            contents.sort();

            let parent =
                Huffman::build_from_children(contents.pop().unwrap(), contents.pop().unwrap());
            contents.push(parent);
        }

        contents.pop().unwrap()
    }
}

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
//! assert_eq!(bitvec![Msb0, u8; 1, 0, 1], huffman.get_code('i').unwrap());
//! ```

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

pub use bitvec::prelude::*;
pub use bitvec::vec::BitVec;

/// A huffman encoding metadata tree.
/// # Examples
/// ```
/// use huffman_comprs::*;
///
/// let script = "aabcd";
///
/// let huffman = Huffman::from(script);
///
/// assert_eq!(bitvec![Msb0, u8; 0], huffman.get_code('a').unwrap());
/// ```
#[cfg_attr(
    feature = "serde_support",
    derive(Serialize, Deserialize),
    serde(rename = "h")
)]
#[derive(Default, Debug, Clone)]
pub struct Huffman {
    #[cfg_attr(feature = "serde_support", serde(rename = "f"))]
    freq: usize,
    #[cfg_attr(feature = "serde_support", serde(rename = "l"))]
    left: Option<Box<Huffman>>,
    #[cfg_attr(feature = "serde_support", serde(rename = "r"))]
    right: Option<Box<Huffman>>,
    #[cfg_attr(feature = "serde_support", serde(rename = "c"))]
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
    pub fn get_code(&self, to_get: char) -> Option<BitVec> {
        let mut code = BitVec::new();
        self._get_code(to_get, &mut code);

        if !code.is_empty() {
            Some(code)
        } else {
            None
        }
    }
    fn _get_code(&self, to_get: char, code: &mut BitVec) {
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
    /// The frequency of all the characters in the huffman tree.
    /// Should be equal to the length of the given input
    pub const fn freq(&self) -> usize {
        self.freq
    }
    /// Gets a reference to the current tree's contents
    pub const fn contents(&self) -> &Vec<char> {
        &self.contents
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

impl<'a, T: Into<&'a str>> From<T> for Huffman {
    fn from(buf: T) -> Self {
        let buf = buf.into();

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

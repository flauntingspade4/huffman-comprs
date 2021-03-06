#![warn(clippy::pedantic, clippy::nursery)]

//! A small crate, defining a [Huffman tree](https://en.wikipedia.org/wiki/Huffman_coding), and easy methods to generate one
//! # Example
//! ```
//! use huffman_comprs::Huffman;
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
/// use huffman_comprs::Huffman;
///
/// let script = "aabcd";
///
/// let huffman = Huffman::from(script);
///
/// assert_eq!(vec![false], huffman.get_code('a').unwrap());
/// ```
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    freq: usize,
    left: Option<Box<Huffman<T>>>,
    right: Option<Box<Huffman<T>>>,
    contents: Vec<T>,
}

impl<T> Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    fn new(contents: Vec<T>, freq: usize) -> Self {
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
    #[must_use]
    pub fn get_code(&self, to_get: T) -> Option<Vec<bool>> {
        let mut code = Vec::new();
        self._get_code(to_get, &mut code);

        if code.is_empty() {
            None
        } else {
            Some(code)
        }
    }
    fn _get_code(&self, to_get: T, code: &mut Vec<bool>) {
        if let Some(left) = &self.left {
            if left.contents.contains(&to_get) {
                code.push(false);
                left._get_code(to_get, code);
            } else if let Some(right) = &self.right {
                if right.contents.contains(&to_get) {
                    code.push(true);
                    right._get_code(to_get, code);
                }
            }
        }
    }
    /// Attempts to get the `char` associated with a given code.
    /// # Errors
    /// Returns `None` if no matching code is found in the tree
    #[must_use]
    pub fn get_char(&self, mut input: Vec<bool>) -> Option<T> {
        input.reverse();
        self._get_char(&mut input)
    }
    fn _get_char(&self, input: &mut Vec<bool>) -> Option<T> {
        if self.contents.len() == 1 {
            Some(self.contents[0].clone())
        } else if input.pop()? {
            self.right.as_ref().and_then(|right| right._get_char(input))
        } else {
            self.left.as_ref().and_then(|left| left._get_char(input))
        }
    }
    /// The frequency of all the characters in the huffman tree.
    /// This value should be equal to the total length of the string
    /// used to generate this Huffman tree
    #[must_use]
    pub fn freq(&self) -> usize {
        self.freq
    }
    /// Gets a reference to the current tree's contents
    #[must_use]
    pub fn contents(&self) -> &Vec<T> {
        &self.contents
    }
    /// Converts self to a `BTreeMap<char, Vec<bool>>`, to allow for faster compression
    #[must_use]
    pub fn to_btree(&self) -> BTreeMap<T, Vec<bool>> {
        let mut b_tree = BTreeMap::new();

        if let Some(left) = &self.left {
            left._to_btree(&mut b_tree, vec![false]);
        }
        if let Some(right) = &self.right {
            right._to_btree(&mut b_tree, vec![true]);
        }

        b_tree
    }
    fn _to_btree(&self, b_tree: &mut BTreeMap<T, Vec<bool>>, path: Vec<bool>) {
        if self.contents.len() == 1 {
            b_tree.insert(self.contents[0].clone(), path);
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
    /// Attempts to reconstruct a String from a given Vec<bool>, also taking
    /// a u8 'zeros', indicating how many '0's are appended upon the end of
    /// input. This should be the fifth byte of the .rz file
    #[must_use]
    pub fn reconstruct(&self, mut data: Vec<bool>, zeros: u8) -> Option<Vec<T>> {
        let mut to_return = Vec::new();

        for _ in 0..zeros {
            data.pop();
        }

        while !data.is_empty() {
            to_return.push(self._get_char(&mut data)?);
        }

        Some(to_return)
    }
}

impl<T> Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    /// Attempts to compress a given `&[T]` to a `Vec<bool>`, representing it's
    /// compressed version
    ///
    /// # Errors
    /// This method will fail and return `None` if any of the characters in `input`
    /// are not contained in self's tree
    #[must_use]
    pub fn compress(&self, input: &[T]) -> Option<Vec<bool>> {
        let mut output = Vec::with_capacity(input.len());

        let symbols = self.to_btree();

        for character in input.iter() {
            let c = symbols.get(character)?;
            output.reserve(c.len());
            for t in c {
                output.push(*t);
            }
        }

        Some(output)
    }
}

impl Huffman<char> {
    /// Attempts to compress a given `&str` to a `Vec<bool>`, representing it's
    /// compressed version
    ///
    /// # Errors
    /// This method will fail and return `None` if any of the characters in `input`
    /// are not contained in self's tree
    #[must_use]
    pub fn compress_str(&self, input: &str) -> Option<Vec<bool>> {
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
}

impl<T> PartialEq for Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl<T> Eq for Huffman<T> where T: Serialize + Ord + Clone {}

impl<T> PartialOrd for Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.freq.cmp(&self.freq))
    }
}

impl<T> Ord for Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl<T> From<Vec<T>> for Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    fn from(buf: Vec<T>) -> Self {
        Self::from(&buf)
    }
}

impl<T> From<&Vec<T>> for Huffman<T>
where
    T: Serialize + Ord + Clone,
{
    fn from(buf: &Vec<T>) -> Self {
        let mut contents = Vec::new();
        for character in buf.iter() {
            if let Some(i) = contents
                .iter()
                .position(|a: &Self| a.contents[0] == *character)
            {
                contents[i].freq += 1;
            } else {
                contents.push(Self::new(vec![character.clone()], 1));
            }
        }

        while contents.len() > 1 {
            contents.sort();

            let parent =
                Self::build_from_children(contents.pop().unwrap(), contents.pop().unwrap());
            contents.push(parent);
        }

        contents.pop().unwrap()
    }
}

impl From<&str> for Huffman<char> {
    fn from(buf: &str) -> Self {
        let mut contents = Vec::new();
        for character in buf.chars() {
            if let Some(i) = contents
                .iter()
                .position(|a: &Self| a.contents[0] == character)
            {
                contents[i].freq += 1;
            } else {
                contents.push(Self::new(vec![character], 1));
            }
        }

        while contents.len() > 1 {
            contents.sort();

            let parent =
                Self::build_from_children(contents.pop().unwrap(), contents.pop().unwrap());
            contents.push(parent);
        }

        contents.pop().unwrap()
    }
}

/// A utility function, splitting up `byte` into a `Vec<bool>`, representing
/// it's bits
#[must_use]
#[inline(always)]
pub fn u8_to_bits(byte: u8) -> Vec<bool> {
    vec![
        byte & 1 == 1,
        byte & 2 == 2,
        byte & 4 == 4,
        byte & 8 == 8,
        byte & 16 == 16,
        byte & 32 == 32,
        byte & 64 == 64,
        byte & 128 == 128,
    ]
}

type Child = Option<Box<Huffman>>;

#[derive(Default, Debug, Clone)]
pub struct Huffman {
    freq: usize,
    pub left: Child,
    pub right: Child,
    contents: Vec<char>,
}

impl Huffman {
    fn new(contents: Vec<char>, freq: usize) -> Self {
        Self {
            freq,
            left: None,
            right: None,
            contents,
        }
    }
    fn from_children(left: Self, right: Self) -> Self {
        let mut contents = left.contents.clone();
        contents.append(&mut right.contents.clone());

        Self {
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            contents,
        }
    }
    /*fn add_child(&mut self, contents: char, freq: usize) {
        self.contents.push(contents);
        self.freq += freq;
        if let Some(left) = &mut self.left {
            if let Some(right) = &mut self.right {
                left.max(right).add_child(contents, freq);
            } else {
                self.right = Some(Box::new(Self::new(vec![contents], freq)));
            }
        } else {
            self.left = Some(Box::new(Self::new(vec![contents], freq)));
        }
    }*/
    pub fn freq(&self) -> usize {
        self.freq
    }
    pub fn is_full(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }
    pub fn contents(&self) -> &Vec<char> {
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

        let mut all_contents: Vec<(char, usize)> = Vec::new();

        for byte in buf.chars() {
            if let Some(i) = all_contents.iter().position(|a| a.0 == byte) {
                all_contents[i].1 += 1;
            } else {
                all_contents.push((byte, 1));
            }
        }

        let mut contents = Vec::with_capacity(all_contents.len());
        for item in all_contents {
            contents.push(Self::new(vec![item.0], item.1));
        }

        while contents.len() > 1 {
            contents.sort();

            let parent = Huffman::from_children(contents.pop().unwrap(), contents.pop().unwrap());
            contents.push(parent);
        }

        contents.pop().unwrap()
    }
}

#[test]
fn from_hello_world() {
    let script = include_str!("../script.txt");

    let huffman = Huffman::from(script);

    println!("{:#?}", huffman);
}

/*#[test]
fn add_child() {
    let mut huffman = Huffman::default();

    huffman.add_child('a', 2);
    huffman.add_child('b', 1);
    huffman.add_child('c', 1);
    huffman.add_child('d', 1);

    println!("{:?}", huffman);
}*/

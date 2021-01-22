use std::{convert::TryFrom, fs};

use huffman_comprs::{Huffman, RZFile};

use clap::{App, Arg, SubCommand};

// An example app for compression and decompression, using clap to parse arguments
fn main() {
    let matches = App::new("huffman-comprs-CLI")
        .version("0.1.0")
        .author("Elliot W")
        .about("Compresses and decompresses text files to .rz and from .txt")
        // Subcommand for compression
        .subcommand(
            SubCommand::with_name("compress")
                .about("Compresses a given text file")
                .version("0.1.0")
                .author("Elliot W")
                .arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .index(1)
                        .help("Input text file"),
                ),
        )
        // Subcommand for decompresion
        .subcommand(
            SubCommand::with_name("decompress")
                .about("Decompresses a given rz file")
                .version("0.1.0")
                .author("Elliot W")
                .arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .index(1)
                        .help("Input rz file"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("compress") {
        let path = matches.value_of("INPUT").unwrap();

        let input = fs::read(path).unwrap();

        // Generates a Huffman tree from the given input
        let tree: Huffman<u8> = Huffman::from(&input);

        // Compresses the given input, using the Huffman tree
        let data = tree.compress(&input).unwrap();

        // Turns the Huffman tree and compressed data to a RZ file
        let file = RZFile::new(tree, data);

        // Saves said file
        file.save_to_file(format!("{}.rz", path)).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("decompress") {
        let input = matches.value_of("INPUT").unwrap();

        let buf = fs::read(input).unwrap();

        // Generates an RZ file from input's contents
        let file = RZFile::try_from(buf.as_slice()).unwrap();

        // Pre-allocates for data, to reduce the amount of allocations nededed
        let mut data = Vec::with_capacity(buf.len() * 8);

        // Compressed data is represented in fewer bits than a byte, thus the compressed data must be split into a Vec<bool>
        for a in file.data() {
            data.append(&mut huffman_comprs::u8_to_bits(*a));
        }

        // Reconstructs (decompresses) the data, using the given huffman tree, before converting it to a Vec<u8>
        let contents = file.tree.reconstruct(data, file.zeros()).unwrap().to_vec();

        // Parses the file extension from the input, assuming input's file extension is `.rz`
        let file_name = match input[..input.len() - 3].rfind('.') {
            Some(t) => input.split_at(t).0,
            None => input,
        };

        // Writes to a text file of the same name as input, excluding the '.rz'
        fs::write(format!("{}.txt", file_name), contents).unwrap();
    }
}

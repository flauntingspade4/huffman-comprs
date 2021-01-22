use std::convert::TryFrom;

use huffman_comprs::{Huffman, RZFile};

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("huffman-comprs-CLI")
        .version("0.1.0")
        .author("Elliot W")
        .about("Compresses and decompresses text files to .rz and from .txt")
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

        let input = match std::fs::read(path) {
            Ok(t) => t,
            Err(_) => path.as_bytes().to_vec(),
        };

        let tree = Huffman::from(&input);

        let data = tree.compress(&input).unwrap();

        let file = RZFile::new(tree, data);

        file.save_to_file(format!("{}.rz", path)).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("decompress") {
        let input = matches.value_of("INPUT").unwrap();

        let buf = std::fs::read(input).unwrap();

        let file = RZFile::try_from(buf.as_slice()).unwrap();

        let mut data = Vec::with_capacity(buf.len() * 8);

        for a in file.data() {
            data.append(&mut huffman_comprs::u8_to_bits(*a));
        }

        let contents = file.tree.reconstruct(data, file.zeros()).unwrap().to_vec();

        let file_name = match input.rfind('.') {
            Some(t) => input.split_at(t).0,
            None => input,
        };

        std::fs::write(format!("{}", file_name), contents).unwrap();
    }
}

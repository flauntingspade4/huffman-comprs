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

        let input = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => path.to_string(),
        };

        let tree = Huffman::from(input.as_str());

        let data = tree.compress(input.as_str()).unwrap();

        let file = RZFile::new(tree, data);

        file.save_to_file(format!("{}.rz", path)).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("decompress") {
        let input = matches.value_of("INPUT").unwrap();

        let buf = std::fs::read(input).unwrap();

        let file = RZFile::from(buf.as_slice());

        let mut data = Vec::with_capacity(buf.len() * 8);

        for a in file.data() {
            data.append(&mut Huffman::u8_to_bits(*a));
        }

        let contents = file
            .tree
            .reconstruct(data, file.zeros)
            .unwrap()
            .as_bytes()
            .to_vec();

        let file_name = match input[..input.len() - 3].rfind('.') {
            Some(t) => input.split_at(t).0,
            None => input,
        };

        std::fs::write(format!("{}.txt", file_name), contents).unwrap();
    }
}

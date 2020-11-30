# huffman-comprs

huffman-comprs is a rust implementation of a Huffman tree, with a work-in-progress CLI to go with it, allowing fast compression and decompression from the associated .rz filetype

# Usage

Both the library, and a basic cli are avaliable:

## Library

In `Cargo.toml` :

``` toml
[dependencies]
huffman-comprs = {git = "https://github.com/flauntingspade4/huffman-comprs"}
```

In `main.rs` :

``` rust
use comprs::Huffman;

fn main() {
	let input = "This is a test input";

	// A Huffman tree can be generated from anything
	// that can be made into a &str
	let huffman = Huffman::from(input);

	assert_eq!("101".to_string(), huffman.get_code('i').unwrap());
}
```

## CLI

Function      | Description                               | Usage
--------------|-------------------------------------------|-----------------------------------------------|
compress      | Compresses the given text file 			  | `huffman-comprs-cli compress <FILENAME>`
decompress	  | Decompresses the given `.rz` file 		  | `huffman-comprs-cli decompress <FILENAME>.rz`

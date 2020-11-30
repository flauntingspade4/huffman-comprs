# huffman-comprs

A basic implementation of a Huffman Tree, in rust, with methods to generate it from a string reference

# Example

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

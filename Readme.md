# pretty-print

`pretty-print` is a Rust library that provides a convenient macro for pretty-printing complex data structures using Rust's `println!("{:#?}", x)` functionality. Instead of manually writing `println!("{:#?}", x)` every time, you can use the `pretty_print!` macro to simplify your code.

## Installation

To use `pretty-print` in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
pretty-print = "0.1.0"
```

Then, run:

```bash
cargo build
```

## Usage

After adding pretty-print to your dependencies, you can use the `pretty_print!` macro to pretty-print your data structures in an easy-to-read format.

### Example

```rust
use pretty_print::pretty_print;

fn main() {
    let my_data = vec![1, 2, 3, 4];
    pretty_print!(my_data);
}
```

This will output:

```
[
    1,
    2,
    3,
    4,
]
```

## Features

- Simplified pretty-printing: No need to write `println!("{:#?}", x)` every time.
- Clean, readable output: Helps in debugging complex structures.

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue to improve the library or add more features.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

# rs-enry
Rust bindings for Enry

## Prerequisites

To use this library you should have a Go compiler available on your path.

## Development

The bindings depend on **go-enry v2.6.1** library that is vendored as a submodule in this project.

```
git clone --recurse-submodules https://github.com/go-enry/rs-enry

# or
git submodule init
git submodule update
```

To run the tests

```
cargo test
```

## Usage in Rust project


Simply add this line to your dependency list.

```toml
[dependencies]
enry = { git = "https://github.com/go-enry/rs-enry", branch = "master" }
```

## Example

```rust
use enry::{get_languages, get_language_by_content, get_language_extensions, get_language};
use std::fs;

fn main() {
    let filename = "src/lib.rs";
    let content = fs::read_to_string(filename).unwrap();

    println!("{:?}", get_language_by_content(filename, content.as_str()).unwrap());
    println!("{:?}", get_languages(filename, content.as_str()).unwrap());
    println!("{:?}", get_language_extensions("Rust").unwrap());
    println!("{:?}", get_language(filename, content.as_str()).unwrap());
}
```

The program above yields following:

```text
Guess { language: "Rust", safe: true }
["Rust"]
[".rs", ".rs.in"]
"Rust"
```

You can find more examples on how to use it in `tests/test_enry.rs`.
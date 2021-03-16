# rs-enry
Rust bindings for Enry

## Build

```bash
$ pushd go-enry && make static && popd
$ cargo build
```

We build static library for CGo wrapper `libenry` and then make rust bindings through Rust `ffi` library.

## Usage in Rust project

First, add the library as submodule to your project:

```bash
$ git submodule add git@github.com:go-enry/rs-enry.git
```

After that specify the path dependency in your `Cargo.toml` like that:

```toml
[dependencies]
enry = { path = "path/to/submodule/rs-enry/" }
```

## Example

```rust
use enry::{get_languages, get_language_by_content, get_language_extensions, get_language};
use std::fs;

fn main() {
    let filename = "src/lib.rs";
    let content = fs::read_to_string(filename).unwrap();

    println!("{:?}", get_language_by_content(filename, content.as_str()));
    println!("{:?}", get_languages(filename, content.as_str()));
    println!("{:?}", get_language_extensions("Rust"));
    println!("{:?}", get_language(filename, content.as_str()));
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
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

Add search path for library into your `rustc` flags, possible way of doing this is to add following to your `.cargo/config`:

```toml
# platform here should correspond to your target platform
[target.platform.enry]
rustc-link-search = ["path/to/rs-enry/go-enry/.shared"]
rustc-link-lib = ["enry"]
```

Full list of platforms supported by Rust can be found [here](https://doc.rust-lang.org/nightly/rustc/platform-support.html).


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
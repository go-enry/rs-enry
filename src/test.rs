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

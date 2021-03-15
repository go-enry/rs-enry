
fn main() {
    let path = "go-enry/.shared";
    let lib = "enry";

    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static={}", lib);
}
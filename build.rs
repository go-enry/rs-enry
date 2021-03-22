use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_name = "libenry.a";

    Command::new("go")
        .current_dir("go-enry")
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(format!("{}/{}", out_dir, out_name))
        .arg("shared/enry.go")
        .status()
        .expect("can't compile Go library");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=enry");
}

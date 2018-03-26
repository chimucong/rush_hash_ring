use std::env::current_dir;
use std::process::Command;

fn main() {
    let build_shell = current_dir().unwrap().join("build_c");
    Command::new(build_shell).status().unwrap();
    let lib_dir = current_dir().unwrap().join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=hash_ring");
}
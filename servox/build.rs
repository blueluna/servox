use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Put the linker script in output
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::copy("memory.x", out_dir.join("memory.x")).unwrap();
    println!("cargo:rerun-if-changed=memory.x");
}

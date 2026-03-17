use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let link_x = include_bytes!("link.x.in");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = File::create(out_dir.join("link.x")).unwrap();
    file.write_all(link_x).unwrap();
    fs::write(out_dir.join("link.x"), link_x).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=link.x.in");
}

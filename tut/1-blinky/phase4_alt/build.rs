use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Re-run script when init.S or layout.ld changes
    println!("cargo:rerun-if-changed=src/init.S");
    println!("cargo:rerun-if-changed=layout.ld");
    let out_dir = env::var("OUT_DIR").unwrap();
    let init_o = Path::new(&out_dir).join("init.o");

    // Compile init.S to init.o
    Command::new("clang")
        .args(&[
            "-target",
            "aarch64-unknown-none",
            "-c",
            "src/init.S",
            "-o",
            init_o.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to compile init.S");

    // Pass path to  linker
    println!("cargo:rustc-link-arg={}", init_o.to_str().unwrap());
    println!("cargo:rustc-link-arg=-Tlayout.ld");
}

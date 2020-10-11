extern crate cc;

use cc::Build;

fn main() {
    Build::new()
        .cpp(true)
        .file("hellopp.cc")
        .cpp_link_stdlib("stdc++")
        .compile("libhellopp.a");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=hellopp.cc");
}

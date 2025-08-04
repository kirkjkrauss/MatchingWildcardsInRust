// build.rs
use cc;

fn main() {
    cc::Build::new()
        .file("src/fastwildcompare.cpp")
        .compile("fastwildcompare");
}
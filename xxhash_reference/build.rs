fn main() {
    cc::Build::new()
        .file("xxHash/xxhash.c")
        .flag("-march=native")
        .compile("xxhash");
}

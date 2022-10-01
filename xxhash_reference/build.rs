use cfg_if::cfg_if;

fn main() {
    cfg_if! {
        if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
            cc::Build::new()
                .file("xxHash/xxhash.c")
                .flag("-mcpu=apple-m1")
                .compile("xxhash");
        } else {
            cc::Build::new()
                .file("xxHash/xxhash.c")
                .flag("-march=native")
                .compile("xxhash");
        }
    }
}

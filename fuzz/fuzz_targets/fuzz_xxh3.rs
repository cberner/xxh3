#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
pub struct Config {
    hash128: bool,
    data: Vec<u8>,
    seed: u64,
}

fuzz_target!(|config: Config| {
    if config.hash128 {
        assert_eq!(xxh3::hash128_with_seed(&config.data, config.seed), xxhash_reference::xxh3_128(&config.data, config.seed));
    } else {
        assert_eq!(xxh3::hash64_with_seed(&config.data, config.seed), xxhash_reference::xxh3_64(&config.data, config.seed));
    }
});

#[repr(C)]
struct XXHashU128 {
    low64: u64,
    high64: u64,
}

extern "C" {
    fn XXH3_64bits_withSeed(input: *const libc::c_void, length: libc::size_t, seed: u64) -> u64;
    fn XXH3_128bits_withSeed(
        input: *const libc::c_void,
        length: libc::size_t,
        seed: u64,
    ) -> XXHashU128;
}

pub fn xxh3_64(data: &[u8], seed: u64) -> u64 {
    unsafe { XXH3_64bits_withSeed(data.as_ptr() as *const libc::c_void, data.len(), seed) }
}

pub fn xxh3_128(data: &[u8], seed: u64) -> u128 {
    let hash =
        unsafe { XXH3_128bits_withSeed(data.as_ptr() as *const libc::c_void, data.len(), seed) };

    ((hash.high64 as u128) << 64) | hash.low64 as u128
}

#[cfg(test)]
mod test {
    use crate::{xxh3_128, xxh3_64};

    #[test]
    fn hash() {
        let data = vec![0, 1, 2, 3];
        assert_ne!(xxh3_64(&data, 0), 0);
        assert_ne!(xxh3_128(&data, 0), 0);
    }
}

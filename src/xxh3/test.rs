// The known-answer tests run everywhere, including under Miri. The other tests compare against
// the C reference implementation, except under Miri, which can't call C. There the generic
// implementation stands in for it, and known_answers checks that one against the reference

use super::*;
// The tests detect CPU features at runtime with or without the std feature, so that they test
// every implementation the CPU can run
#[cfg(all(target_arch = "aarch64", target_endian = "little"))]
use std::arch::is_aarch64_feature_detected;

#[cfg(not(miri))]
const SEEDS: &[u64] = &[
    0,
    1,
    0x9e37_79b9_7f4a_7c15,
    0xffff_ffff,
    0x1_0000_0000,
    0x8000_0000_0000_0000,
    u64::MAX,
];
// Miri is slow, so it gets fewer
#[cfg(miri)]
const SEEDS: &[u64] = &[0, 0x9e37_79b9_7f4a_7c15, u64::MAX];

// (input length, seed, 64-bit hash, 128-bit hash) of test_data(input length), from the C
// reference implementation. The lengths are on both sides of every boundary between the code
// paths for short inputs. Above 240 bytes, they're on both sides of the first few stripe & block
// boundaries
#[rustfmt::skip]
const KNOWN_ANSWERS: &[(usize, u64, u64, u128)] = &[
    (0, 0x0, 0x2d06800538d394c2, 0x99aa06d3014798d86001c324468d497f),
    (0, 0x9e3779b97f4a7c15, 0x602b0e2cd6662c8b, 0xd142977a2cca554b4ca5176998171787),
    (1, 0x0, 0xf538d79fd227fb5a, 0x22e0f229e62f4dfaf538d79fd227fb5a),
    (1, 0x9e3779b97f4a7c15, 0x9f411d77030bd686, 0x3113ecd8d0f545bc9f411d77030bd686),
    (2, 0x0, 0xa79cde597ee82968, 0xc5acf4fc95a8ecd3a79cde597ee82968),
    (2, 0x9e3779b97f4a7c15, 0xbc70aa2e0fcca44b, 0x50b99a748bc0aab6bc70aa2e0fcca44b),
    (3, 0x0, 0xa923e223fb2db579, 0xb7ff0aed4fe98ff5a923e223fb2db579),
    (3, 0x9e3779b97f4a7c15, 0xac8172b7ee38552c, 0xa8f87a87c8b7f4bfac8172b7ee38552c),
    (4, 0x0, 0x71023fafbec54755, 0x7163585e882646f58eb11cfde244273d),
    (4, 0x9e3779b97f4a7c15, 0xe82bd2b19a32b723, 0xb75e6b03c65e59843aabd0ce7426e594),
    (5, 0x0, 0x9540055bcb3da0dc, 0xc574160818b442e09ab2241625465f9e),
    (5, 0x9e3779b97f4a7c15, 0xe2c803512982db6a, 0x2637851825365dea5ab8e5c439795e2b),
    (7, 0x0, 0xda00268c11f3ab82, 0x823dc4de3dbf0e012ec75abbf6703b59),
    (7, 0x9e3779b97f4a7c15, 0xdf5c454abdfff860, 0xe0c8bc314ec7191e9b8edef2b2e8518c),
    (8, 0x0, 0xc709accf2cba8434, 0xb19d0b5570d4f7c24cc884410fc431b0),
    (8, 0x9e3779b97f4a7c15, 0x96b2759c6d272c16, 0xf4e23b2196d02e463e8d5ed60a22986b),
    (9, 0x0, 0x2222f4335ae47fbf, 0xba2dc1a661b9dde23694afbed3f7bfac),
    (9, 0x9e3779b97f4a7c15, 0xeb4ced1fa1b1db07, 0x2f9dc14b47387eeb01cf8fb909217aed),
    (12, 0x0, 0xb23c8adca7c73b3a, 0x39e80ce6d9596d5c7d408b8bf1f32373),
    (12, 0x9e3779b97f4a7c15, 0xa564dbd193857771, 0x56a28fe1b51e6a6f453593b28d19e2f2),
    (15, 0x0, 0x3248634ee5d14c39, 0xcbb3a87cf8bc039efd1c39dff92be7e9),
    (15, 0x9e3779b97f4a7c15, 0x543b9e153257f4d7, 0x0ae0cd4ad65028d18d8c9cc97101c8ea),
    (16, 0x0, 0xf39430324abc1245, 0xd59f7d8a670a120cda104e0e92b92ab5),
    (16, 0x9e3779b97f4a7c15, 0x28c77a68b015cab8, 0x8eaece11c97ac866c7f5686de24dbf3c),
    (17, 0x0, 0x45d22d285ec34cdb, 0x855f74874203b1df5fe607dc60f45ad0),
    (17, 0x9e3779b97f4a7c15, 0x724e38be80d5ddcf, 0x914b0c38f4b69b8cc4800e0d1ce42ca7),
    (31, 0x0, 0x7f1af1e91122e58e, 0x956a2ed0a588d6bae8bddd0ff80c56e5),
    (31, 0x9e3779b97f4a7c15, 0x026b19835e6d3ae2, 0x958cef70b3c066cec5ab5ecc4b0baf71),
    (32, 0x0, 0xac71facd8f1e1483, 0x151e2f52a5dde7ef2bc2f6e5de47c30d),
    (32, 0x9e3779b97f4a7c15, 0xf610d230713112cd, 0x8c03ce319a857681bcd3aee640a23028),
    (33, 0x0, 0x7d4834cd82f761f1, 0xac3c5e799a4ad9bb6cfddcb70a6b40f3),
    (33, 0x9e3779b97f4a7c15, 0x42dd947be24f901b, 0xf9859fef069938bcec4361775f8e9b45),
    (63, 0x0, 0x05db3949c48ba4bf, 0xb8cb9bb5c5113c747a4a25bef09cd1e7),
    (63, 0x9e3779b97f4a7c15, 0xaad20b47a696e4fd, 0x0ef207457710b1bea886c4b0adfd27ab),
    (64, 0x0, 0x5f967e5ce7da2152, 0x4acaaacf4922e9e41bdf0a1e32df97c5),
    (64, 0x9e3779b97f4a7c15, 0xdff6689d3f6fd207, 0x5f76ffca79e2e1a38ee08b292fb7f642),
    (65, 0x0, 0x36574896f4c8ef63, 0x380df5dc0734fe9793c601091bdd7f9c),
    (65, 0x9e3779b97f4a7c15, 0xefee41c888059298, 0x3852797662146dda124aaa467baf744e),
    (95, 0x0, 0xd8a88d83b23763ca, 0x15b2d96364b8d00b8e12f5d71b7ac3e6),
    (95, 0x9e3779b97f4a7c15, 0x32f778fd489ca5c6, 0xcccddcd634754ee91d1b5c371edf14ac),
    (96, 0x0, 0x7b22d97b5905e704, 0x6eca0a9b056606c6c7010c1e0f83caaa),
    (96, 0x9e3779b97f4a7c15, 0xb7dc6eb6c4cd76c5, 0x9ebf4586e6c79fb4c2a3b789895f7f80),
    (97, 0x0, 0x53e8595edd9de997, 0xf64af078be1185b2ab264fa223d27377),
    (97, 0x9e3779b97f4a7c15, 0xdd4f69c7cf7c18da, 0x807e0e88ab2fd06af84776dfca82fc19),
    (127, 0x0, 0xe9c5448f9026288b, 0xc323896344e4323f919362487f9bb0fe),
    (127, 0x9e3779b97f4a7c15, 0x0d44b6b3f4d8fbeb, 0xde71723abd2fd3c24216ec7175101de6),
    (128, 0x0, 0xd5fd82121708bef2, 0xb03c567db648673d0346c78ee187f3ca),
    (128, 0x9e3779b97f4a7c15, 0x7547b93a10926bfc, 0xb4ad687268efadff2e0bbd0d90d8b5de),
    (129, 0x0, 0xbd639760fc0b2880, 0x86adf1dcf824c5a7f34f4ab299a70685),
    (129, 0x9e3779b97f4a7c15, 0x4631f12fd85c1675, 0x867be2fec66e9a8477d225e26a72c0d3),
    (143, 0x0, 0xae572a5add1239c0, 0x604a05bd59da385e56f3f4becb0fe938),
    (143, 0x9e3779b97f4a7c15, 0xf1ab3bc834b46078, 0xb52a21312c718c5b79e663af419f31a1),
    (144, 0x0, 0x21c1747b0d826aa4, 0x58654f43118323b762b6c5571a268850),
    (144, 0x9e3779b97f4a7c15, 0x8d4f5b2ba1ef6872, 0xf973db642e17e1a4675fa9256c27c58c),
    (159, 0x0, 0xb85eb1d812ebf10e, 0x25466cb9395932f469ff20feff7cd30d),
    (159, 0x9e3779b97f4a7c15, 0x05d4876738b2a8c9, 0x8fbe8028dcd372950d999b2cc9067977),
    (160, 0x0, 0xe7a92c7c889bd5e2, 0x098f883b52204d9b0762057ace64a53c),
    (160, 0x9e3779b97f4a7c15, 0x761c2ada10655070, 0xfa4cfb779edacfa2c741bc5bdc36674b),
    (161, 0x0, 0x6716ced7b33c3904, 0xdd0b7d3c6691147f3d502874dfc0ca89),
    (161, 0x9e3779b97f4a7c15, 0x6ed9cd3d9995713a, 0x832fa7543bad0ba652d08c4f13d82c2d),
    (191, 0x0, 0x07da6a39f0365724, 0x24ea137a7968c88bf0e389c14c0bf113),
    (191, 0x9e3779b97f4a7c15, 0xa440cf29313f0a7b, 0x744cc23a0fc3feefa643fd353ba99371),
    (192, 0x0, 0xb0c9dcdafe6d9cd5, 0x26baff3c524bc33e3bb7e3549c52c8a8),
    (192, 0x9e3779b97f4a7c15, 0x61cc117890871bcd, 0x700b0bd4964095fc36d8eb22dd24c17b),
    (193, 0x0, 0xf65c56d87c2c4e23, 0x4a00958aebc122ba52daeb6425e8c20a),
    (193, 0x9e3779b97f4a7c15, 0x6a002e438f14bc2c, 0x79b328cf88e6d915bcfe79d6b55f9580),
    (239, 0x0, 0x411c6b04a5c4da5c, 0x80d7ba40cdbfd79956c76df7ab85e8a8),
    (239, 0x9e3779b97f4a7c15, 0xac3d03c3a0eaeb23, 0xb044ba02d071d6d80453d278a70c5530),
    (240, 0x0, 0xc20b499b4eca149d, 0x900a1bc3b899881746a9f03f26915c0a),
    (240, 0x9e3779b97f4a7c15, 0x69ad3df8a0b49709, 0x3d8edd0b1f54eef2ea6d907b5c83cd1d),
    (241, 0x0, 0x15d313c6669c668c, 0xb98e6930da993da115d313c6669c668c),
    (241, 0x9e3779b97f4a7c15, 0x3227a8e371ca7cb4, 0xc2ed9eeb224bd05d3227a8e371ca7cb4),
    (255, 0x0, 0x1642a53373964c19, 0x8d5ff90989c406221642a53373964c19),
    (255, 0x9e3779b97f4a7c15, 0x88251ad0b9baa4d4, 0xcd2505640486456088251ad0b9baa4d4),
    (256, 0x0, 0x782895fbd13d85de, 0x5e58f84b07a84274782895fbd13d85de),
    (256, 0x9e3779b97f4a7c15, 0x885cd7769cf22a56, 0x6beba8826065aec4885cd7769cf22a56),
    (257, 0x0, 0xefd14f70cfcdecfa, 0x13fe2141de89b1ebefd14f70cfcdecfa),
    (257, 0x9e3779b97f4a7c15, 0x00989ecceeb6b74c, 0x2c9e13c5f254881a00989ecceeb6b74c),
    (319, 0x0, 0x7a511a985e9d4e51, 0x57b0fa131a869a0e7a511a985e9d4e51),
    (319, 0x9e3779b97f4a7c15, 0xd7c08f77164de400, 0x9e93be9cbd82bfa8d7c08f77164de400),
    (320, 0x0, 0x47d09dad84de4130, 0x06345de2ea99139e47d09dad84de4130),
    (320, 0x9e3779b97f4a7c15, 0x2de0dbed672e867e, 0xcefd26b194d3e3752de0dbed672e867e),
    (321, 0x0, 0x9e68a3eb2a3a5305, 0xb206eaca5ef7f0d39e68a3eb2a3a5305),
    (321, 0x9e3779b97f4a7c15, 0x341c009058227c44, 0x633b8feffc0d8d2a341c009058227c44),
    (1023, 0x0, 0x8f85c61ecbf55026, 0x42d46569fca1bcdf8f85c61ecbf55026),
    (1023, 0x9e3779b97f4a7c15, 0x2b6746b44a44f471, 0x17ee57de235344802b6746b44a44f471),
    (1024, 0x0, 0x167a4ff2b7f6e8df, 0x42b9fc9a5558669d167a4ff2b7f6e8df),
    (1024, 0x9e3779b97f4a7c15, 0x4d80b642ee2a4dc8, 0xe5dcce39334635c14d80b642ee2a4dc8),
    (1025, 0x0, 0x175b38b1d35c8b95, 0xf841813c059e8d32175b38b1d35c8b95),
    (1025, 0x9e3779b97f4a7c15, 0x30db0d6312aadc5a, 0x1102cbc39d663fb330db0d6312aadc5a),
    (1087, 0x0, 0xcfced334d91d538a, 0xa6640f65e1e66a58cfced334d91d538a),
    (1087, 0x9e3779b97f4a7c15, 0xdbf70e21d251e073, 0xa31cdac53f13081cdbf70e21d251e073),
    (1088, 0x0, 0xa7ae335f263c37ac, 0xb23887e977458fc4a7ae335f263c37ac),
    (1088, 0x9e3779b97f4a7c15, 0x951a0ebb71769e87, 0xf73311a7f134441a951a0ebb71769e87),
    (1089, 0x0, 0xcf5fb001e32a4dd5, 0x598bf64e567f555ccf5fb001e32a4dd5),
    (1089, 0x9e3779b97f4a7c15, 0xd1514f0ab93dd4cd, 0x62df52bbe62de560d1514f0ab93dd4cd),
    (2047, 0x0, 0xe304d1f728cb9246, 0x04fada17474c81b6e304d1f728cb9246),
    (2047, 0x9e3779b97f4a7c15, 0x306d5a199a62073a, 0x3a0cf9fd2e1f27fc306d5a199a62073a),
    (2048, 0x0, 0x12a658eaedebfe21, 0xa73b4d7860c1d55a12a658eaedebfe21),
    (2048, 0x9e3779b97f4a7c15, 0x055525e2d55c8d4e, 0x82aa19fd7c3f95bc055525e2d55c8d4e),
    (2049, 0x0, 0x5277d95f547af988, 0x7715de9de5fdab245277d95f547af988),
    (2049, 0x9e3779b97f4a7c15, 0x06889df594edbc13, 0x46720d8fc69ec7ed06889df594edbc13),
    (4103, 0x0, 0x59ef180881f3c864, 0x85466ce3a3709bf559ef180881f3c864),
    (4103, 0x9e3779b97f4a7c15, 0xba3db95d522e2307, 0x241173665f4448a9ba3db95d522e2307),
];

// Input that doesn't repeat, so an implementation that reads the wrong stripe can't get the right
// hash by coincidence. A shorter output is a prefix of a longer one
fn test_data(len: usize) -> Vec<u8> {
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state.to_le_bytes()[0]
        })
        .collect()
}

struct Implementation {
    name: &'static str,
    hash64: fn(&[u8], u64) -> u64,
    hash128: fn(&[u8], u64) -> u128,
}

// The public functions, plus every implementation of the >240 byte path this CPU can run. The
// public functions only use the fastest one, so the others are only tested by calling them
// directly
fn large_input_implementations() -> Vec<Implementation> {
    // Only targets with SIMD implementations add more
    #[allow(unused_mut)]
    let mut implementations = vec![
        Implementation {
            name: "public",
            hash64: hash64_with_seed,
            hash128: hash128_with_seed,
        },
        Implementation {
            name: "generic",
            hash64: hash64_large_generic,
            hash128: hash128_large_generic,
        },
    ];
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            implementations.push(Implementation {
                name: "avx2",
                // SAFETY: the CPU supports AVX2
                hash64: |data, seed| unsafe { hash64_large_avx2(data, seed) },
                // SAFETY: the CPU supports AVX2
                hash128: |data, seed| unsafe { hash128_large_avx2(data, seed) },
            });
        }
        if is_x86_feature_detected!("avx512f") {
            implementations.push(Implementation {
                name: "avx512",
                // SAFETY: the CPU supports AVX-512F
                hash64: |data, seed| unsafe { hash64_large_avx512(data, seed) },
                // SAFETY: the CPU supports AVX-512F
                hash128: |data, seed| unsafe { hash128_large_avx512(data, seed) },
            });
        }
    }
    #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
    {
        if is_aarch64_feature_detected!("neon") {
            implementations.push(Implementation {
                name: "neon",
                // SAFETY: the CPU supports NEON
                hash64: |data, seed| unsafe { hash64_large_neon(data, seed) },
                // SAFETY: the CPU supports NEON
                hash128: |data, seed| unsafe { hash128_large_neon(data, seed) },
            });
        }
    }
    implementations
}

// Input lengths over 240 bytes
fn large_lengths() -> Vec<usize> {
    if cfg!(miri) {
        // Up to 3 blocks, ending: just past a block boundary, on one, in the stripe after one,
        // and in a later stripe
        vec![241, 1024, 1025, 1089, 2048, 3137]
    } else {
        // Every length up to 3 blocks and 2 stripes, and a few far longer
        let block_len =
            STRIPE_LENGTH * (DEFAULT_SECRET.len() - STRIPE_LENGTH) / SECRET_CONSUME_RATE;
        let mut lengths: Vec<usize> = (241..=3 * block_len + 2 * STRIPE_LENGTH).collect();
        lengths.extend([65_535, 65_536, 65_537, 1 << 20, (1 << 20) + 1]);
        lengths
    }
}

#[cfg(not(miri))]
fn expected_large(data: &[u8], seed: u64) -> (u64, u128) {
    (
        xxhash_reference::xxh3_64(data, seed),
        xxhash_reference::xxh3_128(data, seed),
    )
}

#[cfg(miri)]
fn expected_large(data: &[u8], seed: u64) -> (u64, u128) {
    (
        hash64_large_generic(data, seed),
        hash128_large_generic(data, seed),
    )
}

fn check_large(implementations: &[Implementation], data: &[u8], seed: u64, context: &str) {
    assert!(data.len() > 240);
    let (expected64, expected128) = expected_large(data, seed);
    for implementation in implementations {
        let name = implementation.name;
        let len = data.len();
        assert_eq!(
            (implementation.hash64)(data, seed),
            expected64,
            "{name} 64-bit, len={len} seed={seed:#x} {context}"
        );
        assert_eq!(
            (implementation.hash128)(data, seed),
            expected128,
            "{name} 128-bit, len={len} seed={seed:#x} {context}"
        );
    }
}

#[test]
fn known_answers() {
    let implementations = large_input_implementations();
    for &(len, seed, expected64, expected128) in KNOWN_ANSWERS {
        let data = test_data(len);
        assert_eq!(
            hash64_with_seed(&data, seed),
            expected64,
            "64-bit, len={len} seed={seed:#x}"
        );
        assert_eq!(
            hash128_with_seed(&data, seed),
            expected128,
            "128-bit, len={len} seed={seed:#x}"
        );
        if len > 240 {
            for implementation in &implementations {
                let name = implementation.name;
                assert_eq!(
                    (implementation.hash64)(&data, seed),
                    expected64,
                    "{name} 64-bit, len={len} seed={seed:#x}"
                );
                assert_eq!(
                    (implementation.hash128)(&data, seed),
                    expected128,
                    "{name} 128-bit, len={len} seed={seed:#x}"
                );
            }
        }
    }
}

#[cfg(not(miri))]
#[test]
fn short_inputs() {
    let buffer = test_data(240);
    for len in 0..=240 {
        let data = &buffer[..len];
        for &seed in SEEDS {
            assert_eq!(
                hash64_with_seed(data, seed),
                xxhash_reference::xxh3_64(data, seed),
                "64-bit, len={len} seed={seed:#x}"
            );
            assert_eq!(
                hash128_with_seed(data, seed),
                xxhash_reference::xxh3_128(data, seed),
                "128-bit, len={len} seed={seed:#x}"
            );
        }
    }
}

#[test]
fn large_inputs() {
    let implementations = large_input_implementations();
    let lengths = large_lengths();
    let buffer = test_data(*lengths.iter().max().unwrap());
    for len in lengths {
        for &seed in SEEDS {
            check_large(&implementations, &buffer[..len], seed, "");
        }
    }
}

// The SIMD implementations use unaligned loads, so the input may start at any address
#[test]
fn unaligned_large_inputs() {
    let implementations = large_input_implementations();
    let lengths = [241, 1089];
    let offsets: Vec<usize> = if cfg!(miri) {
        vec![1, 7, 31, 63]
    } else {
        (1..STRIPE_LENGTH).collect()
    };
    let buffer = test_data(lengths.iter().max().unwrap() + STRIPE_LENGTH);
    for len in lengths {
        for &offset in &offsets {
            let data = &buffer[offset..offset + len];
            check_large(
                &implementations,
                data,
                0x9e37_79b9_7f4a_7c15,
                &format!("offset={offset}"),
            );
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[test]
fn simd_secret_generation() {
    for &seed in SEEDS {
        let expected = gen_secret_generic(seed);
        if is_x86_feature_detected!("avx2") {
            // SAFETY: the CPU supports AVX2
            let secret = unsafe { gen_secret_avx2(seed) };
            assert_eq!(secret, expected, "avx2 seed={seed:#x}");
        }
        if is_x86_feature_detected!("avx512f") {
            // SAFETY: the CPU supports AVX-512F
            let secret = unsafe { gen_secret_avx512(seed) };
            assert_eq!(secret, expected, "avx512 seed={seed:#x}");
        }
    }
}

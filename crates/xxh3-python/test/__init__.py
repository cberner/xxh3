from threading import Thread
from unittest import TestCase

import xxh3


def data(length):
    return bytes((i * 7 + 3) % 256 for i in range(length))


SEED = 0x9E3779B97F4A7C15

# (length of data(length), seed, xxh3_64, xxh3_128), from the C reference implementation. The
# lengths cover each of the code paths for inputs up to 240 bytes, and the large-input path, with
# 100,000 bytes being long enough to hash with the GIL released
VECTORS = [
    (0, 0, 0x2D06800538D394C2, 0x99AA06D3014798D86001C324468D497F),
    (0, SEED, 0x602B0E2CD6662C8B, 0xD142977A2CCA554B4CA5176998171787),
    (1, 0, 0x13E608BC156DEFED, 0x22BBB76B211A39BA13E608BC156DEFED),
    (1, SEED, 0x1B4C466098160569, 0x8B0BDE64EBB5391A1B4C466098160569),
    (3, 0, 0xA9088DDA485B481C, 0xCE31763CBF8245A5A9088DDA485B481C),
    (3, SEED, 0xA8BACD847619199E, 0xA6C5358DFF93AF85A8BACD847619199E),
    (4, 0, 0x6D9253B16C8B1ED3, 0x47197970590746B1788A609154B0FE20),
    (4, SEED, 0xE1C585329CF1878E, 0x7FBDADE8669A4AB094C1979D995C4870),
    (8, 0, 0x60539DB630471163, 0xE3BC8A5F461715553CD024E3D63A1588),
    (8, SEED, 0xBC53D62E02F670A4, 0xD63852876FE8A157DE775C059292F841),
    (9, 0, 0xFEFF668361D723A8, 0xC72C88247A9A56D7EAFAB1C7F123109F),
    (9, SEED, 0xD4FB426F424E6E62, 0x0B38A113B694FCAE51E1392707D4BBB9),
    (16, 0, 0xB8C859B0F030B585, 0xCE0B9647AB24F88460D75C5E47D40A24),
    (16, SEED, 0x7775D23337D796B5, 0x4F914088F379A471525DA27D50E50D60),
    (17, 0, 0x714A04408E79B80F, 0xBFD327EDCC2FBD12EEED7654312A26D7),
    (17, SEED, 0x7D1872B1361C0FA6, 0xA61A1CA6A6CBA50B3445A39302223BAF),
    (128, 0, 0x67425A03650261BF, 0x1B1962A096BAC78BC580008B6C92AC53),
    (128, SEED, 0xE9E239440DAC1B3C, 0x0E547FAD963E783E11B625103E3F16E8),
    (129, 0, 0xC664BF3311C6ABC4, 0x293E4968C4619023BD91CE7ACE4D385B),
    (129, SEED, 0xB11455AB08C506D4, 0xB29AA6B7F2BDB673FB2FDD6E76E6384F),
    (240, 0, 0x64556DC6B462A6CF, 0xAD46C1021B076BC704E0B5F034BEE80B),
    (240, SEED, 0x6EA73B2BE19B57C5, 0x9BD1B9F5F322C628ECAE892A3FAC66C3),
    (241, 0, 0x8BEADD3A8874FE17, 0xAC6C3492C3D6B45D8BEADD3A8874FE17),
    (241, SEED, 0xA0462D397650B282, 0x44BD02453C9C891CA0462D397650B282),
    (1024, 0, 0x9B81661C641C72B1, 0x18BC0EACA9A336369B81661C641C72B1),
    (1024, SEED, 0xE955D0AFE88A0F51, 0x3C6F1311239E88B0E955D0AFE88A0F51),
    (100000, 0, 0x0C056F6FCC340974, 0xD9E155DD16E141D00C056F6FCC340974),
    (100000, SEED, 0x687E4DC68AF3C6F5, 0x8E0E25FE49159D85687E4DC68AF3C6F5),
]


class Xxh3TestCase(TestCase):
    def test_version(self):
        self.assertIsInstance(xxh3.__version__, str)

    def test_vectors(self):
        for length, seed, expected_64, expected_128 in VECTORS:
            with self.subTest(length=length, seed=seed):
                self.assertEqual(xxh3.xxh3_64(data(length), seed), expected_64)
                self.assertEqual(xxh3.xxh3_128(data(length), seed), expected_128)
                self.assertEqual(xxh3.xxh3_64(bytearray(data(length)), seed=seed), expected_64)
                self.assertEqual(xxh3.xxh3_128(bytearray(data(length)), seed=seed), expected_128)

    def test_default_seed_is_zero(self):
        self.assertEqual(xxh3.xxh3_64(b"xxh3"), xxh3.xxh3_64(b"xxh3", 0))
        self.assertEqual(xxh3.xxh3_128(b"xxh3"), xxh3.xxh3_128(b"xxh3", 0))

    def test_threads(self):
        results = []
        threads = [
            Thread(target=lambda: results.append(xxh3.xxh3_64(data(100000), SEED)))
            for _ in range(4)
        ]
        for thread in threads:
            thread.start()
        for thread in threads:
            thread.join()
        self.assertEqual(results, [0x687E4DC68AF3C6F5] * 4)

    def test_rejects_other_types(self):
        for value in ["xxh3", memoryview(b"xxh3"), None, 1]:
            with self.subTest(value=value):
                with self.assertRaises(TypeError):
                    xxh3.xxh3_64(value)
                with self.assertRaises(TypeError):
                    xxh3.xxh3_128(value)

    def test_rejects_out_of_range_seeds(self):
        for seed in [-1, 2**64]:
            with self.subTest(seed=seed):
                with self.assertRaises(OverflowError):
                    xxh3.xxh3_64(b"xxh3", seed)
                with self.assertRaises(OverflowError):
                    xxh3.xxh3_128(b"xxh3", seed)

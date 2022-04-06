use ark_ff::{biginteger::BigInteger320 as BigInteger, FftParameters, Fp320, Fp320Parameters};
pub type Fq = Fp320<FpParameters>;

pub struct FpParameters;

impl Fp320Parameters for FpParameters {}
impl FftParameters for FpParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 1;

    #[rustfmt::skip]
    //115792089237316195423570985008687907853269984665561335858920850710301058792495
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xfffffffefffffc2f,
        0xfffffffefffffc2e,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x0000000000000000
    ]);
}

impl ark_ff::FpParameters for FpParameters {
    //115792089237316195423570985008687907853269984665640564039457584007908834671663
    const MODULUS: BigInteger = BigInteger([
        0xfffffffefffffc2f,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x0000000000000000
    ]);

    //4294968273 * 2^64
    const R: BigInteger = BigInteger([0x0, 0x1000003D1, 0x0, 0x0, 0x0]);

    //18446752466076602529 * 2^128
    const R2: BigInteger = BigInteger([0x0, 0x0, 0x000007A2000E90A1, 0x1, 0x0]);

    //57896044618658097711785492504343953926634992332820282019728792003954417335831
    //7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7FFFFE17
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xFFFFFFFF7FFFFE17,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x7FFFFFFFFFFFFFFF,
        0x0,
    ]);

    //57896044618658097711785492504343953926634992332820282019728792003954417335831
    //7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF7FFFFE17
    const T: BigInteger = BigInteger([
        0xFFFFFFFF7FFFFE17,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x7FFFFFFFFFFFFFFF,
        0x0,
    ]);

    //28948022309329048855892746252171976963317496166410141009864396001977208667915
    //3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFBFFFFF0B
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xFFFFFFFFBFFFFF0B,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x3FFFFFFFFFFFFFFF,
        0x0,
    ]);

    //GENERATOR = 3
    const GENERATOR: BigInteger = BigInteger([
        0x0000000000000000,
        0x0000000300000b73,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000
    ]);

    const MODULUS_BITS: u32 = 256;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 64;

    const INV: u64 = 15580212934572586289;
}

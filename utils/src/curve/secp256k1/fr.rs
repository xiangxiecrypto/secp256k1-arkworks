use ark_ff::{biginteger::BigInteger320 as BigInteger, FftParameters, Fp320, Fp320Parameters};
pub type Fr = Fp320<FpParameters>;

pub struct FpParameters;

impl Fp320Parameters for FpParameters {}
impl FftParameters for FpParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 6;

    #[rustfmt::skip]
    //38715229011166149576104664426121941072306984375677460650018045533266598568266
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0x0112cb0f605a214a,
        0x92225daffb794500,
        0x7e42003a6ccb6212,
        0x55980b07bc222114,
        0x0000000000000000
    ]);
}

impl ark_ff::FpParameters for FpParameters {
    //115792089237316195423570985008687907852837564279074904382605163141518161494337
    const MODULUS: BigInteger = BigInteger([
        0xbfd25e8cd0364141,
        0xbaaedce6af48a03b,
        0xfffffffffffffffe,
        0xffffffffffffffff,
        0x0000000000000000
    ]);

    //432420386565659656852420866394968145599 * 2^64
    const R: BigInteger = BigInteger([0x0, 0x402DA1732FC9BEBF, 0x4551231950B75FC4, 0x1, 0x0]);

    //27185382341430777127667013160941942500583767731514336557959633868809832333177
    const R2: BigInteger = BigInteger([
        0x1e004f504dfd7f79,
        0x08fcf59774a052ea,
        0x27c4120fc94e1653,
        0x3c1a6191e5702644,
        0x0000000000000000
    ]);

    //57896044618658097711785492504343953926418782139537452191302581570759080747168
    //7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xDFE92F46681B20A0,
        0x5D576E7357A4501D,
        0xFFFFFFFFFFFFFFFF,
        0x7FFFFFFFFFFFFFFF,
        0x0
    ]);

    //1809251394333065553493296640760748560200586941860545380978205674086221273349
    //3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFAEABB739ABD2280EEFF497A3340D905
    const T: BigInteger = BigInteger([
        0xEEFF497A3340D905,
        0xFAEABB739ABD2280,
        0xFFFFFFFFFFFFFFFF,
        0x3FFFFFFFFFFFFFF,
        0x0
    ]);

    //904625697166532776746648320380374280100293470930272690489102837043110636674
    //1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD755DB9CD5E9140777FA4BD19A06C82
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x777FA4BD19A06C82,
        0xFD755DB9CD5E9140,
        0xFFFFFFFFFFFFFFFF,
        0x1FFFFFFFFFFFFFF,
        0x0
    ]);

    //GENERATOR = 7
    const GENERATOR: BigInteger = BigInteger([
        0x0000000000000000,
        0xc13f6a264e843739,
        0xe537f5b135039e5d,
        0x0000000000000008,
        0x0000000000000000
    ]);

    const MODULUS_BITS: u32 = 256;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 64;

    const INV: u64 = 5408259542528602431;
}

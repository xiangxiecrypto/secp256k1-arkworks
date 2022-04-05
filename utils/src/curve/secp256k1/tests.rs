use ark_algebra_test_templates::fields::{field_test, primefield_test, sqrt_field_test};
use ark_std::test_rng;
use rand::Rng;

use crate::curve::*;

#[test]
fn test_fq() {
    let mut rng = test_rng();
    let a: secp256k1::Fq = rng.gen();
    let b: secp256k1::Fq = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    primefield_test::<secp256k1::Fq>();
}

#[test]
fn test_fr() {
    let mut rng = test_rng();
    let a: secp256k1::Fr = rng.gen();
    let b: secp256k1::Fr = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    primefield_test::<secp256k1::Fr>();
}

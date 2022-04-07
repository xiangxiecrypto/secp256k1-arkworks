use ark_algebra_test_templates::fields::{field_test, primefield_test, sqrt_field_test};
use ark_algebra_test_templates::{curves::*, groups::*};
use ark_ec::AffineCurve;
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

#[test]
fn test_serialize_and_deserialize(){}

#[test]
fn test_secp256k1_projective_curve() {
    curve_tests::<secp256k1::Projective>();
    sw_tests::<secp256k1::Secp256k1Parameters>();
}

#[test]
fn test_secp256k1_projective_group() {
    let mut rng = test_rng();
    let a: secp256k1::Projective = rng.gen();
    let b: secp256k1::Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_secp256k1_generator() {
    let generator = secp256k1::Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

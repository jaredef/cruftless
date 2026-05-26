//! WC-EXT 8 smoke test: verify Montgomery multiplication for P-256
//! against the canonical mod_mul reference across a set of fixtures.
//!
//! The cryptographic primitive `p256_mont_mul(am, bm)` returns
//! `am · bm · R⁻¹ mod p`. The conversion functions `p256_to_mont(a)`
//! and `p256_from_mont(am)` are inverses. For all standard-form
//! a, b in [0, p):
//!
//!   p256_from_mont(p256_mont_mul(p256_to_mont(a), p256_to_mont(b)))
//!     ≡ a · b (mod p)
//!
//! The reference is direct schoolbook multiplication followed by
//! reduction. If the assertion holds across many random fixtures,
//! the Montgomery infrastructure is gold-standard correct.

use rusty_web_crypto::{p256_from_mont, p256_mont_mul, p256_to_mont, BigUInt};

fn p256_p_local() -> BigUInt {
    // Pull P-256 prime from web-crypto's public surface via mod path.
    // (curve_p256().p exposes it; using the raw bytes here keeps the
    // fixture self-contained.)
    BigUInt::from_be_bytes(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ])
}

fn hex_to_big(s: &str) -> BigUInt {
    let bytes: Vec<u8> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect();
    BigUInt::from_be_bytes(&bytes)
}

fn assert_mont_correct(a: &BigUInt, b: &BigUInt) {
    let p = p256_p_local();
    let am = p256_to_mont(a);
    let bm = p256_to_mont(b);
    let cm = p256_mont_mul(&am, &bm);
    let c_via_mont = p256_from_mont(&cm);
    let c_reference = a.mul(b).modulo(&p);
    assert_eq!(
        c_via_mont.to_be_bytes(32),
        c_reference.to_be_bytes(32),
        "Montgomery output diverges from reference at\n  a={:?}\n  b={:?}\n",
        a.to_be_bytes(32),
        b.to_be_bytes(32)
    );
}

#[test]
fn mont_trivial_zero() {
    let zero = BigUInt::zero();
    let one = BigUInt::one();
    assert_mont_correct(&zero, &one);
    assert_mont_correct(&one, &zero);
    assert_mont_correct(&zero, &zero);
}

#[test]
fn mont_trivial_one() {
    let one = BigUInt::one();
    let two = BigUInt::from_be_bytes(&[2]);
    assert_mont_correct(&one, &one);
    assert_mont_correct(&one, &two);
}

#[test]
fn mont_apigithub_qx_qy() {
    // The WC-EXT 1 fixture from api.github.com — known-good 256-bit values.
    let qx = hex_to_big("8ed3d85107094884d4c046e17d3a572273211aa6ae75447e1c4c2c627012de50");
    let qy = hex_to_big("60c59fd560f710ca942b0785d246242a64c59c1b40af904664a6b6b5c727003d");
    assert_mont_correct(&qx, &qy);
    assert_mont_correct(&qy, &qx);
    assert_mont_correct(&qx, &qx);
    assert_mont_correct(&qy, &qy);
}

#[test]
fn mont_near_modulus() {
    // Values close to p exercise the final subtraction in REDC.
    let p = p256_p_local();
    let p_minus_1 = p.sub(&BigUInt::one());
    let p_minus_2 = p.sub(&BigUInt::from_be_bytes(&[2]));
    assert_mont_correct(&p_minus_1, &p_minus_1);
    assert_mont_correct(&p_minus_1, &p_minus_2);
    assert_mont_correct(&p_minus_2, &p_minus_2);
}

#[test]
fn mont_random_fixtures() {
    // Hardcoded "random-looking" fixtures — deterministic test inputs
    // that exercise different bit patterns.
    let fixtures: &[(&str, &str)] = &[
        (
            "0000000000000000000000000000000000000000000000000000000000000001",
            "ffffffff00000001000000000000000000000000fffffffffffffffffffffffe",
        ),
        (
            "dead000000000000000000000000000000000000000000000000000000000000",
            "00000000000000000000000000000000000000000000000000000000000000ad",
        ),
        (
            "123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0",
            "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210",
        ),
        (
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "5555555555555555555555555555555555555555555555555555555555555555",
        ),
        (
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ),
    ];
    for (a_hex, b_hex) in fixtures {
        let a = hex_to_big(a_hex);
        let b = hex_to_big(b_hex);
        // Reduce inputs mod p to ensure they're in valid Montgomery range.
        let p = p256_p_local();
        let a_r = a.modulo(&p);
        let b_r = b.modulo(&p);
        assert_mont_correct(&a_r, &b_r);
    }
}

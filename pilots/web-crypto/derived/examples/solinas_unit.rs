// WC-EXT 24: cross-check v3 (BigUInt-based) against canonical.
use rusty_web_crypto::*;

fn p256_p_local() -> BigUInt {
    BigUInt::from_be_bytes(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ])
}

fn main() {
    let p = p256_p_local();
    let mut state: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let mut next = || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        state
    };
    let mut div_v2 = 0;
    let mut div_v3 = 0;
    for _ in 0..2000 {
        let mut bytes = [0u8; 32];
        for j in 0..32 {
            bytes[j] = (next() & 0xFF) as u8;
        }
        let a = BigUInt::from_be_bytes(&bytes).modulo(&p);
        for j in 0..32 {
            bytes[j] = (next() & 0xFF) as u8;
        }
        let b = BigUInt::from_be_bytes(&bytes).modulo(&p);
        let canonical = a.mul(&b).modulo(&p);
        let v2 = p256_mod_mul_solinas_v2(&a, &b);
        let v3 = p256_mod_mul_solinas_v3(&a, &b);
        if canonical.to_be_bytes(32) != v2.to_be_bytes(32) {
            div_v2 += 1;
        }
        if canonical.to_be_bytes(32) != v3.to_be_bytes(32) {
            div_v3 += 1;
        }
    }
    println!(
        "fuzz: v2 diverged {}/2000; v3 diverged {}/2000",
        div_v2, div_v3
    );
}

use rusty_web_crypto::*;
fn p384_p_local() -> BigUInt {
    BigUInt::from_be_bytes(
        &(0..48)
            .map(|i| {
                if i < 3 {
                    0xff
                } else if i == 3 {
                    0xff
                } else if i == 36 {
                    0xfe
                } else if i >= 37 && i < 40 {
                    0xff
                } else if i >= 40 && i < 44 {
                    0x00
                } else {
                    0xff
                }
            })
            .collect::<Vec<u8>>(),
    )
}
fn main() {
    // Use the real curve_p384().p
    let p = curve_p384().p;
    let mut state: u64 = 0xCAFE_BABE_DEAD_BEEF;
    let mut next = || {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        state
    };
    let mut div = 0;
    for _ in 0..2000 {
        let mut bytes = [0u8; 48];
        for j in 0..48 {
            bytes[j] = (next() & 0xFF) as u8;
        }
        let a = BigUInt::from_be_bytes(&bytes).modulo(&p);
        for j in 0..48 {
            bytes[j] = (next() & 0xFF) as u8;
        }
        let b = BigUInt::from_be_bytes(&bytes).modulo(&p);
        let canonical = a.mul(&b).modulo(&p);
        let via_sol = p384_mod_mul_solinas(&a, &b);
        if canonical.to_be_bytes(48) != via_sol.to_be_bytes(48) {
            div += 1;
        }
    }
    println!("P-384 Solinas fuzz: {}/2000 divergent", div);
}

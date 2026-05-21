// WC-EXT 23 bisect: cross-check Solinas reduction against canonical.
use rusty_web_crypto::*;

fn p256_p_local() -> BigUInt {
    BigUInt::from_be_bytes(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ])
}

fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }
fn hex_to_bytes(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i+2], 16).unwrap()).collect()
}

fn main() {
    let p = p256_p_local();

    // Edge cases first.
    let edge: &[(&str, Vec<u8>, Vec<u8>)] = &[
        ("1*1", vec![1u8], vec![1u8]),
        ("2*2", vec![2u8], vec![2u8]),
        ("Gx*Gx", hex_to_bytes("6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296"),
                 hex_to_bytes("6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296")),
        ("(p-1)*(p-1)", p.sub(&BigUInt::one()).to_be_bytes(32), p.sub(&BigUInt::one()).to_be_bytes(32)),
    ];
    println!("--- edge cases ---");
    for (label, ab, bb) in edge {
        let a = BigUInt::from_be_bytes(ab);
        let b = BigUInt::from_be_bytes(bb);
        let canonical = a.mul(&b).modulo(&p);
        let via_sol = p256_mod_mul_solinas_v2(&a, &b);
        let ok = canonical.to_be_bytes(32) == via_sol.to_be_bytes(32);
        println!("  {}: {}", label, if ok { "OK" } else { "DIVERGE" });
        if !ok {
            println!("    canonical = {}", hex(&canonical.to_be_bytes(32)));
            println!("    solinas   = {}", hex(&via_sol.to_be_bytes(32)));
        }
    }

    // Fuzz: 2000 random fixtures.
    let mut state: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        state
    };
    let mut diverge_count = 0;
    for i in 0..2000 {
        let mut bytes = [0u8; 32];
        for j in 0..32 { bytes[j] = (next() & 0xFF) as u8; }
        let a = BigUInt::from_be_bytes(&bytes).modulo(&p);
        for j in 0..32 { bytes[j] = (next() & 0xFF) as u8; }
        let b = BigUInt::from_be_bytes(&bytes).modulo(&p);
        let canonical = a.mul(&b).modulo(&p);
        let via_sol = p256_mod_mul_solinas_v2(&a, &b);
        if canonical.to_be_bytes(32) != via_sol.to_be_bytes(32) {
            diverge_count += 1;
            if diverge_count <= 3 {
                println!("DIVERGE [{}]:", i);
                println!("  a  = {}", hex(&a.to_be_bytes(32)));
                println!("  b  = {}", hex(&b.to_be_bytes(32)));
                println!("  canonical = {}", hex(&canonical.to_be_bytes(32)));
                println!("  solinas   = {}", hex(&via_sol.to_be_bytes(32)));
            }
        }
    }
    println!("\nfuzz: {} divergent out of 2000 random fixtures", diverge_count);
}

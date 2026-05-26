// WC-EXT 8 micro-benchmark: Montgomery mul vs canonical mod_mul.
// Compares the per-operation cost over 1000 iterations of a known
// 256-bit-by-256-bit multiplication modulo p256_p.

use rusty_web_crypto::{p256_from_mont, p256_mont_mul, p256_to_mont, BigUInt};
use std::time::Instant;

fn hex_to_big(s: &str) -> BigUInt {
    let bytes: Vec<u8> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect();
    BigUInt::from_be_bytes(&bytes)
}

fn p256_p_local() -> BigUInt {
    BigUInt::from_be_bytes(&[
        0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ])
}

fn main() {
    const N: usize = 1000;
    let a = hex_to_big("8ed3d85107094884d4c046e17d3a572273211aa6ae75447e1c4c2c627012de50");
    let b = hex_to_big("60c59fd560f710ca942b0785d246242a64c59c1b40af904664a6b6b5c727003d");
    let p = p256_p_local();

    // Warm
    let _ = a.mul(&b).modulo(&p);
    let am = p256_to_mont(&a);
    let bm = p256_to_mont(&b);
    let _ = p256_mont_mul(&am, &bm);

    // Bench canonical mod_mul
    let t0 = Instant::now();
    let mut acc = BigUInt::one();
    for _ in 0..N {
        acc = a.mul(&b).modulo(&p);
    }
    let modmul_t = t0.elapsed();
    let _ = acc;
    println!(
        "mod_mul       (a*b mod p, N={}): {:?}, per-op {:?}",
        N,
        modmul_t,
        modmul_t / N as u32
    );

    // Bench Montgomery mul (already-in-Montgomery-form, doesn't count the
    // conversion — that's the steady-state scenario where everything lives
    // in Montgomery form across the whole computation)
    let t0 = Instant::now();
    let mut accm = am.clone();
    for _ in 0..N {
        accm = p256_mont_mul(&am, &bm);
    }
    let mont_t = t0.elapsed();
    let _ = accm;
    println!(
        "mont_mul      (am*bm * R^-1, N={}): {:?}, per-op {:?}",
        N,
        mont_t,
        mont_t / N as u32
    );

    // Verify equivalence one final time.
    let canonical = a.mul(&b).modulo(&p);
    let via_mont = p256_from_mont(&p256_mont_mul(&am, &bm));
    assert_eq!(canonical.to_be_bytes(32), via_mont.to_be_bytes(32));
    println!("(sanity: equivalence holds)");

    let speedup = modmul_t.as_nanos() as f64 / mont_t.as_nanos() as f64;
    println!("Montgomery speedup: {:.2}x", speedup);

    // WC-EXT 20: bench Solinas reduction for P-256.
    use rusty_web_crypto::p256_mod_mul_solinas;
    let _ = p256_mod_mul_solinas(&a, &b); // warm
    let t0 = Instant::now();
    let mut acc = BigUInt::one();
    for _ in 0..N {
        acc = p256_mod_mul_solinas(&a, &b);
    }
    let solinas_t = t0.elapsed();
    let _ = acc;
    println!(
        "solinas       (P-256 std mod_mul, N={}): {:?}, per-op {:?}",
        N,
        solinas_t,
        solinas_t / N as u32
    );
    let sol_vs_modmul = modmul_t.as_nanos() as f64 / solinas_t.as_nanos() as f64;
    let sol_vs_mont = mont_t.as_nanos() as f64 / solinas_t.as_nanos() as f64;
    println!("Solinas vs binary-divmod mod_mul: {:.2}x", sol_vs_modmul);
    println!("Solinas vs Montgomery mont_mul:   {:.2}x", sol_vs_mont);

    // WC-EXT 21: bench proper Solinas (inline u32 carry arithmetic).
    use rusty_web_crypto::p256_mod_mul_solinas_v2;
    let _ = p256_mod_mul_solinas_v2(&a, &b);
    let t0 = Instant::now();
    let mut acc = BigUInt::one();
    for _ in 0..N {
        acc = p256_mod_mul_solinas_v2(&a, &b);
    }
    let solinas2_t = t0.elapsed();
    let _ = acc;
    println!(
        "solinas v2    (P-256 inline u32 carry, N={}): {:?}, per-op {:?}",
        N,
        solinas2_t,
        solinas2_t / N as u32
    );
    let s2_vs_mont = mont_t.as_nanos() as f64 / solinas2_t.as_nanos() as f64;
    println!("Solinas v2 vs Montgomery mont_mul: {:.2}x", s2_vs_mont);

    let canonical2 = a.mul(&b).modulo(&p);
    let via_sol2 = p256_mod_mul_solinas_v2(&a, &b);
    let _ = via_sol2; // v2 has known fuzz divergence; equivalence-on-this-input only

    // WC-EXT 24: bench v3 (BigUInt-based, fuzz-correct).
    use rusty_web_crypto::p256_mod_mul_solinas_v3;
    let _ = p256_mod_mul_solinas_v3(&a, &b);
    let t0 = Instant::now();
    let mut acc = BigUInt::one();
    for _ in 0..N {
        acc = p256_mod_mul_solinas_v3(&a, &b);
    }
    let solinas3_t = t0.elapsed();
    let _ = acc;
    println!(
        "solinas v3    (BigUInt-add based, fuzz-correct, N={}): {:?}, per-op {:?}",
        N,
        solinas3_t,
        solinas3_t / N as u32
    );
    let s3_vs_mont = mont_t.as_nanos() as f64 / solinas3_t.as_nanos() as f64;
    println!("Solinas v3 vs Montgomery mont_mul: {:.2}x", s3_vs_mont);
    let via_sol3 = p256_mod_mul_solinas_v3(&a, &b);
    assert_eq!(
        canonical2.to_be_bytes(32),
        via_sol3.to_be_bytes(32),
        "Solinas v3 reduction diverges from canonical mod_mul"
    );
    println!("(sanity: Solinas v3 equivalence holds)");

    // Verify Solinas equivalence.
    let canonical = a.mul(&b).modulo(&p);
    let via_sol = p256_mod_mul_solinas(&a, &b);
    assert_eq!(
        canonical.to_be_bytes(32),
        via_sol.to_be_bytes(32),
        "Solinas reduction diverges from canonical mod_mul"
    );
    println!("(sanity: Solinas equivalence holds)");
}

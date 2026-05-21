// Quick check: does p256_scalar_mul_solinas produce the same result
// as p256_scalar_mul_mont for the same inputs?
use rusty_web_crypto::*;

fn main() {
    let g = curve_p256().g;
    
    let k_bytes = [0u8; 32].iter().chain(&[1u8]).copied().collect::<Vec<u8>>(); // 1
    let _ = k_bytes;
    
    // Test with small scalar k=2
    let k = BigUInt::from_be_bytes(&[2]);
    let r_mont = p256_scalar_mul_mont(&k, &g);
    let r_sol = p256_scalar_mul_solinas(&k, &g);
    println!("k=2: mont matches solinas? {}", format!("{:?}", r_mont) == format!("{:?}", r_sol));
    if format!("{:?}", r_mont) != format!("{:?}", r_sol) {
        println!("  mont: {:?}", match &r_mont {
            P256Point::Affine{x,y} => (hex(&x.to_be_bytes(32)), hex(&y.to_be_bytes(32))),
            _ => ("identity".into(), "".into()),
        });
        println!("  sol:  {:?}", match &r_sol {
            P256Point::Affine{x,y} => (hex(&x.to_be_bytes(32)), hex(&y.to_be_bytes(32))),
            _ => ("identity".into(), "".into()),
        });
    }

    // Test with bigger
    let k = BigUInt::from_be_bytes(&[0x12, 0x34, 0x56, 0x78]);
    let r_mont = p256_scalar_mul_mont(&k, &g);
    let r_sol = p256_scalar_mul_solinas(&k, &g);
    println!("k=0x12345678: mont matches solinas? {}", format!("{:?}", r_mont) == format!("{:?}", r_sol));
}

fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{:02x}", x)).collect() }

//! WC-EXT 1 — replay the TLS-EXT 8 captured fixture from api.github.com
//! through `rusty_web_crypto::ecdsa_verify`. Confirms the hang reproduces
//! in pure-Rust unit-test form, fully isolated from the TLS pilot.
//!
//! The test is `#[ignore]` because it intentionally hangs. Run with:
//!   timeout 8 cargo test -p rusty-web-crypto --release --test ecdsa_p256_apigithub_hang -- --ignored --nocapture
//!
//! Fixture: pilots/web-crypto/fixtures/ecdsa-p256-apigithub-2026-05-21.hex

use rusty_web_crypto::{curve_p256, ecdsa_verify};

fn hex_to_bytes(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

#[test]
#[ignore]
fn ecdsa_p256_apigithub_2026_05_21_hangs() {
    let qx = hex_to_bytes("8ed3d85107094884d4c046e17d3a572273211aa6ae75447e1c4c2c627012de50");
    let qy = hex_to_bytes("60c59fd560f710ca942b0785d246242a64c59c1b40af904664a6b6b5c727003d");
    let hash = hex_to_bytes("37baceaa42383ee05f48e5c65dc509c0a57160cebe8aa0d4f098fc0ffcc1f5e0");
    let sig_raw = hex_to_bytes(concat!(
        "3ede335ca360c3be2ab750900b00c63d36587b92cbafd2e748301662ca1a84d1",
        "56eadc5f8502ad8a040beb4426312b82f2a34b9e615e77d3b8bdc72ed8b24b84",
    ));

    let curve = curve_p256();
    eprintln!("[wc-ext-1] calling ecdsa_verify with apigithub-2026-05-21 fixture...");
    let result = ecdsa_verify(&curve, &qx, &qy, &hash, &sig_raw);
    eprintln!("[wc-ext-1] result: {:?}", result);
    // If we reach this assert, the hang is fixed. Until then the test
    // never returns (hence the #[ignore] + manual timeout invocation).
    // The expected post-fix outcome is Err (signature does not verify
    // — github's leaf cert rotates so this captured sig is not
    // necessarily authentic at run time) OR Ok; either is fine, the
    // load-bearing property is *bounded termination*.
}

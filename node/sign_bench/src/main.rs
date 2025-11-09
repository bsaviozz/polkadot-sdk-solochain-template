use sp_core::sr25519;
use sp_core::Pair;
use std::time::Instant;

#[derive(Debug)]
struct ExtrinsicPayload<'a> {
    msg: &'a [u8],
    user: &'a str,
    nonce: u32,
}

impl<'a> ExtrinsicPayload<'a> {
    fn encode(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(self.msg);
        v.extend_from_slice(self.user.as_bytes());
        v.extend_from_slice(&self.nonce.to_le_bytes());
        v
    }
}

// Function to generate a pre-signed extrinsic
fn off_chain_sign(ext: &ExtrinsicPayload) -> sr25519::Signature {
    let pair = sr25519::Pair::from_string("//Alice", None).unwrap();
    pair.sign(&ext.encode())
}

// Main function to generate average signing time
fn main() {
    let extrinsics = vec![
        ExtrinsicPayload { msg: b"payload 1", user: "Alice", nonce: 0 },
        ExtrinsicPayload { msg: b"payload 2", user: "Alice", nonce: 1 },
        ExtrinsicPayload { msg: b"payload 3", user: "Alice", nonce: 2 },
    ];

    let iterations = 1000;

    for ext in extrinsics.iter() {
        let encoded = ext.encode();
        let start = Instant::now();
        for _ in 0..iterations {
            let _sig = sr25519::Pair::from_string("//Alice", None).unwrap().sign(&encoded);
        }
        let duration = start.elapsed();
        println!(
            "Extrinsic {:?} signed {} times in {:?} ({:?} per signature)",
            ext,
            iterations,
            duration,
            duration / iterations
        );

        // Generate a pre-signed signature for on-chain benchmark
        let multi_sig = off_chain_sign(ext);
        println!("Precomputed MultiSignature: {:?}", multi_sig);
    }
}

use std::{env, fs, path::PathBuf};

use sp_core::Pair as _;
use sp_core::{dilithium, ecdsa, sr25519};
use sp_core::hashing::blake2_256;
use sp_core::ByteArray;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR set by cargo"));

    // Deterministic message
    let msg: Vec<u8> = vec![2u8; 128];
    /*
    // ----------------
    // Dilithium
    // ----------------
    {
        let seed: [u8; 32] = *b"12345678901234567890123456789012";
        let pair = dilithium::Pair::from_seed(&seed);

        let public = pair.public();
        let signature = pair.sign(&msg);

        // Runtime rule you used: signer = blake2_256(public)
        let signer32 = blake2_256(public.as_ref());

        fs::write(out_dir.join("dil_msg.bin"), &msg).expect("write dil_msg");
        fs::write(out_dir.join("dil_pub.bin"), public.as_slice()).expect("write dil_pub");
        fs::write(out_dir.join("dil_sig.bin"), signature.as_slice()).expect("write dil_sig");
        fs::write(out_dir.join("dil_signer.bin"), signer32).expect("write dil_signer");
    }*/

    // ----------------
    // sr25519
    // ----------------
    {
        let seed: [u8; 32] = *b"sr25519-seed-0000000000000000000";
        let pair = sr25519::Pair::from_seed(&seed);

        let public = pair.public();       // 32 bytes
        let signature = pair.sign(&msg);  // 64 bytes

        // Runtime rule for MultiSignature::Sr25519: signer == public key bytes
        let signer32: [u8; 32] = <[u8; 32]>::from(public);

        fs::write(out_dir.join("sr_msg.bin"), &msg).expect("write sr_msg");
        fs::write(out_dir.join("sr_pub.bin"), public.as_slice()).expect("write sr_pub");
        fs::write(out_dir.join("sr_sig.bin"), signature.as_slice()).expect("write sr_sig");
        fs::write(out_dir.join("sr_signer.bin"), signer32).expect("write sr_signer");
    }

    // ----------------
    // ecdsa (secp256k1)
    // ----------------
    {
        let seed: [u8; 32] = [7u8; 32];
        let pair = ecdsa::Pair::from_seed(&seed);

        let public = pair.public(); // compressed 33 bytes

        // IMPORTANT: sign the original message; runtime will hash msg once during verify
        let signature = pair.sign(&msg);

        // AccountId32 derivation rule in your runtime: blake2_256(compressed_pubkey_33)
        let signer32 = blake2_256(public.as_ref());

        let pub33: &[u8; 33] = public.as_ref();
        let sig65: &[u8; 65] = signature.as_ref();

        fs::write(out_dir.join("ec_msg.bin"), &msg).expect("write ec_msg");
        fs::write(out_dir.join("ec_pub33.bin"), pub33).expect("write ec_pub33");
        fs::write(out_dir.join("ec_sig.bin"), sig65).expect("write ec_sig");
        fs::write(out_dir.join("ec_signer.bin"), signer32).expect("write ec_signer");
    }
}

use std::{env, fs, path::PathBuf};

use sp_core::Pair as _;
use sp_core::{ecdsa, sr25519};
use sp_core::hashing::blake2_256;
use sp_core::ByteArray;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR set by cargo"));

    // Deterministic message (keep constant across algos)
    let msg: Vec<u8> = vec![2u8; 128];

    // ----------------
    // sr25519
    // ----------------
    {
        let seed: [u8; 32] = *b"sr25519-seed-0000000000000000000";
        let pair = sr25519::Pair::from_seed(&seed);

        let public = pair.public();       // 32 bytes
        let signature = pair.sign(&msg);  // 64 bytes

        // MultiSigner::Sr25519 -> AccountId32 == public bytes
        let signer32: [u8; 32] = public.0;

        fs::write(out_dir.join("sr_msg.bin"), &msg).expect("write sr_msg");
        fs::write(out_dir.join("sr_sig.bin"), signature.as_ref() as &[u8]).expect("write sr_sig");
        fs::write(out_dir.join("sr_signer.bin"), signer32).expect("write sr_signer");
    }

    // ----------------
    // ecdsa (secp256k1)
    // ----------------
    {
        let seed: [u8; 32] = [7u8; 32];
        let pair = ecdsa::Pair::from_seed(&seed);

        let public = pair.public(); // compressed 33 bytes
        let signature = pair.sign(&msg); // 65 bytes

        // MultiSigner::Ecdsa -> AccountId32 == blake2_256(compressed_pubkey_33)
        let signer32 = blake2_256(public.as_ref());

        fs::write(out_dir.join("ec_msg.bin"), &msg).expect("write ec_msg");
        fs::write(out_dir.join("ec_sig.bin"), signature.as_ref() as &[u8]).expect("write ec_sig");
        fs::write(out_dir.join("ec_signer.bin"), signer32).expect("write ec_signer");
    }

    // ----------------
    // Dilithium / ML-DSA-87
    // ----------------
    {
        use sp_core::dilithium;
        let seed: [u8; 32] = *b"12345678901234567890123456789012";
        let pair = dilithium::Pair::from_seed(&seed);

        let public = pair.public();
        let signature = pair.sign(&msg);

        let signer32 = blake2_256(public.as_ref());

        fs::write(out_dir.join("dil_msg.bin"), &msg).expect("write dil_msg");
        fs::write(out_dir.join("dil_pub.bin"), public.as_slice()).expect("write dil_pub");
        fs::write(out_dir.join("dil_sig.bin"), signature.as_slice()).expect("write dil_sig");
        fs::write(out_dir.join("dil_signer.bin"), signer32).expect("write dil_signer");
    }
    /*
    // ----------------
    // Dilithium / ML-DSA-44
    // ----------------
    {
        use qp_rusty_crystals_dilithium::ml_dsa_44;

        let entropy: [u8; 32] = *b"12345678901234567890123456789012";
        let kp = ml_dsa_44::Keypair::generate(Some(&entropy));

        let pk_bytes: [u8; ml_dsa_44::PUBLICKEYBYTES] = kp.public.to_bytes();

        let sig_bytes: [u8; ml_dsa_44::SIGNBYTES] =
            kp.sign(&msg, None, false).expect("ml_dsa_44 sign returned None");

        let signer32 = blake2_256(&pk_bytes);

        fs::write(out_dir.join("dil_msg.bin"), &msg).expect("write dil_msg");
        fs::write(out_dir.join("dil_pub.bin"), &pk_bytes).expect("write dil_pub");
        fs::write(out_dir.join("dil_sig.bin"), &sig_bytes).expect("write dil_sig");
        fs::write(out_dir.join("dil_signer.bin"), signer32).expect("write dil_signer");
    }

    // ----------------
    // Dilithium / ML-DSA-65
    // ----------------
    {
         use qp_rusty_crystals_dilithium::ml_dsa_65;

        let entropy: [u8; 32] = *b"12345678901234567890123456789012";
        let kp = ml_dsa_65::Keypair::generate(Some(&entropy));

        let pk_bytes: [u8; ml_dsa_65::PUBLICKEYBYTES] = kp.public.to_bytes();

        let sig_bytes: [u8; ml_dsa_65::SIGNBYTES] =
            kp.sign(&msg, None, false).expect("ml_dsa_65 sign returned None");

        let signer32 = blake2_256(&pk_bytes);

        fs::write(out_dir.join("dil_msg.bin"), &msg).expect("write dil_msg");
        fs::write(out_dir.join("dil_pub.bin"), &pk_bytes).expect("write dil_pub");
        fs::write(out_dir.join("dil_sig.bin"), &sig_bytes).expect("write dil_sig");
        fs::write(out_dir.join("dil_signer.bin"), signer32).expect("write dil_signer");
    }*/
}
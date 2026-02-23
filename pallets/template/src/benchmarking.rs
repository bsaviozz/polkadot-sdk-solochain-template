//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;
	use sp_core::ByteArray;
	use sp_runtime::{AccountId32, MultiSignature};
	use codec::Encode;

	#[benchmark]
	fn do_something() {
		let value = 100u32;
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		do_something(RawOrigin::Signed(caller), value);

		assert_eq!(Something::<T>::get(), Some(value));
	}

	#[benchmark]
	fn cause_error() {
		Something::<T>::put(100u32);
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));

		assert_eq!(Something::<T>::get(), Some(101u32));
	}

	const N: u32 = 100; // use 100 or 500 for stable per-verify cost

    #[benchmark]
    fn verify_sr25519() {
		use sp_core::sr25519;

        const MSG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sr_msg.bin"));
        const SIG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sr_sig.bin"));
        const SIGNER_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sr_signer.bin"));

        let signature = sr25519::Signature::from_slice(SIG).expect("sr_sig.bin must be 64 bytes");
        let signer_arr: [u8; 32] = SIGNER_BYTES.try_into().expect("sr_signer.bin must be 32 bytes");
        let signer = AccountId32::from(signer_arr);

        let ms = MultiSignature::Sr25519(signature).encode();

        #[extrinsic_call]
        verify_sr25519(RawOrigin::None, N, signer, MSG.to_vec(), ms);
    }

    #[benchmark]
    fn verify_ecdsa() {
        use sp_core::ecdsa;

        const MSG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ec_msg.bin"));
        const SIG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ec_sig.bin"));
        const SIGNER_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ec_signer.bin"));

        let signature = ecdsa::Signature::from_slice(SIG).expect("ec_sig.bin must be 65 bytes");
        let signer_arr: [u8; 32] = SIGNER_BYTES.try_into().expect("ec_signer.bin must be 32 bytes");
        let signer = AccountId32::from(signer_arr);

        let ms = MultiSignature::Ecdsa(signature).encode();

        #[extrinsic_call]
        verify_ecdsa(RawOrigin::None, N, signer, MSG.to_vec(), ms);
    }

    #[benchmark]
    fn verify_dilithium() {
        use sp_core::dilithium;
        use sp_core::hashing::blake2_256;
        use sp_runtime::DilithiumMultiSig;

        const MSG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dil_msg.bin"));
        const PUB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dil_pub.bin"));
        const SIG: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dil_sig.bin"));
        const SIGNER_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dil_signer.bin"));

        let public = dilithium::Public::from_slice(PUB).expect("PUB len must match");
        let signature = dilithium::Signature::from_slice(SIG).expect("SIG len must match");

        let signer_arr: [u8; 32] = SIGNER_BYTES.try_into().expect("dil_signer.bin must be 32 bytes");
        let signer = AccountId32::from(signer_arr);

        // sanity check signer derivation rule
        let derived: AccountId32 = blake2_256(public.as_ref()).into();
        assert_eq!(derived, signer);

        let dil_ms = DilithiumMultiSig { signature, public };
        let ms = MultiSignature::Dilithium(dil_ms).encode();

        #[extrinsic_call]
        verify_dilithium(RawOrigin::None, N, signer, MSG.to_vec(), ms);
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

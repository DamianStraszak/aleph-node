
//! Autogenerated weights for `pallettants::RocksDbWeight}babytants::RocksDbWeight}liminal`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-26, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ramanujan`, CPU: `12th Gen Intel(R) Core(TM) i7-1255U`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("chainspec.json"), DB CACHE: 1024

// Executed Command:
// ./target/production/aleph-node
// benchmark
// pallet
// --chain
// chainspec.json
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_baby_liminal
// --extrinsic=*
// --steps
// 20
// --repeat
// 50
// --output=pallets/baby-liminal/src/weigths.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_baby_liminal.
pub trait WeightInfo {
	fn store_key(key_length: u32) -> Weight;
	fn overwrite_key(key_length: u32) -> Weight;
	fn verify_groth16() -> Weight;
	fn verify_gm17() -> Weight;
	fn verify_marlin() -> Weight;
	fn verify_data_too_long(excess: u32) -> Weight;
	fn verify_data_deserializing_fails(data_length: u32) -> Weight;
	fn verify_key_deserializing_fails(key_length: u32) -> Weight;
	fn poseidon_one_to_one_wasm() -> Weight;
	fn poseidon_two_to_one_wasm() -> Weight;
	fn poseidon_four_to_one_wasm() -> Weight;
	fn poseidon_one_to_one_host() -> Weight;
	fn poseidon_two_to_one_host() -> Weight;
	fn poseidon_four_to_one_host() -> Weight;
}

impl<I: BenchmarkInfo> WeightInfo for I {
	fn store_key(key_length: u32) -> Weight {
		<I as BenchmarkInfo>::store_key(key_length)
	}

	fn overwrite_key(key_length: u32) -> Weight {
		<I as BenchmarkInfo>::overwrite_key(key_length)
	}

	fn verify_groth16() -> Weight {
		<I as BenchmarkInfo>::verify_groth16_xor()
			.max(<I as BenchmarkInfo>::verify_groth16_linear_equation())
			.max(<I as BenchmarkInfo>::verify_groth16_merkle_tree_8())
			.max(<I as BenchmarkInfo>::verify_groth16_merkle_tree_64())
			.max(<I as BenchmarkInfo>::verify_groth16_merkle_tree_1024())
	}

	fn verify_gm17() -> Weight {
		<I as BenchmarkInfo>::verify_gm17_xor()
			.max(<I as BenchmarkInfo>::verify_gm17_linear_equation())
			.max(<I as BenchmarkInfo>::verify_gm17_merkle_tree_8())
			.max(<I as BenchmarkInfo>::verify_gm17_merkle_tree_64())
			.max(<I as BenchmarkInfo>::verify_gm17_merkle_tree_1024())
	}

	fn verify_marlin() -> Weight {
		<I as BenchmarkInfo>::verify_marlin_xor()
			.max(<I as BenchmarkInfo>::verify_marlin_linear_equation())
			.max(<I as BenchmarkInfo>::verify_marlin_merkle_tree_8())
			.max(<I as BenchmarkInfo>::verify_marlin_merkle_tree_64())
			.max(<I as BenchmarkInfo>::verify_marlin_merkle_tree_1024())
	}

	fn verify_data_too_long(excess: u32) -> Weight {
		<I as BenchmarkInfo>::verify_data_too_long(excess)
	}

	fn verify_data_deserializing_fails(data_length: u32) -> Weight {
		<I as BenchmarkInfo>::verify_data_deserializing_fails(data_length)
	}

	fn verify_key_deserializing_fails(key_length: u32) -> Weight {
		<I as BenchmarkInfo>::verify_key_deserializing_fails(key_length)
	}

	fn poseidon_one_to_one_wasm() -> Weight {
		<I as BenchmarkInfo>::poseidon_one_to_one_wasm()
	}

	fn poseidon_two_to_one_wasm() -> Weight {
		<I as BenchmarkInfo>::poseidon_two_to_one_wasm()
	}

	fn poseidon_four_to_one_wasm() -> Weight {
		<I as BenchmarkInfo>::poseidon_four_to_one_wasm()
	}

	fn poseidon_one_to_one_host() -> Weight {
		<I as BenchmarkInfo>::poseidon_one_to_one_host()
	}

	fn poseidon_two_to_one_host() -> Weight {
		<I as BenchmarkInfo>::poseidon_two_to_one_host()
	}

	fn poseidon_four_to_one_host() -> Weight {
		<I as BenchmarkInfo>::poseidon_four_to_one_host()
	}
}

/// Benchmark results for pallet_baby_liminal.
trait BenchmarkInfo {
	fn store_key(l: u32, ) -> Weight;
	fn overwrite_key(l: u32, ) -> Weight;
	fn verify_groth16_xor() -> Weight;
	fn verify_groth16_linear_equation() -> Weight;
	fn verify_groth16_merkle_tree_8() -> Weight;
	fn verify_groth16_merkle_tree_64() -> Weight;
	fn verify_groth16_merkle_tree_1024() -> Weight;
	fn verify_gm17_xor() -> Weight;
	fn verify_gm17_linear_equation() -> Weight;
	fn verify_gm17_merkle_tree_8() -> Weight;
	fn verify_gm17_merkle_tree_64() -> Weight;
	fn verify_gm17_merkle_tree_1024() -> Weight;
	fn verify_marlin_xor() -> Weight;
	fn verify_marlin_linear_equation() -> Weight;
	fn verify_marlin_merkle_tree_8() -> Weight;
	fn verify_marlin_merkle_tree_64() -> Weight;
	fn verify_marlin_merkle_tree_1024() -> Weight;
	fn verify_data_too_long(e: u32, ) -> Weight;
	fn verify_data_deserializing_fails(l: u32, ) -> Weight;
	fn verify_key_deserializing_fails(l: u32, ) -> Weight;
	fn poseidon_one_to_one_wasm() -> Weight;
	fn poseidon_two_to_one_wasm() -> Weight;
	fn poseidon_four_to_one_wasm() -> Weight;
	fn poseidon_one_to_one_host() -> Weight;
	fn poseidon_two_to_one_host() -> Weight;
	fn poseidon_four_to_one_host() -> Weight;
}

/// Weight functions for `pallet_baby_liminal`.
pub struct AlephWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> BenchmarkInfo for AlephWeight<T> {
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	/// The range of component `l` is `[1, 10000]`.
	fn store_key(l: u32, ) -> Weight {
		// Minimum execution time: 13_650 nanoseconds.
		Weight::from_ref_time(16_410_454_u64)
			// Standard Error: 32
			.saturating_add(Weight::from_ref_time(386_u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	/// The range of component `l` is `[1, 10000]`.
	fn overwrite_key(l: u32, ) -> Weight {
		// Minimum execution time: 12_518 nanoseconds.
		Weight::from_ref_time(13_732_622_u64)
			// Standard Error: 12
			.saturating_add(Weight::from_ref_time(661_u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_xor() -> Weight {
		// Minimum execution time: 43_306_861 nanoseconds.
		Weight::from_ref_time(44_376_062_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_linear_equation() -> Weight {
		// Minimum execution time: 33_945_198 nanoseconds.
		Weight::from_ref_time(35_818_945_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_merkle_tree_8() -> Weight {
		// Minimum execution time: 45_464_791 nanoseconds.
		Weight::from_ref_time(54_101_966_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_merkle_tree_64() -> Weight {
		// Minimum execution time: 46_770_237 nanoseconds.
		Weight::from_ref_time(55_113_514_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_merkle_tree_1024() -> Weight {
		// Minimum execution time: 47_533_967 nanoseconds.
		Weight::from_ref_time(56_068_681_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_xor() -> Weight {
		// Minimum execution time: 49_140_894 nanoseconds.
		Weight::from_ref_time(58_228_515_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_linear_equation() -> Weight {
		// Minimum execution time: 41_032_856 nanoseconds.
		Weight::from_ref_time(45_451_116_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_merkle_tree_8() -> Weight {
		// Minimum execution time: 50_517_683 nanoseconds.
		Weight::from_ref_time(56_571_405_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_merkle_tree_64() -> Weight {
		// Minimum execution time: 53_557_585 nanoseconds.
		Weight::from_ref_time(59_065_540_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_merkle_tree_1024() -> Weight {
		// Minimum execution time: 51_797_543 nanoseconds.
		Weight::from_ref_time(59_398_939_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_xor() -> Weight {
		// Minimum execution time: 80_300_561 nanoseconds.
		Weight::from_ref_time(89_666_715_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_linear_equation() -> Weight {
		// Minimum execution time: 83_064_889 nanoseconds.
		Weight::from_ref_time(91_133_867_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_merkle_tree_8() -> Weight {
		// Minimum execution time: 78_568_810 nanoseconds.
		Weight::from_ref_time(85_744_686_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_merkle_tree_64() -> Weight {
		// Minimum execution time: 77_075_597 nanoseconds.
		Weight::from_ref_time(82_604_719_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_merkle_tree_1024() -> Weight {
		// Minimum execution time: 77_758_938 nanoseconds.
		Weight::from_ref_time(82_499_717_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// The range of component `e` is `[1, 10000000]`.
	fn verify_data_too_long(_e: u32, ) -> Weight {
		// Minimum execution time: 2_932 nanoseconds.
		Weight::from_ref_time(3_489_066_u64)
	}
	/// The range of component `l` is `[1, 10000]`.
	fn verify_data_deserializing_fails(l: u32, ) -> Weight {
		// Minimum execution time: 5_458 nanoseconds.
		Weight::from_ref_time(16_229_053_u64)
			// Standard Error: 82
			.saturating_add(Weight::from_ref_time(764_u64).saturating_mul(l as u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	/// The range of component `l` is `[1, 10000]`.
	fn verify_key_deserializing_fails(_l: u32, ) -> Weight {
		// Minimum execution time: 5_670_195 nanoseconds.
		Weight::from_ref_time(6_706_839_489_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_wasm() -> Weight {
		// Minimum execution time: 6_275_221 nanoseconds.
		Weight::from_ref_time(6_821_819_756_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_wasm() -> Weight {
		// Minimum execution time: 10_092_627 nanoseconds.
		Weight::from_ref_time(9_243_181_507_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_wasm() -> Weight {
		// Minimum execution time: 20_195_827 nanoseconds.
		Weight::from_ref_time(21_752_966_199_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_host() -> Weight {
		// Minimum execution time: 899_894 nanoseconds.
		Weight::from_ref_time(1_063_158_980_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_host() -> Weight {
		// Minimum execution time: 1_442_250 nanoseconds.
		Weight::from_ref_time(1_533_174_418_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_host() -> Weight {
		// Minimum execution time: 2_689_428 nanoseconds.
		Weight::from_ref_time(3_376_808_137_u64)
	}
}

// For backwards compatibility and tests
impl BenchmarkInfo for () {
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	/// The range of component `l` is `[1, 10000]`.
	fn store_key(l: u32, ) -> Weight {
		// Minimum execution time: 13_650 nanoseconds.
		Weight::from_ref_time(16_410_454_u64)
			// Standard Error: 32
			.saturating_add(Weight::from_ref_time(386_u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	/// The range of component `l` is `[1, 10000]`.
	fn overwrite_key(l: u32, ) -> Weight {
		// Minimum execution time: 12_518 nanoseconds.
		Weight::from_ref_time(13_732_622_u64)
			// Standard Error: 12
			.saturating_add(Weight::from_ref_time(661_u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_xor() -> Weight {
		// Minimum execution time: 43_306_861 nanoseconds.
		Weight::from_ref_time(44_376_062_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_linear_equation() -> Weight {
		// Minimum execution time: 33_945_198 nanoseconds.
		Weight::from_ref_time(35_818_945_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_merkle_tree_8() -> Weight {
		// Minimum execution time: 45_464_791 nanoseconds.
		Weight::from_ref_time(54_101_966_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_merkle_tree_64() -> Weight {
		// Minimum execution time: 46_770_237 nanoseconds.
		Weight::from_ref_time(55_113_514_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_groth16_merkle_tree_1024() -> Weight {
		// Minimum execution time: 47_533_967 nanoseconds.
		Weight::from_ref_time(56_068_681_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_xor() -> Weight {
		// Minimum execution time: 49_140_894 nanoseconds.
		Weight::from_ref_time(58_228_515_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_linear_equation() -> Weight {
		// Minimum execution time: 41_032_856 nanoseconds.
		Weight::from_ref_time(45_451_116_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_merkle_tree_8() -> Weight {
		// Minimum execution time: 50_517_683 nanoseconds.
		Weight::from_ref_time(56_571_405_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_merkle_tree_64() -> Weight {
		// Minimum execution time: 53_557_585 nanoseconds.
		Weight::from_ref_time(59_065_540_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_gm17_merkle_tree_1024() -> Weight {
		// Minimum execution time: 51_797_543 nanoseconds.
		Weight::from_ref_time(59_398_939_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_xor() -> Weight {
		// Minimum execution time: 80_300_561 nanoseconds.
		Weight::from_ref_time(89_666_715_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_linear_equation() -> Weight {
		// Minimum execution time: 83_064_889 nanoseconds.
		Weight::from_ref_time(91_133_867_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_merkle_tree_8() -> Weight {
		// Minimum execution time: 78_568_810 nanoseconds.
		Weight::from_ref_time(85_744_686_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_merkle_tree_64() -> Weight {
		// Minimum execution time: 77_075_597 nanoseconds.
		Weight::from_ref_time(82_604_719_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Storage: System ParentHash (r:1 w:0)
	fn verify_marlin_merkle_tree_1024() -> Weight {
		// Minimum execution time: 77_758_938 nanoseconds.
		Weight::from_ref_time(82_499_717_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// The range of component `e` is `[1, 10000000]`.
	fn verify_data_too_long(_e: u32, ) -> Weight {
		// Minimum execution time: 2_932 nanoseconds.
		Weight::from_ref_time(3_489_066_u64)
	}
	/// The range of component `l` is `[1, 10000]`.
	fn verify_data_deserializing_fails(l: u32, ) -> Weight {
		// Minimum execution time: 5_458 nanoseconds.
		Weight::from_ref_time(16_229_053_u64)
			// Standard Error: 82
			.saturating_add(Weight::from_ref_time(764_u64).saturating_mul(l as u64))
	}
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	/// The range of component `l` is `[1, 10000]`.
	fn verify_key_deserializing_fails(_l: u32, ) -> Weight {
		// Minimum execution time: 5_670_195 nanoseconds.
		Weight::from_ref_time(6_706_839_489_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_wasm() -> Weight {
		// Minimum execution time: 6_275_221 nanoseconds.
		Weight::from_ref_time(6_821_819_756_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_wasm() -> Weight {
		// Minimum execution time: 10_092_627 nanoseconds.
		Weight::from_ref_time(9_243_181_507_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_wasm() -> Weight {
		// Minimum execution time: 20_195_827 nanoseconds.
		Weight::from_ref_time(21_752_966_199_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_host() -> Weight {
		// Minimum execution time: 899_894 nanoseconds.
		Weight::from_ref_time(1_063_158_980_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_host() -> Weight {
		// Minimum execution time: 1_442_250 nanoseconds.
		Weight::from_ref_time(1_533_174_418_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_host() -> Weight {
		// Minimum execution time: 2_689_428 nanoseconds.
		Weight::from_ref_time(3_376_808_137_u64)
	}
}
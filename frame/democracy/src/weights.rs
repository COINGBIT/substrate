
//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-17, STEPS: `2`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --chain=dev
// --steps=2
// --repeat=1
// --pallet=pallet-democracy
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/democracy/src/._weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_democracy.
pub trait WeightInfo {
	fn propose() -> Weight;
	fn second() -> Weight;
	fn vote_new() -> Weight;
	fn vote_existing() -> Weight;
	fn emergency_cancel() -> Weight;
	fn blacklist() -> Weight;
	fn external_propose() -> Weight;
	fn external_propose_majority() -> Weight;
	fn external_propose_default() -> Weight;
	fn fast_track() -> Weight;
	fn veto_external() -> Weight;
	fn cancel_proposal() -> Weight;
	fn cancel_referendum() -> Weight;
	fn on_initialize_base(r: u32, ) -> Weight;
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight;
	fn delegate(r: u32, ) -> Weight;
	fn undelegate(r: u32, ) -> Weight;
	fn clear_public_proposals() -> Weight;
	fn unlock_remove(r: u32, ) -> Weight;
	fn unlock_set(r: u32, ) -> Weight;
	fn remove_vote(r: u32, ) -> Weight;
	fn remove_other_vote(r: u32, ) -> Weight;
	fn set_external_metadata() -> Weight;
	fn clear_external_metadata() -> Weight;
	fn set_proposal_metadata() -> Weight;
	fn clear_proposal_metadata() -> Weight;
	fn set_referendum_metadata() -> Weight;
	fn clear_referendum_metadata() -> Weight;
}

/// Weights for pallet_democracy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new() -> Weight {
		// Minimum execution time: 44_000 nanoseconds.
		Weight::from_ref_time(44_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing() -> Weight {
		// Minimum execution time: 43_000 nanoseconds.
		Weight::from_ref_time(43_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn emergency_cancel() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(35_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Democracy MetadataOf (r:3 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	fn blacklist() -> Weight {
		// Minimum execution time: 81_000 nanoseconds.
		Weight::from_ref_time(81_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(18_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		// Minimum execution time: 5_000 nanoseconds.
		Weight::from_ref_time(5_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:2)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(26_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn veto_external() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(35_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn cancel_proposal() -> Weight {
		// Minimum execution time: 65_000 nanoseconds.
		Weight::from_ref_time(65_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(_r: u32, ) -> Weight {
		// Minimum execution time: 7_000 nanoseconds.
		Weight::from_ref_time(302_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(101 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(_r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(239_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(104 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(_r: u32, ) -> Weight {
		// Minimum execution time: 40_000 nanoseconds.
		Weight::from_ref_time(375_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(103 as u64))
			.saturating_add(T::DbWeight::get().writes(103 as u64))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(_r: u32, ) -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(334_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(101 as u64))
			.saturating_add(T::DbWeight::get().writes(101 as u64))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		// Minimum execution time: 5_000 nanoseconds.
		Weight::from_ref_time(5_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(_r: u32, ) -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(_r: u32, ) -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(_r: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(21_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(_r: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(24_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Democracy MetadataOf (r:0 w:1)
	fn set_external_metadata() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(23_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn clear_external_metadata() -> Weight {
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_ref_time(27_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Democracy MetadataOf (r:0 w:1)
	fn set_proposal_metadata() -> Weight {
		// Minimum execution time: 42_000 nanoseconds.
		Weight::from_ref_time(42_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn clear_proposal_metadata() -> Weight {
		// Minimum execution time: 39_000 nanoseconds.
		Weight::from_ref_time(39_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Democracy MetadataOf (r:0 w:1)
	fn set_referendum_metadata() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(21_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn clear_referendum_metadata() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new() -> Weight {
		// Minimum execution time: 44_000 nanoseconds.
		Weight::from_ref_time(44_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing() -> Weight {
		// Minimum execution time: 43_000 nanoseconds.
		Weight::from_ref_time(43_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn emergency_cancel() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(35_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Democracy MetadataOf (r:3 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	fn blacklist() -> Weight {
		// Minimum execution time: 81_000 nanoseconds.
		Weight::from_ref_time(81_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(18_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		// Minimum execution time: 6_000 nanoseconds.
		Weight::from_ref_time(6_000_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		// Minimum execution time: 5_000 nanoseconds.
		Weight::from_ref_time(5_000_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:2)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(26_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn veto_external() -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(35_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn cancel_proposal() -> Weight {
		// Minimum execution time: 65_000 nanoseconds.
		Weight::from_ref_time(65_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(29_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(_r: u32, ) -> Weight {
		// Minimum execution time: 7_000 nanoseconds.
		Weight::from_ref_time(302_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(101 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(_r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(239_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(104 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(_r: u32, ) -> Weight {
		// Minimum execution time: 40_000 nanoseconds.
		Weight::from_ref_time(375_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(103 as u64))
			.saturating_add(RocksDbWeight::get().writes(103 as u64))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(_r: u32, ) -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(334_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(101 as u64))
			.saturating_add(RocksDbWeight::get().writes(101 as u64))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		// Minimum execution time: 5_000 nanoseconds.
		Weight::from_ref_time(5_000_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(_r: u32, ) -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(31_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(_r: u32, ) -> Weight {
		// Minimum execution time: 29_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(_r: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(21_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(_r: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(24_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Democracy MetadataOf (r:0 w:1)
	fn set_external_metadata() -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(23_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn clear_external_metadata() -> Weight {
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_ref_time(27_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Democracy MetadataOf (r:0 w:1)
	fn set_proposal_metadata() -> Weight {
		// Minimum execution time: 42_000 nanoseconds.
		Weight::from_ref_time(42_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn clear_proposal_metadata() -> Weight {
		// Minimum execution time: 39_000 nanoseconds.
		Weight::from_ref_time(39_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Democracy MetadataOf (r:0 w:1)
	fn set_referendum_metadata() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(21_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	// Storage: Democracy MetadataOf (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn clear_referendum_metadata() -> Weight {
		// Minimum execution time: 33_000 nanoseconds.
		Weight::from_ref_time(33_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
}

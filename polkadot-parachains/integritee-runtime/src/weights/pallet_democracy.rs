
//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/integritee-collator
// benchmark
// pallet
// --chain=integritee-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_democracy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		Weight::from_ref_time(167_163_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second(s: u32, ) -> Weight {
		Weight::from_ref_time(94_940_000)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(447_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new(r: u32, ) -> Weight {
		Weight::from_ref_time(126_933_000)
			// Standard Error: 24_000
			.saturating_add(Weight::from_ref_time(603_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing(r: u32, ) -> Weight {
		Weight::from_ref_time(128_563_000)
			// Standard Error: 17_000
			.saturating_add(Weight::from_ref_time(438_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		Weight::from_ref_time(64_339_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn blacklist(p: u32, ) -> Weight {
		Weight::from_ref_time(185_505_000)
			// Standard Error: 28_000
			.saturating_add(Weight::from_ref_time(756_000).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose(v: u32, ) -> Weight {
		Weight::from_ref_time(38_254_000)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(71_000).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		Weight::from_ref_time(2_813_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		Weight::from_ref_time(2_829_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		Weight::from_ref_time(64_024_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	fn veto_external(v: u32, ) -> Weight {
		Weight::from_ref_time(72_042_000)
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(90_000).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn cancel_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(135_481_000)
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(925_000).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		Weight::from_ref_time(47_394_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_queued(r: u32, ) -> Weight {
		Weight::from_ref_time(80_817_000)
			// Standard Error: 34_000
			.saturating_add(Weight::from_ref_time(2_129_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	fn on_initialize_base(r: u32, ) -> Weight {
		Weight::from_ref_time(9_037_000)
			// Standard Error: 74_000
			.saturating_add(Weight::from_ref_time(19_325_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		Weight::from_ref_time(24_683_000)
			// Standard Error: 84_000
			.saturating_add(Weight::from_ref_time(19_941_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn delegate(r: u32, ) -> Weight {
		Weight::from_ref_time(108_173_000)
			// Standard Error: 99_000
			.saturating_add(Weight::from_ref_time(22_120_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes(Weight::from_ref_time(1).saturating_mul(r.into()).ref_time()))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	fn undelegate(r: u32, ) -> Weight {
		Weight::from_ref_time(47_067_000)
			// Standard Error: 67_000
			.saturating_add(Weight::from_ref_time(22_971_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes(Weight::from_ref_time(1).saturating_mul(r.into()).ref_time()))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		Weight::from_ref_time(3_184_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	fn note_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(88_601_000)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	fn note_imminent_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(65_383_000)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn reap_preimage(b: u32, ) -> Weight {
		Weight::from_ref_time(89_493_000)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(4_000).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_remove(r: u32, ) -> Weight {
		Weight::from_ref_time(71_802_000)
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(160_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_set(r: u32, ) -> Weight {
		Weight::from_ref_time(68_758_000)
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(388_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	fn remove_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(54_930_000)
			// Standard Error: 11_000
			.saturating_add(Weight::from_ref_time(308_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	fn remove_other_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(46_336_000)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(353_000).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

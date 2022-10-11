//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-solo-fresh"), DB CACHE: 128

// Executed Command:
// ./integritee-node
// benchmark
// --chain=integritee-solo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_scheduler.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_scheduler.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(8_183_000)
			// Standard Error: 36_000
			.saturating_add(Weight::from_ref_time(34_670_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(11_520_000)
			// Standard Error: 30_000
			.saturating_add(Weight::from_ref_time(26_386_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn on_initialize_periodic_resolved(s: u32) -> Weight {
		Weight::from_ref_time(8_222_000)
			// Standard Error: 33_000
			.saturating_add(Weight::from_ref_time(28_925_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn on_initialize_resolved(s: u32) -> Weight {
		Weight::from_ref_time(11_610_000)
			// Standard Error: 26_000
			.saturating_add(Weight::from_ref_time(23_857_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named_aborted(s: u32) -> Weight {
		Weight::from_ref_time(11_067_000)
			// Standard Error: 15_000
			.saturating_add(Weight::from_ref_time(11_728_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	fn on_initialize_aborted(s: u32) -> Weight {
		Weight::from_ref_time(13_045_000)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(6_378_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_named(s: u32) -> Weight {
		Weight::from_ref_time(13_496_000)
			// Standard Error: 27_000
			.saturating_add(Weight::from_ref_time(17_932_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	fn on_initialize_periodic(s: u32) -> Weight {
		Weight::from_ref_time(17_074_000)
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(11_982_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named(s: u32) -> Weight {
		Weight::from_ref_time(18_730_000)
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(9_909_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(17_844_000)
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(7_719_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule(s: u32) -> Weight {
		Weight::from_ref_time(54_318_000)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(180_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn cancel(s: u32) -> Weight {
		Weight::from_ref_time(50_614_000)
			// Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(1_809_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule_named(s: u32) -> Weight {
		Weight::from_ref_time(70_748_000)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(245_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_named(s: u32) -> Weight {
		Weight::from_ref_time(62_401_000)
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(1_887_000).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
<<<<<<< HEAD
//! DATE: 2022-04-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
=======
//! DATE: 2022-03-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_scheduler.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_named_resolved(s: u32, ) -> Weight {
		(0 as Weight)
<<<<<<< HEAD
			// Standard Error: 28_000
			.saturating_add((25_341_000 as Weight).saturating_mul(s as Weight))
=======
			// Standard Error: 29_000
			.saturating_add((24_825_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named_resolved(s: u32, ) -> Weight {
<<<<<<< HEAD
		(2_091_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((20_372_000 as Weight).saturating_mul(s as Weight))
=======
		(2_154_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((19_940_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn on_initialize_periodic_resolved(s: u32, ) -> Weight {
<<<<<<< HEAD
		(0 as Weight)
			// Standard Error: 34_000
			.saturating_add((23_032_000 as Weight).saturating_mul(s as Weight))
=======
		(33_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((22_619_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn on_initialize_resolved(s: u32, ) -> Weight {
<<<<<<< HEAD
		(1_701_000 as Weight)
			// Standard Error: 26_000
			.saturating_add((19_301_000 as Weight).saturating_mul(s as Weight))
=======
		(3_886_000 as Weight)
			// Standard Error: 24_000
			.saturating_add((19_025_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named_aborted(s: u32, ) -> Weight {
<<<<<<< HEAD
		(3_406_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((9_606_000 as Weight).saturating_mul(s as Weight))
=======
		(4_957_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((9_857_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Preimage PreimageFor (r:1 w:0)
	fn on_initialize_aborted(s: u32, ) -> Weight {
<<<<<<< HEAD
		(5_008_000 as Weight)
			// Standard Error: 24_000
			.saturating_add((7_520_000 as Weight).saturating_mul(s as Weight))
=======
		(5_284_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((7_969_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_periodic_named(s: u32, ) -> Weight {
<<<<<<< HEAD
		(5_811_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((15_673_000 as Weight).saturating_mul(s as Weight))
=======
		(6_648_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((15_659_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:2 w:2)
	fn on_initialize_periodic(s: u32, ) -> Weight {
<<<<<<< HEAD
		(7_913_000 as Weight)
			// Standard Error: 19_000
			.saturating_add((13_250_000 as Weight).saturating_mul(s as Weight))
=======
		(5_139_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((13_568_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn on_initialize_named(s: u32, ) -> Weight {
<<<<<<< HEAD
		(7_894_000 as Weight)
			// Standard Error: 21_000
			.saturating_add((11_042_000 as Weight).saturating_mul(s as Weight))
=======
		(9_825_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((11_117_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	fn on_initialize(s: u32, ) -> Weight {
<<<<<<< HEAD
		(8_628_000 as Weight)
			// Standard Error: 29_000
			.saturating_add((10_127_000 as Weight).saturating_mul(s as Weight))
=======
		(8_613_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((10_307_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule(s: u32, ) -> Weight {
<<<<<<< HEAD
		(15_902_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((44_000 as Weight).saturating_mul(s as Weight))
=======
		(15_504_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((40_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	fn cancel(s: u32, ) -> Weight {
<<<<<<< HEAD
		(14_565_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((2_263_000 as Weight).saturating_mul(s as Weight))
=======
		(14_421_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((2_409_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule_named(s: u32, ) -> Weight {
<<<<<<< HEAD
		(18_466_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((59_000 as Weight).saturating_mul(s as Weight))
=======
		(17_707_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((56_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_named(s: u32, ) -> Weight {
<<<<<<< HEAD
		(15_653_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((2_258_000 as Weight).saturating_mul(s as Weight))
=======
		(15_157_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((2_408_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

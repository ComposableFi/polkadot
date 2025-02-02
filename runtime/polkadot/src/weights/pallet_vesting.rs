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
//! Autogenerated weights for `pallet_vesting`
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
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_vesting.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_locked(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(24_024_000 as Weight)
			// Standard Error: 0
			.saturating_add((96_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((164_000 as Weight).saturating_mul(s as Weight))
=======
		(24_014_000 as Weight)
			// Standard Error: 0
			.saturating_add((94_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((186_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(24_075_000 as Weight)
			// Standard Error: 0
			.saturating_add((84_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((145_000 as Weight).saturating_mul(s as Weight))
=======
		(24_550_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((85_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((129_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(24_064_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((95_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 2_000
			.saturating_add((171_000 as Weight).saturating_mul(s as Weight))
=======
		(23_990_000 as Weight)
			// Standard Error: 0
			.saturating_add((92_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((183_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(24_235_000 as Weight)
			// Standard Error: 0
			.saturating_add((80_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((137_000 as Weight).saturating_mul(s as Weight))
=======
		(24_598_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((75_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 2_000
			.saturating_add((139_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(38_964_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((83_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 4_000
			.saturating_add((155_000 as Weight).saturating_mul(s as Weight))
=======
		(38_852_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((85_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((152_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(38_810_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((90_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 5_000
			.saturating_add((138_000 as Weight).saturating_mul(s as Weight))
=======
		(38_419_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((85_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((157_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(24_830_000 as Weight)
			// Standard Error: 0
			.saturating_add((91_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((177_000 as Weight).saturating_mul(s as Weight))
=======
		(25_159_000 as Weight)
			// Standard Error: 0
			.saturating_add((90_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((181_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
<<<<<<< HEAD
		(24_915_000 as Weight)
			// Standard Error: 0
			.saturating_add((89_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((171_000 as Weight).saturating_mul(s as Weight))
=======
		(24_798_000 as Weight)
			// Standard Error: 0
			.saturating_add((90_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 1_000
			.saturating_add((186_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

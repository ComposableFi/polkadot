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
//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
<<<<<<< HEAD
//! DATE: 2022-04-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
=======
//! DATE: 2022-03-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_balances
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/pallet_balances.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
<<<<<<< HEAD
		(33_412_000 as Weight)
=======
		(30_938_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive() -> Weight {
<<<<<<< HEAD
		(24_931_000 as Weight)
=======
		(23_670_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_creating() -> Weight {
<<<<<<< HEAD
		(15_682_000 as Weight)
=======
		(15_117_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_killing() -> Weight {
<<<<<<< HEAD
		(18_732_000 as Weight)
=======
		(17_395_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	fn force_transfer() -> Weight {
<<<<<<< HEAD
		(34_076_000 as Weight)
=======
		(31_672_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_all() -> Weight {
<<<<<<< HEAD
		(30_417_000 as Weight)
=======
		(28_884_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn force_unreserve() -> Weight {
<<<<<<< HEAD
		(14_368_000 as Weight)
=======
		(13_776_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

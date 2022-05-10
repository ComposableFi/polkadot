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
//! Autogenerated weights for `pallet_proxy`
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
// --pallet=pallet_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_proxy.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	fn proxy(p: u32, ) -> Weight {
<<<<<<< HEAD
		(13_457_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((78_000 as Weight).saturating_mul(p as Weight))
=======
		(13_146_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((85_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(28_559_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((207_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 1_000
			.saturating_add((61_000 as Weight).saturating_mul(p as Weight))
=======
		(27_526_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((226_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((52_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
<<<<<<< HEAD
		(19_628_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((202_000 as Weight).saturating_mul(a as Weight))
=======
		(18_908_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((219_000 as Weight).saturating_mul(a as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
<<<<<<< HEAD
	fn reject_announcement(a: u32, _p: u32, ) -> Weight {
		(19_532_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((204_000 as Weight).saturating_mul(a as Weight))
=======
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(18_977_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((217_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((4_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn announce(a: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(26_653_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((206_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((61_000 as Weight).saturating_mul(p as Weight))
=======
		(25_604_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((214_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 2_000
			.saturating_add((58_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn add_proxy(p: u32, ) -> Weight {
<<<<<<< HEAD
		(21_877_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((126_000 as Weight).saturating_mul(p as Weight))
=======
		(20_627_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((135_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxy(p: u32, ) -> Weight {
<<<<<<< HEAD
		(21_828_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((128_000 as Weight).saturating_mul(p as Weight))
=======
		(17_524_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((145_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn remove_proxies(p: u32, ) -> Weight {
<<<<<<< HEAD
		(17_805_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((83_000 as Weight).saturating_mul(p as Weight))
=======
		(17_427_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((86_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	fn anonymous(p: u32, ) -> Weight {
<<<<<<< HEAD
		(24_454_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((35_000 as Weight).saturating_mul(p as Weight))
=======
		(23_731_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((29_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	fn kill_anonymous(p: u32, ) -> Weight {
<<<<<<< HEAD
		(18_550_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((92_000 as Weight).saturating_mul(p as Weight))
=======
		(18_099_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((91_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

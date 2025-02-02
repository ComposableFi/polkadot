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
//! Autogenerated weights for `pallet_democracy`
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
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/pallet_democracy.rs

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
<<<<<<< HEAD
		(33_272_000 as Weight)
=======
		(31_921_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second(s: u32, ) -> Weight {
<<<<<<< HEAD
		(23_958_000 as Weight)
			// Standard Error: 0
			.saturating_add((89_000 as Weight).saturating_mul(s as Weight))
=======
		(23_089_000 as Weight)
			// Standard Error: 0
			.saturating_add((87_000 as Weight).saturating_mul(s as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new(r: u32, ) -> Weight {
<<<<<<< HEAD
		(31_103_000 as Weight)
=======
		(31_065_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			// Standard Error: 0
			.saturating_add((134_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing(r: u32, ) -> Weight {
<<<<<<< HEAD
		(31_179_000 as Weight)
			// Standard Error: 0
			.saturating_add((133_000 as Weight).saturating_mul(r as Weight))
=======
		(31_056_000 as Weight)
			// Standard Error: 0
			.saturating_add((134_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
<<<<<<< HEAD
		(13_872_000 as Weight)
=======
		(13_610_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn blacklist(p: u32, ) -> Weight {
<<<<<<< HEAD
		(47_747_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((208_000 as Weight).saturating_mul(p as Weight))
=======
		(45_768_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((207_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose(v: u32, ) -> Weight {
<<<<<<< HEAD
		(7_203_000 as Weight)
			// Standard Error: 0
			.saturating_add((32_000 as Weight).saturating_mul(v as Weight))
=======
		(6_929_000 as Weight)
			// Standard Error: 0
			.saturating_add((34_000 as Weight).saturating_mul(v as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
<<<<<<< HEAD
		(1_366_000 as Weight)
=======
		(1_295_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
<<<<<<< HEAD
		(1_372_000 as Weight)
=======
		(1_282_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
<<<<<<< HEAD
		(14_260_000 as Weight)
=======
		(14_171_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	fn veto_external(v: u32, ) -> Weight {
<<<<<<< HEAD
		(15_068_000 as Weight)
			// Standard Error: 0
			.saturating_add((62_000 as Weight).saturating_mul(v as Weight))
=======
		(14_568_000 as Weight)
			// Standard Error: 0
			.saturating_add((68_000 as Weight).saturating_mul(v as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn cancel_proposal(p: u32, ) -> Weight {
<<<<<<< HEAD
		(35_611_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((189_000 as Weight).saturating_mul(p as Weight))
=======
		(34_419_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((191_000 as Weight).saturating_mul(p as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
<<<<<<< HEAD
		(8_760_000 as Weight)
=======
		(8_700_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_queued(r: u32, ) -> Weight {
<<<<<<< HEAD
		(20_407_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((2_059_000 as Weight).saturating_mul(r as Weight))
=======
		(20_871_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((2_276_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	fn on_initialize_base(r: u32, ) -> Weight {
<<<<<<< HEAD
		(1_537_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_857_000 as Weight).saturating_mul(r as Weight))
=======
		(2_126_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_861_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
<<<<<<< HEAD
		(5_804_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_870_000 as Weight).saturating_mul(r as Weight))
=======
		(6_166_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((2_873_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn delegate(r: u32, ) -> Weight {
<<<<<<< HEAD
		(30_908_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((3_615_000 as Weight).saturating_mul(r as Weight))
=======
		(30_869_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((3_624_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	fn undelegate(r: u32, ) -> Weight {
<<<<<<< HEAD
		(15_020_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((3_582_000 as Weight).saturating_mul(r as Weight))
=======
		(14_486_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((3_601_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
<<<<<<< HEAD
		(1_129_000 as Weight)
=======
		(1_114_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	fn note_preimage(b: u32, ) -> Weight {
<<<<<<< HEAD
		(21_213_000 as Weight)
=======
		(20_608_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	fn note_imminent_preimage(b: u32, ) -> Weight {
<<<<<<< HEAD
		(13_948_000 as Weight)
=======
		(13_898_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy Preimages (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	fn reap_preimage(b: u32, ) -> Weight {
<<<<<<< HEAD
		(20_861_000 as Weight)
=======
		(20_680_000 as Weight)
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_remove(r: u32, ) -> Weight {
<<<<<<< HEAD
		(19_389_000 as Weight)
			// Standard Error: 0
			.saturating_add((30_000 as Weight).saturating_mul(r as Weight))
=======
		(19_441_000 as Weight)
			// Standard Error: 0
			.saturating_add((26_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlock_set(r: u32, ) -> Weight {
<<<<<<< HEAD
		(18_320_000 as Weight)
			// Standard Error: 0
			.saturating_add((123_000 as Weight).saturating_mul(r as Weight))
=======
		(18_365_000 as Weight)
			// Standard Error: 0
			.saturating_add((127_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	fn remove_vote(r: u32, ) -> Weight {
<<<<<<< HEAD
		(10_130_000 as Weight)
			// Standard Error: 0
			.saturating_add((115_000 as Weight).saturating_mul(r as Weight))
=======
		(10_244_000 as Weight)
			// Standard Error: 0
			.saturating_add((121_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	fn remove_other_vote(r: u32, ) -> Weight {
<<<<<<< HEAD
		(10_162_000 as Weight)
			// Standard Error: 0
			.saturating_add((116_000 as Weight).saturating_mul(r as Weight))
=======
		(10_367_000 as Weight)
			// Standard Error: 0
			.saturating_add((118_000 as Weight).saturating_mul(r as Weight))
>>>>>>> 9ed0c98204d25eaad8a6b40248daee8e6a40d111
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

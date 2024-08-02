// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./encointer-kusama-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=./encointer-kusama-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./encointer-kusama-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	fn reset_members(m: u32) -> Weight {
		Weight::from_parts(10_462_422, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 247
			.saturating_add(Weight::from_parts(27_500, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Membership::Members` (r:1 w:1)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Proposals` (r:1 w:0)
	/// Proof: `Collective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Collective::Members` (r:0 w:1)
	/// Proof: `Collective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Collective::Prime` (r:0 w:1)
	/// Proof: `Collective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `102 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 9_820_000 picoseconds.
		Weight::from_parts(10_462_422, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 247
			.saturating_add(Weight::from_parts(27_500, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Membership::Members` (r:1 w:1)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Proposals` (r:1 w:0)
	/// Proof: `Collective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Prime` (r:1 w:0)
	/// Proof: `Membership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Members` (r:0 w:1)
	/// Proof: `Collective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Collective::Prime` (r:0 w:1)
	/// Proof: `Collective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 11_612_000 picoseconds.
		Weight::from_parts(12_159_816, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 350
			.saturating_add(Weight::from_parts(25_767, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Membership::Members` (r:1 w:1)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Proposals` (r:1 w:0)
	/// Proof: `Collective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Prime` (r:1 w:0)
	/// Proof: `Membership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Members` (r:0 w:1)
	/// Proof: `Collective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Collective::Prime` (r:0 w:1)
	/// Proof: `Collective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 11_679_000 picoseconds.
		Weight::from_parts(12_274_587, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 360
			.saturating_add(Weight::from_parts(36_425, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Membership::Members` (r:1 w:1)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Proposals` (r:1 w:0)
	/// Proof: `Collective::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Membership::Prime` (r:1 w:1)
	/// Proof: `Membership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Members` (r:0 w:1)
	/// Proof: `Collective::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Collective::Prime` (r:0 w:1)
	/// Proof: `Collective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `206 + m * (64 ±0)`
		//  Estimated: `4687 + m * (64 ±0)`
		// Minimum execution time: 11_838_000 picoseconds.
		Weight::from_parts(12_806_896, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 406
			.saturating_add(Weight::from_parts(33_732, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `Membership::Prime` (r:0 w:1)
	/// Proof: `Membership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Prime` (r:0 w:1)
	/// Proof: `Collective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32 + m * (32 ±0)`
		//  Estimated: `4687 + m * (32 ±0)`
		// Minimum execution time: 4_967_000 picoseconds.
		Weight::from_parts(5_283_609, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 211
			.saturating_add(Weight::from_parts(12_548, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
	}
	/// Storage: `Membership::Prime` (r:0 w:1)
	/// Proof: `Membership::Prime` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Collective::Prime` (r:0 w:1)
	/// Proof: `Collective::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_797_000 picoseconds.
		Weight::from_parts(2_056_844, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

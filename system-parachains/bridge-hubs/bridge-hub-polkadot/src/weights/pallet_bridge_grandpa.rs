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

//! Autogenerated weights for `pallet_bridge_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./bridge-hub-polkadot-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=./bridge-hub-polkadot-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_grandpa
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./bridge-hub-polkadot-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_grandpa::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeKusamaGrandpa::CurrentAuthoritySet` (r:1 w:0)
	/// Proof: `BridgeKusamaGrandpa::CurrentAuthoritySet` (`max_values`: Some(1), `max_size`: Some(50250), added: 50745, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgeKusamaGrandpa::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::BestFinalized` (r:1 w:1)
	/// Proof: `BridgeKusamaGrandpa::BestFinalized` (`max_values`: Some(1), `max_size`: Some(36), added: 531, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::ImportedHashesPointer` (r:1 w:1)
	/// Proof: `BridgeKusamaGrandpa::ImportedHashesPointer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::ImportedHashes` (r:1 w:1)
	/// Proof: `BridgeKusamaGrandpa::ImportedHashes` (`max_values`: Some(1200), `max_size`: Some(36), added: 1521, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::ImportedHeaders` (r:0 w:2)
	/// Proof: `BridgeKusamaGrandpa::ImportedHeaders` (`max_values`: Some(1200), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 168]`.
	/// The range of component `v` is `[50, 100]`.
	fn submit_finality_proof(p: u32, v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31 + p * (60 ±0)`
		//  Estimated: `51735`
		// Minimum execution time: 384_542_000 picoseconds.
		Weight::from_parts(21_154_104, 0)
			.saturating_add(Weight::from_parts(0, 51735))
			// Standard Error: 4_259
			.saturating_add(Weight::from_parts(49_218_414, 0).saturating_mul(p.into()))
			// Standard Error: 14_213
			.saturating_add(Weight::from_parts(3_129_115, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `BridgeKusamaGrandpa::CurrentAuthoritySet` (r:1 w:1)
	/// Proof: `BridgeKusamaGrandpa::CurrentAuthoritySet` (`max_values`: Some(1), `max_size`: Some(50250), added: 50745, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::ImportedHashesPointer` (r:1 w:1)
	/// Proof: `BridgeKusamaGrandpa::ImportedHashesPointer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::ImportedHashes` (r:1 w:1)
	/// Proof: `BridgeKusamaGrandpa::ImportedHashes` (`max_values`: Some(1200), `max_size`: Some(36), added: 1521, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::BestFinalized` (r:0 w:1)
	/// Proof: `BridgeKusamaGrandpa::BestFinalized` (`max_values`: Some(1), `max_size`: Some(36), added: 531, mode: `MaxEncodedLen`)
	/// Storage: `BridgeKusamaGrandpa::ImportedHeaders` (r:0 w:2)
	/// Proof: `BridgeKusamaGrandpa::ImportedHeaders` (`max_values`: Some(1200), `max_size`: Some(68), added: 1553, mode: `MaxEncodedLen`)
	fn force_set_pallet_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `84`
		//  Estimated: `51735`
		// Minimum execution time: 65_760_000 picoseconds.
		Weight::from_parts(67_200_000, 0)
			.saturating_add(Weight::from_parts(0, 51735))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}

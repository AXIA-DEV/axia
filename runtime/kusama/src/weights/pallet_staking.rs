// Copyright 2017-2021 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `pallet_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/pallet_staking.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_staking::WeightInfo for WeightInfo<T> {
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn bond() -> Weight {
		(60_061_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: BagsList ListBags (r:2 w:2)
	fn bond_extra() -> Weight {
		(96_111_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: BagsList ListBags (r:2 w:2)
	fn unbond() -> Weight {
		(104_230_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn withdraw_unbonded_update(s: u32, ) -> Weight {
		(43_751_000 as Weight)
			// Standard Error: 0
			.saturating_add((22_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn withdraw_unbonded_kill(_s: u32, ) -> Weight {
		(85_563_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinValidatorBond (r:1 w:0)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking MaxValidatorsCount (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	fn validate() -> Weight {
		(58_586_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	fn kick(k: u32, ) -> Weight {
		(9_833_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((15_545_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	fn nominate(n: u32, ) -> Weight {
		(68_238_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((4_618_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(12 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	fn chill() -> Weight {
		(58_301_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking Payee (r:0 w:1)
	fn set_payee() -> Weight {
		(10_375_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn set_controller() -> Weight {
		(22_578_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Staking ValidatorCount (r:0 w:1)
	fn set_validator_count() -> Weight {
		(2_486_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_no_eras() -> Weight {
		(2_751_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		(2_717_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking ForceEra (r:0 w:1)
	fn force_new_era_always() -> Weight {
		(2_713_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Invulnerables (r:0 w:1)
	fn set_invulnerables(v: u32, ) -> Weight {
		(3_075_000 as Weight)
			// Standard Error: 0
			.saturating_add((55_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:2)
	fn force_unstake(s: u32, ) -> Weight {
		(81_282_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_862_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	fn cancel_deferred_slash(s: u32, ) -> Weight {
		(3_431_689_000 as Weight)
			// Standard Error: 224_000
			.saturating_add((19_865_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:2 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	fn payout_stakers_dead_controller(n: u32, ) -> Weight {
		(104_784_000 as Weight)
			// Standard Error: 28_000
			.saturating_add((45_088_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking ErasValidatorReward (r:1 w:0)
	// Storage: Staking Bonded (r:2 w:0)
	// Storage: Staking Ledger (r:2 w:2)
	// Storage: Staking ErasStakersClipped (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:0)
	// Storage: Staking ErasValidatorPrefs (r:1 w:0)
	// Storage: Staking Payee (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:2 w:2)
	fn payout_stakers_alive_staked(n: u32, ) -> Weight {
		(134_609_000 as Weight)
			// Standard Error: 30_000
			.saturating_add((57_863_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((5 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: BagsList ListNodes (r:3 w:3)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: BagsList ListBags (r:2 w:2)
	fn rebond(_l: u32, ) -> Weight {
		(96_874_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:1)
	// Storage: Staking ErasStakersClipped (r:0 w:2)
	// Storage: Staking ErasValidatorPrefs (r:0 w:2)
	// Storage: Staking ErasValidatorReward (r:0 w:1)
	// Storage: Staking ErasRewardPoints (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:2)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	fn set_history_depth(e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 65_000
			.saturating_add((29_033_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((7 as Weight).saturating_mul(e as Weight)))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking Payee (r:0 w:1)
	// Storage: Staking SpanSlash (r:0 w:1)
	fn reap_stash(s: u32, ) -> Weight {
		(84_056_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_870_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Staking CounterForNominators (r:1 w:0)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking Validators (r:2 w:0)
	// Storage: Staking Bonded (r:101 w:0)
	// Storage: Staking Ledger (r:101 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Nominators (r:101 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: Staking MinimumValidatorCount (r:1 w:0)
	// Storage: Staking CurrentEra (r:1 w:1)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Staking ErasStakersClipped (r:0 w:1)
	// Storage: Staking ErasValidatorPrefs (r:0 w:1)
	// Storage: Staking ErasStakers (r:0 w:1)
	// Storage: Staking ErasTotalStake (r:0 w:1)
	// Storage: Staking ErasStartSessionIndex (r:0 w:1)
	fn new_era(v: u32, n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 758_000
			.saturating_add((316_905_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 38_000
			.saturating_add((49_378_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(v as Weight)))
	}
	// Storage: Staking CounterForNominators (r:1 w:0)
	// Storage: Staking CounterForValidators (r:1 w:0)
	// Storage: Staking Validators (r:501 w:0)
	// Storage: Staking Bonded (r:1500 w:0)
	// Storage: Staking Ledger (r:1500 w:0)
	// Storage: Staking SlashingSpans (r:21 w:0)
	// Storage: Staking Nominators (r:1001 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	fn get_npos_voters(v: u32, n: u32, s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 93_000
			.saturating_add((22_439_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 93_000
			.saturating_add((31_005_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 3_196_000
			.saturating_add((35_609_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking Validators (r:501 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	fn get_npos_targets(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 31_000
			.saturating_add((8_949_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Staking MinValidatorBond (r:0 w:1)
	// Storage: Staking MaxValidatorsCount (r:0 w:1)
	// Storage: Staking ChillThreshold (r:0 w:1)
	// Storage: Staking MaxNominatorsCount (r:0 w:1)
	// Storage: Staking MinNominatorBond (r:0 w:1)
	fn set_staking_limits() -> Weight {
		(5_718_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Staking ChillThreshold (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking MaxNominatorsCount (r:1 w:0)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: Staking MinNominatorBond (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: BagsList ListNodes (r:2 w:2)
	// Storage: BagsList ListBags (r:1 w:1)
	// Storage: BagsList CounterForListNodes (r:1 w:1)
	fn chill_other() -> Weight {
		(70_741_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}

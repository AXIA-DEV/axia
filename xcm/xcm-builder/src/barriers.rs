// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.

//! Various implementations for `ShouldExecute`.

use frame_support::{ensure, traits::Contains, weights::Weight};
use axia_allychain::primitives::IsSystem;
use sp_std::{marker::PhantomData, result::Result};
use xcm::latest::{Instruction::*, Junction, Junctions, MultiLocation, WeightLimit::*, Xcm};
use xcm_executor::traits::{OnResponse, ShouldExecute};

/// Execution barrier that just takes `max_weight` from `weight_credit`.
///
/// Useful to allow XCM execution by local chain users via extrinsics.
/// E.g. `pallet_xcm::reserve_asset_transfer` to transfer a reserve asset
/// out of the local chain to another one.
pub struct TakeWeightCredit;
impl ShouldExecute for TakeWeightCredit {
	fn should_execute<Call>(
		_origin: &MultiLocation,
		_message: &mut Xcm<Call>,
		max_weight: Weight,
		weight_credit: &mut Weight,
	) -> Result<(), ()> {
		*weight_credit = weight_credit.checked_sub(max_weight).ok_or(())?;
		Ok(())
	}
}

/// Allows execution from `origin` if it is contained in `T` (i.e. `T::Contains(origin)`) taking
/// payments into account.
///
/// Only allows for `TeleportAsset`, `WithdrawAsset`, `ClaimAsset` and `ReserveAssetDeposit` XCMs
/// because they are the only ones that place assets in the Holding Register to pay for execution.
pub struct AllowTopLevelPaidExecutionFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowTopLevelPaidExecutionFrom<T> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		message: &mut Xcm<Call>,
		max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		ensure!(T::contains(origin), ());
		let mut iter = message.0.iter_mut();
		let i = iter.next().ok_or(())?;
		match i {
			ReceiveTeleportedAsset(..) |
			WithdrawAsset(..) |
			ReserveAssetDeposited(..) |
			ClaimAsset { .. } => (),
			_ => return Err(()),
		}
		let mut i = iter.next().ok_or(())?;
		while let ClearOrigin = i {
			i = iter.next().ok_or(())?;
		}
		match i {
			BuyExecution { weight_limit: Limited(ref mut weight), .. } if *weight >= max_weight => {
				*weight = max_weight;
				Ok(())
			},
			BuyExecution { ref mut weight_limit, .. } if weight_limit == &Unlimited => {
				*weight_limit = Limited(max_weight);
				Ok(())
			},
			_ => Err(()),
		}
	}
}

/// Allows execution from any origin that is contained in `T` (i.e. `T::Contains(origin)`) without any payments.
/// Use only for executions from trusted origin groups.
pub struct AllowUnpaidExecutionFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowUnpaidExecutionFrom<T> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		_message: &mut Xcm<Call>,
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		ensure!(T::contains(origin), ());
		Ok(())
	}
}

/// Allows a message only if it is from a system-level child allychain.
pub struct IsChildSystemAllychain<ParaId>(PhantomData<ParaId>);
impl<ParaId: IsSystem + From<u32>> Contains<MultiLocation> for IsChildSystemAllychain<ParaId> {
	fn contains(l: &MultiLocation) -> bool {
		matches!(
			l.interior(),
			Junctions::X1(Junction::Allychain(id))
				if ParaId::from(*id).is_system() && l.parent_count() == 0,
		)
	}
}

/// Allows only messages if the generic `ResponseHandler` expects them via `expecting_response`.
pub struct AllowKnownQueryResponses<ResponseHandler>(PhantomData<ResponseHandler>);
impl<ResponseHandler: OnResponse> ShouldExecute for AllowKnownQueryResponses<ResponseHandler> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		message: &mut Xcm<Call>,
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		match message.0.first() {
			Some(QueryResponse { query_id, .. })
				if ResponseHandler::expecting_response(origin, *query_id) =>
				Ok(()),
			_ => Err(()),
		}
	}
}

/// Allows execution from `origin` if it is just a straight `SubscribeVerison` or
/// `UnsubscribeVersion` instruction.
pub struct AllowSubscriptionsFrom<T>(PhantomData<T>);
impl<T: Contains<MultiLocation>> ShouldExecute for AllowSubscriptionsFrom<T> {
	fn should_execute<Call>(
		origin: &MultiLocation,
		message: &mut Xcm<Call>,
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ()> {
		ensure!(T::contains(origin), ());
		match (message.0.len(), message.0.first()) {
			(1, Some(SubscribeVersion { .. })) | (1, Some(UnsubscribeVersion)) => Ok(()),
			_ => Err(()),
		}
	}
}

// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types used to connect to the AxiaTest chain.

use relay_axlib_client::{Chain, ChainBase};
use std::time::Duration;

/// AxiaTest header id.
pub type HeaderId = relay_utils::HeaderId<bp_axctest::Hash, bp_axctest::BlockNumber>;

/// AxiaTest chain definition
#[derive(Debug, Clone, Copy)]
pub struct AxiaTest;

impl ChainBase for AxiaTest {
	type BlockNumber = bp_axctest::BlockNumber;
	type Hash = bp_axctest::Hash;
	type Hasher = bp_axctest::Hasher;
	type Header = bp_axctest::Header;
}

impl Chain for AxiaTest {
	const NAME: &'static str = "AxiaTest";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_axctest::AccountId;
	type Index = bp_axctest::Nonce;
	type SignedBlock = bp_axctest::SignedBlock;
	type Call = ();
	type Balance = bp_axctest::Balance;
}

/// AxiaTest header type used in headers sync.
pub type SyncHeader = relay_axlib_client::SyncHeader<bp_axctest::Header>;

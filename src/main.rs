// Copyright 2017-2020 Parity Technologies (UK) Ltd.
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

//! Axia CLI

#![warn(missing_docs)]

use color_eyre::eyre;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	axia_cli::run()?;
	Ok(())
}

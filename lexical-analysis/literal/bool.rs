/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

use location::Location;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct BoolOutput {
	pub value: Bool,
	pub location: Location,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Bool {
	True,
	False,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<bool> for Bool {
	fn from(b: bool) -> Self {
		match b {
			| true => Self::True,
			| false => Self::False,
		}
	}
}

impl From<Bool> for bool {
	fn from(b: Bool) -> Self {
		match b {
			| Bool::True => true,
			| Bool::False => false,
		}
	}
}

impl fmt::Display for Bool {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Bool::True => "true",
				| Bool::False => "false",
			}
		)
	}
}

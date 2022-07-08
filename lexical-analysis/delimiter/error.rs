/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum DelimiterParseError {
	/// N'est pas un délimiteur/séparateur valide.
	Invalid { found: String },
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for DelimiterParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::Invalid { .. } =>
					"Il ne s'agit pas d'un délimiteur valide.",
			}
		)
	}
}

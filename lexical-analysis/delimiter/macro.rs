/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_export]
macro_rules! delim {
	(
		- $enum:ident -
		$( $variant:ident = $char:literal ; )*
	) => {
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum $enum {
	$( $variant ),*
}

impl $enum {
	/// Retourne le signe, symbole, opérateur etc.
	pub fn sign(self) -> String {
		match self {
			$( | Self::$variant => $char.to_string() ),*
		}
	}
}

impl core::fmt::Display for $enum {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				$(
				| Self::$variant => stringify!($variant)
					.replace('_', "-")
					.to_ascii_lowercase()
				),*
			}
		)
	}
}
	};
}

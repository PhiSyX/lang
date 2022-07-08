/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_export]
macro_rules! Kword {
	(
	$(
		- $name:ident as $( $variant:ident )|*
	)*
	) => {
// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Keyword {
	$(
		#[doc = "Le mot-clé : "]
		#[doc = stringify!($name)]
		$name
	),*
}

// -------------- //
// Implémentation // ->  Interface
// -------------- //

impl core::str::FromStr for Keyword {
	type Err = self::error::KeywordParseError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		Ok(match input {
		$(
			$( | stringify!($variant) )* => Self::$name,
		)*
			| _ => {
				return Err(Self::Err::Unknown {
					found: input.to_string(),
				})
			}
		})
	}
}

impl core::fmt::Display for Keyword {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(
			f,
			"{}",
			match self {
			$(
				| Self::$name => stringify!($name).to_ascii_lowercase()
			),*
			}
		)
	}
}
	};
}

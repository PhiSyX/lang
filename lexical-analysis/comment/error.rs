/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum CommentParseError {
	/// Le commentaire n'a pas été terminé dû fait d'une rencontre EOF
	/// (End Of File), autrement dit la fin d'un flux.
	Unterminated { line: usize, column: usize },

	/// N'est pas un commentaire.
	IsNot,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for CommentParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::Unterminated { line, column } => format!(
					"Le commentaire n'a pas été terminé dû fait d'une \
					rencontre du point de code EOF (End Of File). \
					(emplacement {line},{column})"
				),
				| Self::IsNot => "Il ne s'agit pas d'un commentaire.".into(),
			}
		)
	}
}

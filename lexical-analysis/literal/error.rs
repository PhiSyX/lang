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
pub enum LiteralParseError {
	/// Erreur lors de l'analyse lexicale d'un nombre.
	Integer(IntegerParseError),

	/// Erreur lors de l'analyse lexicale d'une chaîne de caractères.
	String(StringParseError),
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum IntegerParseError {
	/// Il ne s'agit pas un point de code numérique.
	IsNot { found: char }, // NOTE(phisyx): peut-être à retirer ?
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum StringParseError {
	/// Il ne s'agit pas une chaîne de caractères.
	IsNot { found: char }, // NOTE(phisyx): peut-être à retirer ?

	/// La chaîne de caractère est mal formée.
	BadString,

	/// La chaîne de caractères n'est pas terminée.
	Unterminated { line: usize, column: usize },
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for LiteralParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::Integer(err) => err.to_string(),
				| Self::String(err) => err.to_string(),
			}
		)
	}
}

impl fmt::Display for IntegerParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::IsNot { found } => format!(
					"Le caractère '{}' n'est pas un point de code numérique.",
					found
				),
			}
		)
	}
}

impl fmt::Display for StringParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
            f,
            "{}",
            match self {
                | Self::IsNot { found } => format!(
                    "Le caractère '{}' n'est valide pour une chaîne de caractères.",
                    found
                ),
                | Self::BadString => "La chaîne de caractères est mal formée.".to_owned(),
                | Self::Unterminated { line, column } => format!(
                    "La chaîne de caractères n'est pas terminée en position 'L:{},C:{}'.",
                    line, column
                ),
            }
        )
	}
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use stream::prelude::StreamIteratorError;

#[cfg(feature = "comment")]
use crate::comment::CommentParseError;
use crate::{
	delimiter::error::DelimiterParseError,
	identifier::IdentifierParseError,
	literal::{IntegerParseError, LiteralParseError, StringParseError},
};

// ----------- //
// Énumération //
// ----------- //

/// Erreur lors d'une analyse lexicale.
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum LexicalError {
	#[cfg(feature = "comment")]
	/// Erreur lors de l'analyse lexicale d'un commentaire.
	Comment(CommentParseError),

	#[cfg(feature = "delimiter")]
	/// Erreur lors de l'analyse lexicale d'un délimiteur/séparateur.
	Delimiter(DelimiterParseError),

	#[cfg(feature = "identifier")]
	/// Erreur lors de l'analyse lexicale d'un identifiant.
	Identifier(IdentifierParseError),

	#[cfg(feature = "literal")]
	/// Erreur lors de l'analyse lexicale d'un literal.
	Literal(LiteralParseError),

	EOS,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[cfg(feature = "comment")]
impl From<CommentParseError> for LexicalError {
	fn from(error: CommentParseError) -> Self {
		Self::Comment(error)
	}
}

#[cfg(feature = "delimiter")]
impl From<DelimiterParseError> for LexicalError {
	fn from(error: DelimiterParseError) -> Self {
		Self::Delimiter(error)
	}
}

#[cfg(feature = "identifier")]
impl From<IdentifierParseError> for LexicalError {
	fn from(error: IdentifierParseError) -> Self {
		Self::Identifier(error)
	}
}

#[cfg(feature = "literal")]
impl From<IntegerParseError> for LexicalError {
	fn from(error: IntegerParseError) -> Self {
		Self::Literal(LiteralParseError::Integer(error))
	}
}

#[cfg(feature = "literal")]
impl From<StringParseError> for LexicalError {
	fn from(error: StringParseError) -> Self {
		Self::Literal(LiteralParseError::String(error))
	}
}

impl StreamIteratorError for LexicalError {
	fn eos() -> Self {
		Self::EOS
	}
}

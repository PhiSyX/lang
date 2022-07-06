/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use stream::prelude::StreamIteratorError;

#[cfg(feature = "comment")]
use crate::comment::CommentParseError;
use crate::delimiter::error::DelimiterParseError;

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

impl StreamIteratorError for LexicalError {
    fn eos() -> Self {
        Self::EOS
    }
}

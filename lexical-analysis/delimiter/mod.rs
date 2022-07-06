/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub(crate) mod error;
mod r#macro;
mod symbol;

use core::fmt;

use codepoints::{CodePoint, CodePointInterface};

use self::{
    error::DelimiterParseError,
    symbol::Symbol,
};

// ----------- //
// Énumération //
// ----------- //

/// Les délimiteurs, les séparateurs.
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Delimiter {
    /// Les symboles de délimitation.
    ///
    /// Par exemple: '#', '@', '.', etc...
    Symbol(Symbol),
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | Delimiter::Symbol(symbol) => symbol.to_string(),
            }
        )
    }
}

// 1 point de code.
impl<U> TryFrom<CodePoint<U>> for Delimiter
where
    U: CodePointInterface,
{
    type Error = DelimiterParseError;

    fn try_from(codepoint: CodePoint<U>) -> Result<Self, Self::Error> {
        Ok(match codepoint {
            // Le symboles de délimitation.
            | CodePoint::REVERSE_SOLIDUS => Self::Symbol(Symbol::ESCAPE),
            | CodePoint::NUMBER_SIGN => Self::Symbol(Symbol::HASH),
            | CodePoint::FULL_STOP => Self::Symbol(Symbol::DOT),
            | CodePoint::COMMA => Self::Symbol(Symbol::COMMA),
            | CodePoint::COLON => Self::Symbol(Symbol::COLON),
            | CodePoint::SEMICOLON => Self::Symbol(Symbol::SEMICOLON),
            | CodePoint::Unit(u) if u.is('@') => Self::Symbol(Symbol::AT),
            | CodePoint::Unit(u) if u.is('$') => Self::Symbol(Symbol::DOLLAR),
            | CodePoint::Unit(u) if u.is('_') => {
                Self::Symbol(Symbol::UNDERSCORE)
            }
            | CodePoint::Unit(u) if u.is('`') => Self::Symbol(Symbol::TILDE),
            | CodePoint::QUESTION_MARK => Self::Symbol(Symbol::QUESTION_MARK),

            | _ => {
                return Err(DelimiterParseError::Invalid {
                    found: codepoint.unit(),
                })
            }
        })
    }
}

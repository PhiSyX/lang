/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod boundary_pairs;
pub(crate) mod error;
mod r#macro;
mod operator;
mod symbol;

use core::fmt;

use codepoints::{CodePoint, CodePointInterface};

use self::{
    boundary_pairs::BoundaryPairs,
    error::DelimiterParseError,
    operator::{
        Assignment,
        Comparison,
        Operator,
    },
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

    /// Les délimiteurs paires.
    ///
    /// Par exemple: '{', '}', '(', ')', etc...
    BoundaryPairs(BoundaryPairs),

    /// Les opérateurs de délimitation.
    ///
    /// Par exemple: '==', '!=', '+=', etc...
    Operator(Operator),
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
                | Self::Symbol(symbol) => symbol.to_string(),
                | Self::BoundaryPairs(boundary_pairs) => {
                    boundary_pairs.to_string()
                }
                | Self::Operator(operator) => operator.to_string(),
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

            // Les délimiteurs paires.
            | CodePoint::LEFT_SQUARE_BRACKET => {
                Self::BoundaryPairs(BoundaryPairs::LEFT_SQUARE_BRACKET)
            }
            | CodePoint::RIGHT_SQUARE_BRACKET => {
                Self::BoundaryPairs(BoundaryPairs::RIGHT_SQUARE_BRACKET)
            }
            | CodePoint::LEFT_PARENTHESIS => {
                Self::BoundaryPairs(BoundaryPairs::LEFT_PARENTHESIS)
            }
            | CodePoint::RIGHT_PARENTHESIS => {
                Self::BoundaryPairs(BoundaryPairs::RIGHT_PARENTHESIS)
            }
            | CodePoint::LEFT_CURLY_BRACKET => {
                Self::BoundaryPairs(BoundaryPairs::LEFT_CURLY_BRACKET)
            }
            | CodePoint::RIGHT_CURLY_BRACKET => {
                Self::BoundaryPairs(BoundaryPairs::RIGHT_CURLY_BRACKET)
            }

            // Les opérateurs de délimitation.
            //   - Opérateurs d'assignations.
            | CodePoint::EQUALS_SIGN => {
                Self::Operator(Operator::Assignment(Assignment::EQUAL))
            }
            //  - Opérateurs de comparaison.
            | CodePoint::GREATER_THAN_SIGN => {
                Self::Operator(Operator::Comparison(Comparison::GREATER_THAN))
            }
            | CodePoint::LESS_THAN_SIGN => {
                Self::Operator(Operator::Comparison(Comparison::LESS_THAN))
            }
            | _ => {
                return Err(DelimiterParseError::Invalid {
                    found: codepoint.unit(),
                })
            }
        })
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod boundary_pairs;
pub(crate) mod error;
mod r#macro;
mod operator;
mod output;
mod symbol;

use core::fmt;

use codepoints::{CodePoint, CodePointInterface};

use self::{
	boundary_pairs::BoundaryPairs,
	operator::{
		Arithmetic, Assignment, Bitwise, Comparison, Logical, Operator,
	},
	symbol::Symbol,
};
pub use self::{error::DelimiterParseError, output::DelimiterOutput};

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
			//  - Opérateurs arithmétiques.
			| CodePoint::PLUS_SIGN => {
				Self::Operator(Operator::Arithmetic(Arithmetic::ADDITION))
			}
			| CodePoint::HYPHEN_MINUS => {
				Self::Operator(Operator::Arithmetic(Arithmetic::SUBTRACTION))
			}
			| CodePoint::SOLIDUS => {
				Self::Operator(Operator::Arithmetic(Arithmetic::DIVISION))
			}
			| CodePoint::ASTERISK => {
				Self::Operator(Operator::Arithmetic(Arithmetic::MULTIPLICATION))
			}
			| CodePoint::PERCENTAGE_SIGN => {
				Self::Operator(Operator::Arithmetic(Arithmetic::REMAINDER))
			}
			//  - Opérateurs logiques.
			| CodePoint::EXCLAMATION_MARK => {
				Self::Operator(Operator::Logical(Logical::NOT))
			}
			//  - Opérateurs binaires.
			| CodePoint::AMPERSAND => {
				Self::Operator(Operator::Bitwise(Bitwise::AND))
			}
			| CodePoint::VERTICAL_LINE => {
				Self::Operator(Operator::Bitwise(Bitwise::OR))
			}
			| CodePoint::CIRCUMFLEX_ACCENT => {
				Self::Operator(Operator::Bitwise(Bitwise::XOR))
			}
			| CodePoint::TILDE => {
				Self::Operator(Operator::Bitwise(Bitwise::NOT))
			}

			| _ if codepoint.is_valid() => {
				return Err(DelimiterParseError::Invalid {
					found: String::from(codepoint.unit()),
				})
			}

			| _ => {
				return Err(DelimiterParseError::Invalid {
					found: String::default(),
				})
			}
		})
	}
}

// 2 points de code.
impl<U> TryFrom<[CodePoint<U>; 2]> for Delimiter
where
	U: CodePointInterface,
{
	type Error = DelimiterParseError;

	fn try_from(codepoints: [CodePoint<U>; 2]) -> Result<Self, Self::Error> {
		Ok(match codepoints {
			// Le symboles de délimitation.
			// ".." (range)
			| [CodePoint::FULL_STOP, CodePoint::FULL_STOP] => {
				Self::Symbol(Symbol::RANGE)
			}
			// "::" (double colon)
			| [CodePoint::COLON, CodePoint::COLON] => {
				Self::Symbol(Symbol::DOUBLE_COLON)
			}
			// "->" (skinny arrow)
			| [CodePoint::HYPHEN_MINUS, CodePoint::GREATER_THAN_SIGN] => {
				Self::Symbol(Symbol::SKINNY_ARROW)
			}
			// "=>" (fat arrow)
			| [CodePoint::EQUALS_SIGN, CodePoint::GREATER_THAN_SIGN] => {
				Self::Symbol(Symbol::FAT_ARROW)
			}

			// Les opérateurs de délimitation.
			//   - Opérateurs d'assignations.
			// "+="
			| [CodePoint::PLUS_SIGN, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::ADDITION))
			}
			// "-="
			| [CodePoint::HYPHEN_MINUS, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::SUBTRACTION))
			}
			// "*="
			| [CodePoint::ASTERISK, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::MULTIPLICATION))
			}
			// "/="
			| [CodePoint::SOLIDUS, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::DIVISION))
			}
			// "%="
			| [CodePoint::PERCENTAGE_SIGN, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::REMAINDER))
			}
			// "&="
			| [CodePoint::AMPERSAND, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::BITWISE_AND))
			}
			// "^="
			| [CodePoint::CIRCUMFLEX_ACCENT, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::BITWISE_XOR))
			}
			// "|="
			| [CodePoint::VERTICAL_LINE, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::BITWISE_OR))
			}
			//   - Opérateurs de comparaison.
			// "=="
			| [CodePoint::EQUALS_SIGN, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Comparison(Comparison::EQUAL))
			}
			// "!="
			| [CodePoint::EXCLAMATION_MARK, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Comparison(Comparison::NOT_EQUAL))
			}
			// "<="
			| [CodePoint::LESS_THAN_SIGN, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Comparison(
					Comparison::LESS_THAN_OR_EQUAL,
				))
			}
			// ">="
			| [CodePoint::GREATER_THAN_SIGN, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Comparison(
					Comparison::GREATER_THAN_OR_EQUAL,
				))
			}
			//   - Opérateurs arithmétiques.
			// "^^"
			| [CodePoint::CIRCUMFLEX_ACCENT, CodePoint::CIRCUMFLEX_ACCENT] => {
				Self::Operator(Operator::Arithmetic(Arithmetic::EXPONENTIATION))
			}
			//  - Opérateurs logiques.
			// "&&"
			| [CodePoint::AMPERSAND, CodePoint::AMPERSAND] => {
				Self::Operator(Operator::Logical(Logical::AND))
			}
			// "||"
			| [CodePoint::VERTICAL_LINE, CodePoint::VERTICAL_LINE] => {
				Self::Operator(Operator::Logical(Logical::OR))
			}
			//   - Opérateurs binaires.
			// "<<"
			| [CodePoint::LESS_THAN_SIGN, CodePoint::LESS_THAN_SIGN] => {
				Self::Operator(Operator::Bitwise(Bitwise::LEFT_SHIFT))
			}
			// ">>"
			| [CodePoint::GREATER_THAN_SIGN, CodePoint::GREATER_THAN_SIGN] => {
				Self::Operator(Operator::Bitwise(Bitwise::RIGHT_SHIFT))
			}

			| _ => {
				return Err(DelimiterParseError::Invalid {
					found: codepoints.iter().map(|c| c.unit()).collect(),
				});
			}
		})
	}
}

// 3 points de code.
impl<U> TryFrom<[CodePoint<U>; 3]> for Delimiter
where
	U: CodePointInterface,
{
	type Error = DelimiterParseError;

	fn try_from(codepoints: [CodePoint<U>; 3]) -> Result<Self, Self::Error> {
		Ok(match codepoints {
			// Le symboles de délimitation.
			// "..=" (range inclusive)
			| [CodePoint::FULL_STOP, CodePoint::FULL_STOP, CodePoint::EQUALS_SIGN] => {
				Self::Symbol(Symbol::RANGE_INCLUSIVE)
			}

			// Les opérateurs de délimitation.
			//   - Opérateurs d'assignations.
			// "<<="
			| [CodePoint::LESS_THAN_SIGN, CodePoint::LESS_THAN_SIGN, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::LEFT_SHIFT))
			}
			// ">>="
			| [CodePoint::GREATER_THAN_SIGN, CodePoint::GREATER_THAN_SIGN, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::RIGHT_SHIFT))
			}
			// "&&="
			| [CodePoint::AMPERSAND, CodePoint::AMPERSAND, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::LOGICAL_AND))
			}
			// "||="
			| [CodePoint::VERTICAL_LINE, CodePoint::VERTICAL_LINE, CodePoint::EQUALS_SIGN] => {
				Self::Operator(Operator::Assignment(Assignment::LOGICAL_OR))
			}

			| _ => {
				return Err(DelimiterParseError::Invalid {
					found: codepoints.iter().map(|cd| cd.unit()).collect(),
				});
			}
		})
	}
}

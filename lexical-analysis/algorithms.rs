/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[cfg(feature = "comment")]
use crate::comment::{CommentOutput, CommentParseError};
#[cfg(feature = "identifier")]
use crate::identifier::{IdentifierOutput, IdentifierParseError};
#[cfg(feature = "literal")]
use crate::literal::{
	BoolOutput, IntegerOutput, IntegerParseError, StringOutput,
	StringParseError,
};

// --------- //
// Interface //
// --------- //

pub trait TokenizerAlgorithms {
	#[cfg(feature = "comment")]
	/// Consommer un commentaire.
	///
	/// Le résumé de l'algorithme, du code, dépend du langage.
	//
	/// Considérons que les commentaires suivants soient de types :
	///    - `// Mon commentaire`    => CommentType::Line
	///    - `/* Mon commentaire */` => CommentType::Block
	///
	/// 1. CommentType::Line :
	///     Si les deux points de code suivant sont U+002F SOLIDUS (/) suivi
	/// d'un U+002F SOLIDUS (/), nous devons les consommer, ainsi que tous les
	/// points de code suivant jusqu'à ce que nous rencontrons un point de code
	/// U+000A LINE FEED (\n), autrement dit la fin de la ligne.
	///
	/// 2. CommentType::Block :
	///   Si les deux points de code suivants sont U+002F SOLIDUS (/) suivi d'un
	/// U+002A ASTERISK (*), nous devons les consommer ainsi que tous les points
	/// de code suivants jusqu'au premier U+002A ASTERISK (*) suivi d'un
	/// U+002F SOLIDUS (/), ou jusqu'à un point de code EOF (End Of File).
	fn consume_comments(&mut self) -> Result<CommentOutput, CommentParseError>;

	#[cfg(feature = "identifier")]
	/// Consommer un identifiant.
	///
	/// Le résumé de l'algorithme, du code, dépend du langage.
	fn consume_ident_sequence(
		&mut self,
	) -> Result<IdentifierOutput, IdentifierParseError>;

	#[cfg(feature = "literal")]
	/// Consommer un nombre, entier.
	///
	/// Un nombre peut être de type :
	///   1. Integer::Decimal     : `71`, `10.0`, `.463`
	///   2. Integer::Hexadecimal : `0x2A`
	///   3. Integer::Binaire     : `0b10000000000`
	///   4. Integer::Octal       : `0o2000`
	fn consume_numeric(&mut self) -> Result<IntegerOutput, IntegerParseError>;

	#[cfg(feature = "literal")]
	/// Consommer une chaîne de caractères.
	///
	/// Une chaîne de caractère commence par un U+0022 QUOTATION MARK (") suivi
	/// d'une suite de caractères, et se termine par un U+0022 QUOTATION MARK
	/// (").
	/// Dans certaines syntaxes, une chaîne de caractères peut également être
	/// délimitée par un point de code U+0027 APOSTROPHE (') suivi d'une suite
	/// de caractères, et se terminant par un point de code U+0027 APOSTROPHE
	/// ('). Exemple, en JavaScript avec `'hello'`. Ce n'est pas le cas dans
	/// MikuScript. La syntaxe de MikuScript utilise le point de code U+0027
	/// pour désigner un SEUL caractère (EX: `let ch: char = 'C';`).
	fn consume_string(&mut self) -> Result<StringOutput, StringParseError>;

	#[cfg(feature = "literal")]
	/// Consommer un booléen.
	///
	/// Un booléen est un mot réservé qui peut être soit `true` soit `false`.
	fn consume_boolean(&mut self) -> Option<BoolOutput>;
}

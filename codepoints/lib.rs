/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod interface;

pub use self::interface::CodePointInterface;

// ----- //
// Macro //
// ----- //

macro_rules! impl_codepoint {
	(
	$(
		$(#[$attr:meta])*
		| $ch:literal => $variant:ident
	)*
	) => {
// ----------- //
// Énumération //
// ----------- //

/// Les différents points de code
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum CodePoint<Unit> {
	// NOTE(phisyx): il y a trop d'unités, je ne vais pas m'amuser à toutes
	// les lister :)
	Unit(Unit),

	/// Espace blanc.
	///
	/// Peut correspondre à l'une de ces unités ASCII:
	///   - U+0009 CHARACTER TABULATION (TAB)
	///   - U+000A LINE FEED (LF)       - LIRE NOTE ci-bas.
	///   - U+000C FORM FEED (FF)       - LIRE NOTE ci-bas.
	///   - U+000D CARRIAGE RETURN (CR) - LIRE NOTE ci-bas.
	///   - U+0020 SPACE
	///
	/// NOTE(phisyx): ces unités ne sont pas mis dans cette variante par
	/// défaut, elles sont définies dans [CodePoint::Newline]. Pour définir ces
	/// unités en tant qu'espace blanc, il faudra appliquer un
	/// [filtre](stream::InputStream::define_filter_scan()) explicitement
	/// à l'initialisation du [flux d'entrée](stream::InputStream).
	Whitespace(Unit),

	/// Saut de ligne.
	///
	/// Peut correspond à l'une de ces unités ASCII:
	///   - U+000A LINE FEED (LF)
	///   - U+000C FORM FEED (FF)
	///   - U+000D CARRIAGE RETURN (CR)
	Newline(Unit),

	$(
		$(#[$attr])*
		$variant,
	)*


	/// EOF.
	EOF,

	/// Ignore.
	// NOTE(phisyx): si une unité a pour valeur cette variante, ce sera la
	// prochaine variante, qui n'est évidemment pas une même variante
	// CodePoint::Ignore, d'un flux qui sera retournée.
	Ignore,
}

impl<U> CodePoint<U>
where
	U: CodePointInterface,
{
	pub fn maybe_unit(&self) -> Option<char> {
		Some(match self {
			| Self::Unit(codepoint)
			| Self::Whitespace(codepoint)
			| Self::Newline(codepoint) => codepoint.as_char(),

			| Self::NULL => return None,

			$(
				#[allow(unreachable_patterns)]
				| Self::$variant => $ch,
			)*

			| Self::EOF | Self::Ignore => return None,
		})
	}
}

impl<U: CodePointInterface> From<U> for CodePoint<U> {
	fn from(unit: U) -> Self {
		match unit {
			| unit if unit.is_newline() => Self::Newline(unit),
			| unit if unit.is_whitespace() => Self::Whitespace(unit),
		 $( | unit if unit.is($ch) => Self::$variant, )*
			| _ => Self::Unit(unit),
		}
	}
}
	};
}

// -------------- //
// Implémentation //
// -------------- //

impl<U> CodePoint<U>
where
	U: CodePointInterface,
{
	pub fn is_ident(&self) -> bool {
		self.maybe_unit()
			.map(|unit| unit.is_ident())
			.unwrap_or_default()
	}

	pub fn is_ident_start(&self) -> bool {
		self.maybe_unit()
			.map(|unit| unit.is_ident_start())
			.unwrap_or_default()
	}

	pub fn is_ident_after_start(&self) -> bool {
		self.maybe_unit()
			.map(|unit| unit.is_ident_after_start())
			.unwrap_or_default()
	}

	pub fn is_newline(&self) -> bool {
		self.maybe_unit()
			.map(|unit| unit.is_newline())
			.unwrap_or_default()
	}

	pub fn is_whitespace(&self) -> bool {
		self.maybe_unit()
			.map(|unit| unit.is_whitespace())
			.unwrap_or_default()
	}

	pub fn is(&self, unit: char) -> bool {
		self.unit().is(unit)
	}

	pub fn is_valid(&self) -> bool {
		!matches!(self, Self::NULL | Self::EOF | Self::Ignore)
	}

	pub fn unit(&self) -> char {
		self.maybe_unit()
			.expect("n'est pas un point de code valide pour notre script.")
	}
}

impl_codepoint! {
	/// Point d'exclamation
	///
	/// Correspond à l'unité ASCII:
	///   - U+0021 EXCLAMATION MARK (!)
	|   '!'   =>  EXCLAMATION_MARK

	/// Double guillemet
	///
	/// Correspond à l'unité ASCII:
	///   - U+0022 QUOTATION MARK (")
	|   '"'   =>  QUOTATION_MARK

	/// Hash
	///
	/// Correspond à l'unité ASCII:
	///   - U+0023 NUMBER SIGN (#)
	|   '#'   =>  NUMBER_SIGN

	/// Pourcentage
	///
	/// Correspond à l'unité ASCII:
	///   - U+0025 PERCENTAGE SIGN (%)
	|   '%'   =>  PERCENTAGE_SIGN

	/// &
	///
	/// Correspond à l'unité ASCII:
	///   - U+0026 AMPERSAND (&)
	|   '&'   =>  AMPERSAND

	/// Simple guillemet
	///
	/// Correspond à l'unité ASCII:
	///  - U+0027 APOSTROPHE (')
	|   '\''   =>  APOSTROPHE

	/// Asterisk
	///
	/// Correspond à l'unité ASCII:
	///   - U+002A ASTERISK (*)
	|   '*'   =>  ASTERISK

	/// Plus
	///
	/// Correspond à l'unité ASCII:
	///   - U+002B PLUS SIGN (+)
	|   '+'   =>  PLUS_SIGN

	/// Tiret
	///
	/// Correspond à l'unité ASCII:
	///   - U+002D HYPHEN-MINUS (-)
	|   '-'   =>  HYPHEN_MINUS

	/// Slash
	///
	/// Correspond à l'unité ASCII:
	///   - U+002F SOLIDUS (/)
	|   '/'   =>  SOLIDUS

	/// Signe <
	///
	/// Correspond à l'unité ASCII:
	///   - U+003C LESS-THAN SIGN (<)
	|   '<'   =>  LESS_THAN_SIGN

	/// Signe >
	///
	/// Correspond à l'unité ASCII:
	///   - U+003E GREATER-THAN SIGN (>)
	|   '>'   =>  GREATER_THAN_SIGN

	/// Question
	///
	/// Correspond à l'unité ASCII:
	///   - U+003F QUESTION MARK (?)
	|   '?'   =>  QUESTION_MARK

	/// Slash inversé
	///
	/// Correspond à l'unité ASCII:
	///   - U+005C REVERSE SOLIDUS (\)
	|   '\\'  =>  REVERSE_SOLIDUS

	/// Crochet gauche
	///
	/// Correspond à l'unité ASCII:
	///   - U+005B LEFT SQUARE BRACKET ([)
	|   '['   =>  LEFT_SQUARE_BRACKET
	/// Crochet droit
	///
	/// Correspond à l'unité ASCII:
	///   - U+005D RIGHT SQUARE BRACKET (])
	|   ']'   =>  RIGHT_SQUARE_BRACKET

	/// Parenthèse gauche
	///
	/// Correspond à l'unité ASCII:
	///   - U+0028 LEFT PARENTHESIS (()
	|   '('   =>  LEFT_PARENTHESIS
	/// Parenthèse droite
	///
	/// Correspond à l'unité ASCII:
	///   - U+0029 RIGHT PARENTHESIS ())
	|   ')'   =>  RIGHT_PARENTHESIS

	/// Accolade gauche
	///
	/// Correspond à l'unité ASCII:
	///   - U+007B LEFT CURLY BRACKET ({)
	|   '{'   =>  LEFT_CURLY_BRACKET
	/// Accolade droite
	///
	/// Correspond à l'unité ASCII:
	///   - U+007B RIGHT CURLY BRACKET ({)
	|   '}'   =>  RIGHT_CURLY_BRACKET

	/// Point
	///
	/// Correspond à l'unité ASCII:
	///   - U+002E FULL STOP (.)
	|   '.'   => FULL_STOP

	/// Virgule
	///
	/// Correspond à l'unité ASCII:
	///   - U+002C COMMA (,)
	|   ','   =>  COMMA

	/// Deux point
	///
	/// Correspond à l'unité ASCII:
	///    - U+003A COLON (:)
	|   ':'   =>  COLON

	/// Point virgule
	///
	/// Correspond à l'unité ASCII:
	///    - U+003B SEMICOLON (;)
	|   ';'   =>  SEMICOLON

	/// Égal
	///
	/// Correspond à l'unité ASCII:
	///    - U+003D EQUALS SIGN (=)
	|   '='   =>  EQUALS_SIGN

	/// Accent circonflexe
	///
	/// Correspond à l'unité ASCII:
	///   - U+005E CIRCUMFLEX ACCENT (^)
	|   '^'   =>  CIRCUMFLEX_ACCENT

	/// Pipe
	///
	/// Correspond à l'unité ASCII:
	///   - U+007C VERTICAL LINE (|)
	|   '|'   =>  VERTICAL_LINE

	/// Tilde
	///
	/// Correspond à l'unité ASCII:
	///   - U+007E TILDE (~)
	|   '~'   =>  TILDE

	/// NULL.
	///
	/// Correspond à l'unité ASCII:
	///   U+0000 NULL
	|   '\0'  =>  NULL
}

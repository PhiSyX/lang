/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// --------- //
// Interface //
// --------- //

pub trait CodePointInterface:
	PartialEq + Eq + Copy + Clone + ::core::fmt::Debug
{
	/// Cast en `char`
	fn as_char(&self) -> char;

	/// Cast en `u8`
	fn as_byte(&self) -> u8;

	fn is(&self, unit: char) -> bool;

	/// Un C0 control est un point de code dans la gamme U+0000 NULL à
	/// U+001F INFORMATION SEPARATOR ONE, inclus.
	fn is_c0_control(&self) -> bool;

	/// A C0 control ou space est un C0 control ou U+0020 SPACE.
	fn is_c0_control_or_space(&self) -> bool;

	/// digit
	///
	/// Un point de code compris entre U+0030 DIGIT ZERO (0) et U+0039 DIGIT
	/// NINE (9) inclus.
	fn is_digit(&self) -> bool;

	/// Lettre
	///
	/// Une lettre majuscule ou une lettre minuscule.
	fn is_letter(&self) -> bool;

	/// Lettre minuscule
	///
	/// Une lettre minuscule.
	fn is_lowercase_letter(&self) -> bool;

	/// Lettre majuscule
	///
	/// Une lettre majuscule.
	fn is_uppercase_letter(&self) -> bool;

	/// Un point de code ASCII est un point de code situé dans la plage
	/// U+0000 NULL à U+007F DELETE, inclusivement.
	fn is_ascii_code_point(&self) -> bool;

	/// ident-start code point
	///
	/// Une lettre, un point de code non ASCII ou U+005F LOW LINE (_).
	fn is_ident_start(&self) -> bool;

	/// ident-after-start code point
	///
	/// Un caractère alphanumérique, un point de code non ASCII ou U+005F LOW
	/// LINE (_).
	fn is_ident_after_start(&self) -> bool;

	/// ident code point
	///
	/// Un point de code ident de début, un chiffre ou U+002D HYPHEN-MINUS
	/// (-).
	fn is_ident(&self) -> bool;

	/// newline
	///
	/// Un point de code U+000A LINE FEED ou U+000D CARRIAGE RETURN ou U+000C
	/// FORM FEED.
	fn is_newline(&self) -> bool;

	/// non-ASCII code point
	///
	/// Un point de code dont la valeur est égale ou supérieure à U+0080
	/// <control>
	fn is_non_ascii(&self) -> bool;

	/// Un non-caractère est un point de code qui se trouve dans
	/// l'intervalle des caractères. U+FDD0 à U+FDEF, inclus,
	/// ou U+FFFE, U+FFFF, U+1FFFE, U+1FFFF, U+2FFFE, U+2FFFF, U+3FFFE,
	/// U+3FFFF, U+4FFFE, U+4FFFF, U+5FFFE, U+5FFFF, U+6FFFE, U+6FFFF,
	/// U+7FFFE, U+7FFFF, U+8FFFE, U+8FFFF, U+9FFFE, U+9FFFF, U+AFFFE,
	/// U+AFFFF, U+BFFFE, U+BFFFF, U+CFFFE, U+CFFFF, U+DFFFE, U+DFFFF,
	/// U+EFFFE, U+EFFFF, U+FFFFE, U+FFFFF, U+10FFFE, ou U+10FFFF.
	fn is_noncharacter(&self) -> bool;

	/// NULL
	///
	/// Un point de code U+0000 NULL.
	fn is_null(&self) -> bool;

	/// Une valeur scalaire est un point de code qui n'est pas un
	/// substitut.
	fn is_scalar_value(&self) -> bool {
		!self.is_surrogate()
	}

	/// Un substitut est un point de code qui se trouve dans la plage
	/// U+D800 à U+DFFF, inclus.
	fn is_surrogate(&self) -> bool;

	/// Espace blanc.
	///
	/// Un saut de ligne, une TABULATION DE CARACTÈRES U+0009 ou un ESPACE
	/// U+0020.
	fn is_whitespace(&self) -> bool;
}

// ----- //
// Macro //
// ----- //

macro_rules! impl_cdi {
	($($type:ty)|*) => {
	$(
	impl CodePointInterface for $type {
		fn as_char(&self) -> char {
			*self as char
		}

		fn as_byte(&self) -> u8 {
			*self as u8
		}

		fn is(&self, unit: char) -> bool {
			*self as char == unit
		}

		fn is_c0_control(&self) -> bool {
			matches!(self.as_char(), '\0'..='\u{001F}')
		}

		fn is_c0_control_or_space(&self) -> bool {
			self.is_c0_control() || self.is(' ')
		}

		fn is_digit(&self) -> bool {
			self.as_char().is_ascii_digit()
		}

		fn is_letter(&self) -> bool {
			self.is_lowercase_letter() || self.is_uppercase_letter()
		}

		 fn is_lowercase_letter(&self) -> bool {
			self.is_ascii_lowercase()
		}

		fn is_uppercase_letter(&self) -> bool {
			self.is_ascii_uppercase()
		}

		fn is_ascii_code_point(&self) -> bool {
			matches!(self.as_char(), '\0'..='\u{007F}')
		}

		fn is_ident_start(&self) -> bool {
			self.is_letter() || self.is_non_ascii() || self.is('_')
		}

		fn is_ident_after_start(&self) -> bool {
			self.is_ascii_alphanumeric() || self.is_non_ascii() || self.is('_')
		}

		fn is_ident(&self) -> bool {
			self.is_ident_start() || self.is_ascii_digit() || self.is('-')
		}

		fn is_newline(&self) -> bool {
			self.is('\n') || self.is('\x0C') || self.is('\r')
		}

		fn is_non_ascii(&self) -> bool {
			*self as u32 >= 0x80
		}

		fn is_noncharacter(&self) -> bool {
			matches!(self.as_char(),
				| '\u{FDD0}'..='\u{FDEF}'
				| '\u{FFFE}'..='\u{FFFF}'
				| '\u{1_FFFE}'..='\u{1_FFFF}'
				| '\u{2_FFFE}'..='\u{2_FFFF}'
				| '\u{3_FFFE}'..='\u{3_FFFF}'
				| '\u{4_FFFE}'..='\u{4_FFFF}'
				| '\u{5_FFFE}'..='\u{5_FFFF}'
				| '\u{6_FFFE}'..='\u{6_FFFF}'
				| '\u{7_FFFE}'..='\u{7_FFFF}'
				| '\u{8_FFFE}'..='\u{8_FFFF}'
				| '\u{9_FFFE}'..='\u{9_FFFF}'
				| '\u{A_FFFE}'..='\u{A_FFFF}'
				| '\u{B_FFFE}'..='\u{B_FFFF}'
				| '\u{C_FFFE}'..='\u{C_FFFF}'
				| '\u{D_FFFE}'..='\u{D_FFFF}'
				| '\u{E_FFFE}'..='\u{E_FFFF}'
				| '\u{F_FFFE}'..='\u{F_FFFF}'
				| '\u{10_FFFE}'..='\u{10_FFFF}')
		}

		fn is_null(&self) -> bool {
			self.is('\0')
		}

		fn is_surrogate(&self) -> bool {
			matches!(self.as_char(), '\u{D_8000}'..='\u{D_FFFF}')
		}

		fn is_whitespace(&self) -> bool {
			self.as_char().is_whitespace()
			// ou self.is_ascii_whitespace()
		}
	}
	)?
	};
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl_cdi! { char | u8 }

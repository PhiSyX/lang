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

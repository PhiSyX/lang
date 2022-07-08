/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

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
    IsNot { found: char },
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum StringParseError {
    /// Il ne s'agit pas une chaîne de caractères.
    IsNot { found: char },

    /// La chaîne de caractère est mal formée.
    BadString,

    /// La chaîne de caractères n'est pas terminée.
    Unterminated { line: usize, column: usize },
}

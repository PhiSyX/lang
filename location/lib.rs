/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod interface;

use core::{fmt, str};

pub use self::interface::LocationInterface;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Location {
    /// La numéro de ligne.
    // NOTE(phisyx): commence à 1.
    pub line: usize,

    /// L'index de la colonne, par rapport au numéro de ligne.
    // NOTE(phisyx): commence à 1.
    pub column: usize,

    /// Le nombre total de points de code analysés.
    pub total: usize,
}

// -------------- //
// Implémentation //
// -------------- //

impl Location {
    /// Crée un emplacement par défaut.
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 1,
            total: 1,
        }
    }
}

impl LocationInterface for Location {
    fn increment_total_by(&mut self, n: usize) {
        self.total = self.total.saturating_add(n);
    }

    fn increment_line_by(&mut self, n: usize) {
        self.line = self.line.saturating_add(n);
    }

    fn increment_column_by(&mut self, n: usize) {
        self.column = self.column.saturating_add(n);
    }

    fn reset_column(&mut self) {
        self.column = 1;
    }
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            total: 1,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L:{},C:{}", self.line, self.column)
    }
}

impl str::FromStr for Location {
    type Err = &'static str;

    // L:{0..9},C:{0..9}
    fn from_str(location_str: &str) -> Result<Self, Self::Err> {
        let mut chars = location_str.chars();

        // Line
        let mut line = String::default();
        for ch in chars.by_ref() {
            if ch == 'L' || ch == ':' {
                continue;
            }

            if ch == ',' {
                break;
            }

            line.push(ch);
        }

        // Column
        let mut column = String::default();
        for ch in chars.by_ref() {
            if ch == 'C' || ch == ':' {
                continue;
            }

            column.push(ch);
        }

        let line: usize = line
            .parse()
            .map_err(|_| "Il ne s'agit pas d'une ligne valide.")?;
        let column: usize = column
            .parse()
            .map_err(|_| "Il ne s'agit pas d'une colonne valide.")?;

        Ok(Self {
            line,
            column,
            total: line + column,
        })
    }
}

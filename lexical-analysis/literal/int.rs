/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Integer {
    /// Nombre décimal.
    ///
    /// Exemple: 71, 10.0, 1.463, ...
    Decimal { int: String },

    /// Nombre hexadécimal.
    ///
    /// Exemple: 0x2A, ...
    Hexadecimal { hex: String },

    /// Nombre binaire.
    ///
    /// Exemple: 0b10000000000, ...
    Binary { bin: String },

    /// Nombre octal.
    ///
    /// Exemple: 0o2000, ...
    Octal { oct: String },
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &'static str = match self {
            | Self::Decimal { .. } => "decimal",
            | Self::Hexadecimal { .. } => "hexadecimal",
            | Self::Binary { .. } => "binary",
            | Self::Octal { .. } => "octal",
        };
        write!(f, "numeric-{s}")
    }
}

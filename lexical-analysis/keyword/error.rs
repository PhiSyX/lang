/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum KeywordParseError {
    Unknown { found: String },
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for KeywordParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | Self::Unknown { found } =>
                    format!("Le mot-clé {} n'existe pas.", found),
            }
        )
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod error;
mod int;
mod output;
mod state;
mod string;

use core::fmt;

use self::int::Integer;
pub use self::{error::*, output::*, state::*};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Literal {
    /// Nombre.
    Integer(Integer),

    /// Chaîne de caractères.
    String(StringOutput),
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = match self {
            | Literal::Integer(int) => int.to_string(),
            | Literal::String(_) => "string".to_owned(),
        };
        write!(f, "{l}")
    }
}

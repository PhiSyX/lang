/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod bool;
mod error;
mod int;
mod output;
mod state;
mod string;

use core::fmt;

pub use self::{bool::Bool, error::*, int::Integer, output::*, state::*};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum Literal {
	/// Nombre.
	Integer(IntegerOutput),

	/// Chaîne de caractères.
	String(StringOutput),

	/// Booléen.
	Bool(BoolOutput),
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let l = match self {
			| Literal::Integer(int) => int.to_string(),
			| Literal::String(_) => "string".to_owned(),
			| Literal::Bool(_) => "bool".to_owned(),
		};
		write!(f, "{l}")
	}
}

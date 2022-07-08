/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod algorithms;
#[cfg(feature = "comment")]
pub mod comment;
#[cfg(feature = "delimiter")]
pub mod delimiter;
mod error;
#[cfg(feature = "identifier")]
pub mod identifier;
mod io;
#[cfg(feature = "keyword")]
pub mod keyword;
#[cfg(feature = "literal")]
pub mod literal;

pub mod prelude {
	pub use super::{algorithms::TokenizerAlgorithms, error::LexicalError};
}

pub use self::io::{Input, ParseState};

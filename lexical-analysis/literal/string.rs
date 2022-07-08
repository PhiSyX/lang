/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::ops;

use location::{Location, LocationInterface};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct StringOutput {
	/// La chaîne de caractères analysée.
	/// Les symboles de début et de fin ne sont pas inclus.
	pub data: String,

	/// L'emplacement de la chaîne de caractères dans la source.
	pub location: Location,
}

// -------------- //
// Implémentation //
// -------------- //

impl StringOutput {
	/// Ajoute un caractère à la fin d'une chaîne de caractères.
	pub fn append(&mut self, ch: char) {
		self.data.push(ch);
		self.location.increment_column();
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ops::Deref for StringOutput {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

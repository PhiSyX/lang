/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::Location;

// --------- //
// Interface //
// --------- //

pub trait LocationInterface {
	/// Défini le nouvel emplacement, dans une structure par exemple.
	fn define_location(&mut self, _: Location) {
		unimplemented!(
			"Défini le nouvel emplacement, dans une structure par exemple."
		);
	}

	/// Défini le nouvel emplacement, dans une structure par exemple, si n'est
	/// pas déjà défini.
	fn define_location_if_not_already_set(&mut self, _: Location) {
		unimplemented!(
			"Défini le nouvel emplacement, dans une structure par exemple\
            , si n'est pas déjà défini."
		);
	}

	/// Incrémente le total des points de code analysés de 1.
	fn increment_total(&mut self) {
		self.increment_total_by(1);
	}

	/// Incrémente le total des points de code analysés de N.
	fn increment_total_by(&mut self, _: usize) {
		unimplemented!("Incrémente le total des points de code analysés de N.");
	}

	/// Incrémente une ligne de 1.
	fn increment_line(&mut self) {
		self.increment_line_by(1);
	}

	/// Incrémente une ligne de N.
	fn increment_line_by(&mut self, _: usize) {
		unimplemented!("Incrémente une ligne de N.");
	}

	/// Incrémente une colonne de 1.
	fn increment_column(&mut self) {
		self.increment_column_by(1);
	}

	/// Incrémente une colonne de N.
	fn increment_column_by(&mut self, _: usize) {
		unimplemented!("Incrémente une colonne de N.");
	}

	/// Réinitialise la colonne.
	fn reset_column(&mut self) {
		unimplemented!("Réinitialise la colonne.");
	}
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use location::Location;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct IdentifierOutput {
	/// L'identifiant analysé.
	pub identifier: String,

	/// L'emplacement de l'identifiant dans la source.
	pub location: Location,
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// --------- //
// Interface //
// --------- //

pub trait Input {
	type Output;
	type State: ParseState;

	/// Fonction d'analyse.
	fn parse(self) -> Self::Output;
}

pub trait ParseState {
	/// Change l'état de l'analyseur.
	fn switch(&mut self, new_state: Self);
}

impl ParseState for () {
	fn switch(&mut self, _: Self) {}
}

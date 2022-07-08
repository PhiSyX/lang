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
    pub data: String,
    pub location: Location,
}

// -------------- //
// Implémentation //
// -------------- //

impl StringOutput {
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

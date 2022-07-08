/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// ----------- //
// Énumération //
// ----------- //

#[derive(Default)]
pub enum IntegerState {
    #[default]
    Initial,

    Zero,

    DecimalInteger,
    DecimalFloat,

    Hexadecimal,
    Binary,
    Octal,

    IsNot,
}

#[derive(Default)]
pub enum StringState {
    #[default]
    Initial,

    SingleQuoted,
    DoubleQuoted,
}

// -------------- //
// Implémentation //
// -------------- //

impl IntegerState {
    pub fn switch(&mut self, new_state: Self) {
        *self = new_state;
    }
}

impl StringState {
    pub fn switch(&mut self, new_state: Self) {
        *self = new_state;
    }
}

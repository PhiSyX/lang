/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod error;
mod output;
mod state;

pub use self::{
	error::CommentParseError,
	output::{CommentOutput, CommentType},
	state::CommentState,
};

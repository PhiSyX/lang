/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub(crate) mod error;
mod r#macro;

use crate::Kword;

Kword! {
    Export   as export | pub | public,
    Function as fn | func | function,

    Let     as let,
    Mut     as mut,
    Const   as const
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword() {
        assert_eq!("function".parse::<Keyword>(), Ok(Keyword::Function));
        assert_eq!("fn".parse::<Keyword>(), Ok(Keyword::Function));
        assert_eq!("func".parse::<Keyword>(), Ok(Keyword::Function));

        assert_eq!("export".parse::<Keyword>(), Ok(Keyword::Export));
        assert_eq!("pub".parse::<Keyword>(), Ok(Keyword::Export));
        assert_eq!("public".parse::<Keyword>(), Ok(Keyword::Export));
    }
}

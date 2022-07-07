/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub(crate) mod error;
mod r#macro;

use crate::Kword;

Kword! {
    - Function as fn | func | function

    - Let   as let
    - Mut   as mut
    - Const as const

    - Loop     as loop
    - For      as for
    - While    as while
    - Break    as break
    - Continue as continue
    - If     as if
    - Else   as else
    - Match  as match
    - Return as return

    - Struct    as struct
    - Enum      as enum
    - Interface as interface

    - Impl as impl

    - Export as export
    - Type   as type
    - As     as as
    - From   as from
    - Import as import | use
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

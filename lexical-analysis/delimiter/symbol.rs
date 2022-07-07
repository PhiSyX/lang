/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::delim;

delim! { - Symbol -
    ESCAPE          = '\\';

    AT              = '@';
    DOLLAR          = '$';
    HASH            = '#';
    DOT             = '.';
    RANGE           = "..";
    RANGE_INCLUSIVE = "..=";

    COMMA           = ',';
    DOUBLE_COLON    = "::";
    COLON           = ':';
    SEMICOLON       = ';';
    UNDERSCORE      = '_';
    TILDE           = '`';
    QUESTION_MARK   = '?';

    SKINNY_ARROW    = "->";
    FAT_ARROW       = "=>";
}

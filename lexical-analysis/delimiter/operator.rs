/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

use crate::delim;

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum Operator {
    /// Opérateurs d'assignations
    Assignment(Assignment),

    /// Opérateurs de comparaisons
    Comparison(Comparison),

    /// Opérateurs arithmétiques
    Arithmetic(Arithmetic),

    /// Opérateurs binaires
    Bitwise(Bitwise),

    /// Opérateurs logiques
    Logical(Logical),
}

delim! { - Assignment -
    EQUAL = '=';
    ADDITION = "+=";
    SUBTRACTION = "-=";
    MULTIPLICATION = "*=";
    DIVISION = "/=";
    REMAINDER = "%=";
    EXPONENTIATION = "**=";
    LEFT_SHIFT = "<<=";
    RIGHT_SHIFT = ">>=";
    BITWISE_AND = "&=";
    BITWISE_XOR = "^=";
    BITWISE_OR = "|=";
    LOGICAL_AND = "&&=";
    LOGICAL_OR = "||=";
}

delim! { - Comparison -
    EQUAL = "==";
    NOT_EQUAL = "!=";
    GREATER_THAN = '>';
    GREATER_THAN_OR_EQUAL = ">=";
    LESS_THAN = '<';
    LESS_THAN_OR_EQUAL = "<=";
}

delim! { - Arithmetic -
    ADDITION = '+';
    SUBTRACTION = '-';
    DIVISION = '/';
    MULTIPLICATION = '*';
    REMAINDER = '%';
    EXPONENTIATION = "^^";
}

delim! { - Logical -
    AND = "&&";
    OR = "||";
    NOT = '!';
}

delim! { - Bitwise -
    AND = '&';
    OR = '|';
    XOR = '^';
    NOT = '~';
    LEFT_SHIFT = "<<";
    RIGHT_SHIFT = ">>";
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | Self::Assignment(assignment) => assignment.to_string(),
                | Self::Comparison(cmp) => cmp.to_string(),
                | Self::Arithmetic(arithmetic) => arithmetic.to_string(),
                | Self::Bitwise(bitwise) => bitwise.to_string(),
                | Self::Logical(logical) => logical.to_string(),
            }
        )
    }
}

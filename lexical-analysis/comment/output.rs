/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::{fmt, str};

use location::Location;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub struct CommentOutput {
    /// La donnée, le texte du commentaire.
    pub data: String,

    /// Le type de commentaire.
    pub ty: CommentType,

    /// L'emplacement du commentaire dans le code source.
    pub location: Location,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum CommentType {
    /// Commentaire de type LINE.
    ///
    /// Dans la syntaxe de MikuScript:
    ///     - `// Mon commentaire`.
    ///     - `-- Mon commentaire`.
    Line,

    /// Commentaire de type BLOCK.
    ///
    /// Généralement un commentaire de type BLOCK commence par deux points
    /// de code: U+002F SOLIDUS (/) suivi d'un U+002A ASTERISK (*), et se
    /// termine par deux points de code: U+002A ASTERISK (*) suivi d'un
    /// U+002F SOLIDUS (/). Mais ce n'est pas toujours le cas, par exemple
    /// dans la syntaxe de l'HTML.
    ///
    /// Exemple dans la syntaxe :
    ///     - MikuScript: /* */
    ///     - CSS:        /* */
    ///     - HTML:       <!-- -->
    #[default]
    Block,
}

// -------------- //
// Implémentation //
// -------------- //

impl CommentType {
    pub fn define(&mut self, ty: Self) {
        *self = ty;
    }
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl str::FromStr for CommentType {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input.to_uppercase().as_ref() {
            | "LINE" => Self::Line,
            | "BLOCK" => Self::Block,
            | _ => {
                return Err(
                    r#"Un commentaire de type "LINE" ou "BLOCK" est attendu"#,
                );
            }
        })
    }
}

impl fmt::Display for CommentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                | Self::Line => "LINE",
                | Self::Block => "BLOCK",
            }
        )
    }
}

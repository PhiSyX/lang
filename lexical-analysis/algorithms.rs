/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[cfg(feature = "comment")]
use crate::comment::{CommentOutput, CommentParseError};

// --------- //
// Interface //
// --------- //

pub trait TokenizerAlgorithms {
    #[cfg(feature = "comment")]
    /// Consommer un commentaire.
    ///
    /// Le résumé de l'algorithme, du code, dépend du langage.
    //
    /// Considérons que les commentaires suivants soient de types :
    ///    - `// Mon commentaire`    => CommentType::Line
    ///    - `/* Mon commentaire */` => CommentType::Block
    ///
    /// 1. CommentType::Line :
    ///     Si les deux points de code suivant sont U+002F SOLIDUS (/) suivi
    /// d'un U+002F SOLIDUS (/), nous devons les consommer, ainsi que tous les
    /// points de code suivant jusqu'à ce que nous rencontrons un point de code
    /// U+000A LINE FEED (\n), autrement dit la fin de la ligne.
    ///
    /// 2. CommentType::Block :
    ///   Si les deux points de code suivants sont U+002F SOLIDUS (/) suivi d'un
    /// U+002A ASTERISK (*), nous devons les consommer ainsi que tous les points
    /// de code suivants jusqu'au premier U+002A ASTERISK (*) suivi d'un
    /// U+002F SOLIDUS (/), ou jusqu'à un point de code EOF (End Of File).
    fn consume_comments(&mut self) -> Result<CommentOutput, CommentParseError>;
}

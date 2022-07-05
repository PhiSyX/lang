/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// ----------- //
// Énumération //
// ----------- //

/// État lexical d'un commentaire.
#[derive(Default)]
pub enum CommentState {
    /// L'état de départ d'un commentaire.
    ///
    /// L'état suivant de cet état est :
    ///   - [CommentState::Start].
    #[default]
    Initial,

    /// Début du commentaire.
    ///
    /// L'état suivant de cet état est :
    ///    - [CommentState::Singleline]
    ///    - [CommentState::Multiline]
    Start,

    /// L'état d'un commentaire d'une seule ligne.
    ///
    /// L'état suivant de cet état est :
    ///   - [CommentState::SinglelineEnd],
    Singleline,

    /// L'état de fin de commentaire d'une seule ligne.
    ///
    /// Soit:
    ///    - Le point de code `\n` a été consommé.
    ///
    /// Une [sortie](super::output::CommentOutput) est retournée.
    SinglelineEnd,

    /// L'état d'un commentaire de plusieurs lignes.
    Multiline,
    MultilineAfterFirstCodePoint, // *
    /// L'état de fin de commentaire de plusieurs lignes.
    MultilineEnd,
}

// -------------- //
// Implémentation //
// -------------- //

impl CommentState {
    /// Change l'état actuel du commentaire.
    pub fn switch(&mut self, state: CommentState) {
        *self = state;
    }
}

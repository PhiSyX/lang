/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::ops;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct Queue<Iter, Item> {
    source: Iter,
    temporary_list: Vec<Item>,
    look_ahead_offset: usize,
}

// -------------- //
// Implémentation //
// -------------- //

// Queue
impl<T, I> Queue<T, I> {
    pub fn new(source: T) -> Self {
        Self {
            source,
            temporary_list: Default::default(),
            look_ahead_offset: Default::default(),
        }
    }
}

// &mut Queue
impl<T, I> Queue<T, I>
where
    T: Iterator<Item = I>,
    I: Clone,
{
    fn decrement(&mut self) {
        self.look_ahead_offset = self.look_ahead_offset.saturating_sub(1);
    }

    /// La sortie d'une file d'attente consiste à retirer son premier élément et
    /// à le renvoyer, si la file n'est pas vide, ou à ne rien renvoyer si elle
    /// l'est.
    pub fn dequeue(&mut self) -> Option<I> {
        if self.temporary_list.is_empty() {
            return None;
        }
        Some(self.temporary_list.remove(0))
    }

    /// Remplis la liste temporaire d'un nombre fini d'éléments venant de la
    /// source de l'itération.
    fn fill(&mut self, required_elements: usize) {
        let stored_elements = self.temporary_list.len();
        if stored_elements <= required_elements {
            (stored_elements..=required_elements).for_each(|_| self.enqueue());
        }
    }

    /// Mettre le prochain élément de la source dans la liste temporaire.
    ///
    /// Terme enqueue : "Mettre en file d'attente"
    /// Mettre en file d'attente dans une file d'attente, c'est l'ajouter à
    /// celle-ci.
    pub fn enqueue(&mut self) {
        if let Some(item) = self.source.next() {
            self.temporary_list.push(item);
        }
    }

    pub fn peek_next(&mut self) -> Option<I> {
        self.fill(self.look_ahead_offset);
        self.temporary_list.get(self.look_ahead_offset).cloned()
    }
}

// &mut Queue
impl<T, I> Queue<T, I>
where
    I: Clone,
{
    /// Ajoute un élément au début de la queue.
    ///
    /// Cette fonction est évidemment à utiliser qu'après qu'un élément eut
    /// été consommé.
    pub fn reconsume(&mut self, maybe_last_consumed_element: Option<I>) {
        debug_assert!(maybe_last_consumed_element.is_some());
        let last_consumed_element = maybe_last_consumed_element.unwrap();
        let mut temp = vec![last_consumed_element];
        self.temporary_list.splice(..0, temp.drain(..));
    }
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<T, I> Iterator for Queue<T, I>
where
    T: Iterator<Item = I>,
    I: Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let consumed_or_queued_item = if self.temporary_list.is_empty() {
            self.source.next()
        } else {
            self.dequeue()
        };

        self.decrement();

        consumed_or_queued_item
    }
}

impl<Iter, Item> ops::Deref for Queue<Iter, Item> {
    type Target = Iter;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<Iter, Item> ops::DerefMut for Queue<Iter, Item> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}

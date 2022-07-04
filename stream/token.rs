/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use super::iterator::*;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct TokenStream<Token, Error> {
    list_of_tokens: Vec<Result<Token, Error>>,
    token_currently_being_operated_on: Option<Result<Token, Error>>,
    reconsume_now: bool,
    recycle: Vec<Result<Token, Error>>,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum TokenStreamError {
    EOF,
}

// -------------- //
// Implémentation //
// -------------- //

impl<T, E> TokenStream<T, E>
where
    T: Clone,
    E: Clone,
{
    /// Collection de jetons.
    pub fn collect(&self) -> Vec<Result<T, E>> {
        self.list_of_tokens.to_vec()
    }
}

impl<T, E> TokenStream<T, E>
where
    T: StreamIteratorItem,
{
    pub fn from_stream<Stream>(mut stream: Stream) -> Self
    where
        Stream: StreamIterator<Item = T, Error = E>,
    {
        let mut list_of_tokens = Vec::new();

        loop {
            match stream.consume_next() {
                | Ok(token) if token.is_eof() => break,
                | Ok(token) if token.is_ignored() => continue,
                | token => list_of_tokens.push(token),
            }
        }

        Self {
            list_of_tokens,
            token_currently_being_operated_on: Default::default(),
            reconsume_now: Default::default(),
            recycle: Default::default(),
        }
    }
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<T, E> StreamIterator for TokenStream<T, E>
where
    T: StreamIteratorItem,
    E: StreamIteratorError,
{
    type Error = E;
    type Item = T;

    fn current(&self) -> Option<Self::Item> {
        self.token_currently_being_operated_on
            .clone()
            .and_then(|t| t.ok())
    }

    fn consume_next(&mut self) -> Result<Self::Item, Self::Error> {
        self.reconsume_now = false;

        if self.list_of_tokens.is_empty() {
            self.token_currently_being_operated_on = None;
        } else {
            let next_token = if self.token_currently_being_operated_on.is_none()
            {
                let token = self.list_of_tokens.remove(0);
                self.recycle.push(token.clone());
                token
            } else {
                let current_token = self
                    .token_currently_being_operated_on
                    .clone()
                    .expect("Le jeton actuel 2");

                self.recycle.push(current_token);

                self.list_of_tokens.remove(0)
            };

            self.token_currently_being_operated_on.replace(next_token);
        }

        self.token_currently_being_operated_on
            .clone()
            .expect("Le jeton actuel 3")
    }

    fn peek_next(&mut self) -> Result<Self::Item, Self::Error> {
        if self.reconsume_now {
            return self
                .token_currently_being_operated_on
                .clone()
                .expect("Le jeton actuel 4");
        }

        if self.list_of_tokens.is_empty() {
            return Err(Self::Error::eos());
        }

        self.list_of_tokens
            .iter()
            .peekable()
            .next()
            .cloned()
            .unwrap()
    }

    fn reconsume_current(&mut self) {
        self.reconsume_now = true;
        let last_consumed_element = self
            .token_currently_being_operated_on
            .clone()
            .expect("Le jeton actuel");
        let mut temp = vec![last_consumed_element];
        self.list_of_tokens.splice(..0, temp.drain(..));
    }

    fn recover_as_long_as_possible<
        Predicate: Fn(&Result<Self::Item, Self::Error>) -> bool,
    >(
        &mut self,
        predicate: Predicate,
    ) -> Vec<Result<Self::Item, Self::Error>> {
        let mut result: Vec<Result<Self::Item, Self::Error>> = vec![];

        let recycle = self.recycle.iter().rev();
        for token in recycle {
            if predicate(token) {
                result.push(token.clone());
            } else {
                break;
            }
        }

        let count = result.len();
        let total_recycle = self.recycle.len();

        self.recycle
            .drain(total_recycle - count..self.recycle.len());

        result
    }
}

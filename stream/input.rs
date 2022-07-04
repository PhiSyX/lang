/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use codepoints::{CodePoint, CodePointInterface};

use crate::{prelude::*, Queue};

// ---- //
// Type //
// ---- //

type InputStreamFilterCallback<U> = fn(&U) -> CodePoint<U>;

// --------- //
// Structure //
// --------- //

pub struct InputStream<CodePoints, U> {
    queue: Queue<CodePoints, U>,
    filter_scan_fn: InputStreamFilterCallback<U>,
    current_input: Option<U>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
pub enum InputStreamError {
    EOS,
}

// -------------- //
// Implémentation //
// -------------- //

impl<CodePoints, U> InputStream<CodePoints, U>
where
    U: CodePointInterface,
{
    /// Crée un nouveau flux d'entrée à partir de points de code.
    pub fn new(codepoints: CodePoints) -> Self {
        Self {
            queue: Queue::new(codepoints),
            filter_scan_fn: Self::default_filter_scan,
            current_input: Default::default(),
        }
    }

    /// Défini un fonction de filtre, qui sera appelé à chaque itération.
    pub fn define_filter_scan(
        mut self,
        filter_scan_fn: InputStreamFilterCallback<U>,
    ) -> Self {
        self.filter_scan_fn = filter_scan_fn;
        self
    }

    /// Filtre par défaut. Par défaut, on ignore aucun point de code.
    fn default_filter_scan(ch: &U) -> CodePoint<U> {
        (*ch).into()
    }
}

impl<CodePoints, U> InputStream<CodePoints, U>
where
    U: CodePointInterface,
{
    pub fn meanwhile(&mut self) -> &mut Queue<CodePoints, U> {
        &mut self.queue
    }
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<CodePoints, U> StreamIterator for InputStream<CodePoints, U>
where
    CodePoints: Iterator<Item = U>,
    U: CodePointInterface,
{
    type Error = InputStreamError;
    type Item = CodePoint<U>;

    fn advance_as_long_as_possible_with_limit<
        Predicate: Fn(&Self::Item) -> bool,
    >(
        &mut self,
        predicate: Predicate,
        with_limit: Option<usize>,
    ) -> Vec<Self::Item> {
        let mut limit = with_limit.map(|n| n + 1).unwrap_or(0);
        let mut result = vec![];

        while self.peek_next().is_ok()
            && predicate(self.peek_next().as_ref().unwrap())
            && (limit > 0 || with_limit.is_none())
        {
            let input = self.consume_next().unwrap();
            result.push(input);
            if with_limit.is_some() {
                limit -= 1;
            }
        }

        result
    }

    fn advance_as_long_as_possible_and_apply<
        Predicate: Fn(&Self::Item) -> bool,
        Callback: FnMut(&Self::Item),
    >(
        &mut self,
        predicate: Predicate,
        mut callback: Callback,
    ) -> Vec<Self::Item> {
        let mut result = vec![];

        while self.peek_next().is_ok()
            && predicate(self.peek_next().as_ref().unwrap())
        {
            let input = self.consume_next().unwrap();
            callback(&input);
            result.push(input);
        }

        result
    }

    fn current(&self) -> Option<Self::Item> {
        self.current_input
            .as_ref()
            .map(|codepoint| (self.filter_scan_fn)(codepoint))
    }

    fn consume_next(&mut self) -> Result<Self::Item, Self::Error> {
        let maybe_unit = self.queue.next();
        self.current_input = maybe_unit;
        let maybe_codepoint = maybe_unit
            .map(|codepoint| (self.filter_scan_fn)(&codepoint))
            .unwrap_or(CodePoint::EOF);
        if let CodePoint::Ignore = maybe_codepoint {
            self.consume_next()
        } else {
            Ok(maybe_codepoint)
        }
    }

    fn peek_next(&mut self) -> Result<Self::Item, Self::Error> {
        let peek_next_element = self
            .meanwhile()
            .peek_next()
            .map(|codepoint| (self.filter_scan_fn)(&codepoint))
            .unwrap_or(CodePoint::EOF);

        if let CodePoint::Ignore = peek_next_element {
            self.peek_next()
        } else {
            Ok(peek_next_element)
        }
    }

    fn reconsume_current(&mut self) {
        let own_current_input = self.current_input.to_owned();
        self.queue.reconsume(own_current_input);
    }
}

impl StreamIteratorError for InputStreamError {
    fn eos() -> Self {
        Self::EOS
    }
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "Hello World\n";

    #[test]
    fn test_input_stream_chars() {
        let mut input_stream = InputStream::new(SOURCE.chars());
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('H')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('e')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('l')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('l')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('o')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Whitespace(' ')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('W')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('o')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('r')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('l')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('d')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Newline('\n')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
    }

    #[test]
    fn test_input_stream_bytes() {
        let mut input_stream = InputStream::new(SOURCE.bytes());

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'H')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'e')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'l')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'l')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'o')));

        assert_eq!(
            input_stream.consume_next(),
            Ok(CodePoint::Whitespace(b' '))
        );

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'W')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'o')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'r')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'l')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit(b'd')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Newline(b'\n')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
    }

    #[test]
    fn test_input_stream_with_filter() {
        let mut input_stream = InputStream::new(SOURCE.chars())
            .define_filter_scan(|codepoint: &char| {
                if codepoint.is_whitespace() {
                    return CodePoint::Whitespace(*codepoint);
                } else if codepoint.is_null() {
                    return CodePoint::NULL;
                }

                // Pour X ou Y raison.
                if *codepoint == 'l' {
                    return CodePoint::Ignore;
                }

                CodePoint::Unit(codepoint.to_owned())
            });

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('H')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('e')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('o')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Whitespace(' ')));

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('W')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('o')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('r')));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::Unit('d')));

        assert_eq!(
            input_stream.consume_next(),
            Ok(CodePoint::Whitespace('\n'))
        );

        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
        assert_eq!(input_stream.consume_next(), Ok(CodePoint::EOF));
    }
}

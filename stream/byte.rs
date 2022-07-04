/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::{fmt, str};
use std::{
    fs,
    io::{self, BufReader, Read},
};

// --------- //
// Structure //
// --------- //

/// Le flux de points de code qui constitue l'entrée de l'étape de
/// tokenisation sera initialement vu par l'agent utilisateur comme un flux
/// d'octets (provenant généralement du réseau ou du système de fichiers
/// local). Les octets codent les caractères réels selon un codage de
/// caractères particulier, que l'agent utilisateur utilise pour décoder
/// les octets en caractères.
#[derive(Debug)]
pub struct ByteStream {
    buffer: String,
}

// -------------- //
// Implémentation //
// -------------- //

impl ByteStream {
    pub fn new(buffer: String) -> Self {
        Self { buffer }
    }
}

impl ByteStream {
    /// Octets de la chaîne de caractères.
    pub fn bytes(&self) -> &[u8] {
        self.buffer.as_bytes()
    }

    /// Liste des caractères de la chaîne de caractères.
    pub fn chars(&self) -> str::Chars {
        self.buffer.chars()
    }
}

// -------------- //
// Implémentation // -> Display
// -------------- //

impl fmt::Display for ByteStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.buffer)
    }
}

// -------------- //
// Implémentation // -> From<T>
// -------------- //

/// Crée un nouveau [ByteStream] à partir d'octets (provenant
/// du réseau/fichier).

impl<const N: usize> From<&[u8; N]> for ByteStream {
    fn from(buf_bytes: &[u8; N]) -> Self {
        let decoded_data = str::from_utf8(buf_bytes)
            .map(|data| data.to_owned())
            .unwrap_or_default();
        Self::new(decoded_data)
    }
}

impl From<&[u8]> for ByteStream {
    fn from(buf_bytes: &[u8]) -> Self {
        let decoded_data = str::from_utf8(buf_bytes)
            .map(|data| data.to_owned())
            .unwrap_or_default();
        Self::new(decoded_data)
    }
}

impl<T> From<BufReader<T>> for ByteStream {
    /// Crée un nouveau [ByteStream] à partir d'octets (provenant
    /// du réseau/fichier).
    fn from(buf_reader: BufReader<T>) -> Self {
        Self::from(buf_reader.buffer())
    }
}

impl From<&str> for ByteStream {
    /// Crée un nouveau [ByteStream] à partir d'une slice string.
    ///
    /// Les chaînes de caractères slices sont toujours valides UTF-8.
    fn from(slice_str: &str) -> Self {
        Self::new(slice_str.to_owned())
    }
}

impl TryFrom<Result<fs::File, io::Error>> for ByteStream {
    type Error = io::Error;

    fn try_from(
        maybe_file: Result<fs::File, io::Error>,
    ) -> Result<Self, Self::Error> {
        maybe_file.map(|mut file| {
            let mut buf = vec![];
            let n = file.read_to_end(&mut buf).expect("a string");
            Self::from(&buf[..n])
        })
    }
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let bytes: &[u8; 13] = b"Hello, world!";
        let stream = ByteStream::from(bytes);
        let mut chars = stream.chars();
        assert_eq!(chars.next(), Some('H'));
        assert_eq!(chars.last(), Some('!'));
    }

    #[test]
    fn test_from_str() {
        let source: &'static str = include_str!("testdata/file.ms");
        let stream = ByteStream::from(source);
        let mut chars = stream.chars();
        assert_eq!(chars.next(), Some('/'));
        assert_eq!(chars.next(), Some('*'));
        assert_eq!(chars.last(), Some('\n'));
    }

    #[test]
    fn test_from_file() {
        let file = fs::File::open("./testdata/file.ms");
        let stream = ByteStream::try_from(file).expect("le fichier");
        let mut chars = stream.chars();
        assert_eq!(chars.next(), Some('/'));
        assert_eq!(chars.next(), Some('*'));
        assert_eq!(chars.last(), Some('\n'));
    }
}

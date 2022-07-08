/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use codepoints::{CodePoint, CodePointInterface};

// --------- //
// Interface //
// --------- //

pub trait StreamIterator {
	type Error: StreamIteratorError;
	type Item: StreamIteratorItem;

	/// Avance dans le flux autant que possible, tant que le prédicat est
	/// vrai.
	///
	/// Exemple:
	///
	/// Le flux d'entrée de départ vaut `[' ', ' ', ' ', 'a', ' ', 'b']`;
	/// On veut avancer dans le flux, tant que le caractère suivant est un
	/// espace.
	///
	/// Code:
	///     advance_as_long_as_possible(|next_ch| next_ch.is_whitespace());
	///
	/// Après cette opération, le flux d'entrée vaut `['a', ' ', 'b']`.
	#[allow(unused_variables)]
	fn advance_as_long_as_possible<Predicate: Fn(&Self::Item) -> bool>(
		&mut self,
		predicate: Predicate,
	) -> Vec<Self::Item> {
		self.advance_as_long_as_possible_with_limit(predicate, None)
	}

	/// Alias de [StreamIterator::advance_as_long_as_possible] avec une
	/// limite.
	#[allow(unused_variables)]
	fn advance_as_long_as_possible_with_limit<
		Predicate: Fn(&Self::Item) -> bool,
	>(
		&mut self,
		predicate: Predicate,
		with_limit: Option<usize>,
	) -> Vec<Self::Item> {
		unimplemented!(
			"Avance dans le flux autant que possible, tant que le prédicat \
            est vrai."
		);
	}

	/// Avance dans le flux autant que possible, tant que le prédicat est
	/// vrai et applique une fonction de retour.
	#[allow(unused_variables)]
	fn advance_as_long_as_possible_and_apply<
		Predicate: Fn(&Self::Item) -> bool,
		Callback: FnMut(&Self::Item),
	>(
		&mut self,
		predicate: Predicate,
		callback: Callback,
	) -> Vec<Self::Item> {
		unimplemented!(
			"Avance dans le flux autant que possible, tant que le prédicat \
            est vrai et applique une fonction de retour."
		);
	}

	/// La dernière entrée d'un flux à avoir été consommée.
	fn current(&self) -> Option<Self::Item>;

	/// Consomme le prochain élément d'un flux. Autrement dit, le premier
	/// élément d'un flux.
	fn consume_next(&mut self) -> Result<Self::Item, Self::Error>;

	/// Le prochain élément d'un flux. Ne DOIT PAS consommer l'itérateur.
	/// Autrement dit, la valeur de retour du prochain appel à la fonction
	/// [StreamIterator::consume_next] DOIT être la valeur de retour de cette
	/// fonction.
	fn peek_next(&mut self) -> Result<Self::Item, Self::Error>;

	/// Les prochains éléments d'un flux. Ne DOIT PAS consommer l'itérateur.
	fn peek_n_next(&mut self, n: usize)
		-> Vec<Result<Self::Item, Self::Error>>;

	/// Pousse (l'entrée actuelle](Self::current_input) à l'avant d'un
	/// flux, de sorte que la prochaine fois qu'il sera demandée de
	/// consommer l'entrée suivante, il reprendra plutôt l'entrée actuelle.
	fn reconsume_current(&mut self);

	/// Récupère des éléments mis dans un flux ("corbeille") autant que
	/// possible, tant que le prédicat est vrai.
	#[allow(unused_variables)]
	fn recover_as_long_as_possible<
		Predicate: Fn(&Result<Self::Item, Self::Error>) -> bool,
	>(
		&mut self,
		predicate: Predicate,
	) -> Vec<Result<Self::Item, Self::Error>> {
		unimplemented!(
			r#"Récupère des éléments mis dans un flux ("corbeille") autant \
               que possible, tant que le prédicat est vrai."#
		)
	}
}

pub trait StreamIteratorError:
	PartialEq + Eq + Clone + ::core::fmt::Debug
{
	/// Un jeton conceptuel représentant la fin d'un flux.
	fn eos() -> Self;

	/// Vérifie si le jeton actuel est un jeton de fin de flux.
	fn is_eos(&self) -> bool {
		*self == Self::eos()
	}
}

pub trait StreamIteratorItem:
	PartialEq + Eq + Clone + core::fmt::Debug
{
	type Kind: PartialEq + Eq + Clone + core::fmt::Debug;

	/// Un jeton conceptuel représentant la fin d'un flux.
	/// Lorsque la liste d'un flux est vide, le prochain jeton d'entrée est
	/// toujours un `EOF`.
	fn eof() -> Self::Kind;

	/// Utilisé pour filtrer les éléments dans les itérations.
	fn ignore() -> Self::Kind;

	fn kind(&self) -> &Self::Kind;

	/// Vérifie si le jeton actuel est un jeton de fin de fichier.
	fn is_eof(&self) -> bool {
		*self.kind() == Self::eof()
	}

	fn is_ignored(&self) -> bool {
		*self.kind() == Self::ignore()
	}
}

pub trait StreamIteratorQueue {}

// ------------------------------------------------------------------- //
// Implémentation par défaut de StreamIteratorItem pour certains types //
// ------------------------------------------------------------------- //

impl<T> StreamIteratorItem for CodePoint<T>
where
	T: CodePointInterface,
{
	type Kind = CodePoint<T>;

	fn eof() -> Self::Kind {
		Self::Kind::EOF
	}

	fn ignore() -> Self::Kind {
		Self::Kind::Ignore
	}

	fn kind(&self) -> &Self::Kind {
		self
	}
}

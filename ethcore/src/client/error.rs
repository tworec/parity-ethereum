// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::fmt::{Display, Formatter, Error as FmtError};
use std::io;
use ethtrie::TrieError;

/// Client configuration errors.
#[derive(Debug)]
pub enum Error {
	/// TrieDB-related error.
	Trie(TrieError),
	/// Io error.
	Io(io::Error),
}

impl From<TrieError> for Error {
	fn from(err: TrieError) -> Self {
		Error::Trie(err)
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Error::Io(err)
	}
}

impl<E> From<Box<E>> for Error where Error: From<E> {
	fn from(err: Box<E>) -> Self {
		Error::from(*err)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
		match *self {
			Error::Trie(ref err) => write!(f, "{}", err),
			Error::Io(ref err) => write!(f, "{}", err),
		}
	}
}

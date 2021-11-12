use std::hash::Hash;

use chumsky::prelude::*;

pub(crate) trait ParserExt<I, O> {}

impl<T: Parser<I, O, Error = E>, I: Clone + Eq + Hash, O, E> ParserExt<I, O> for T {}

use std::fmt::Display;

use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher, MatcherExt},
};

/// Minimal matcher that trys to match exactly one symbol.
pub struct Symbol<S>(S);

impl<I, S> Matcher<I> for Symbol<S>
where
    I: Input<Symbol = S>,
    S: Copy + Eq + Display,
{
    fn try_match<'a>(&self, mut cursor: Cursor<'a, I>) -> MatchResult<'a, I> {
        match cursor.peek() {
            Some(symbol) if symbol == self.0 => {
                cursor.advance();
                MatchResult::Matched(cursor)
            }
            Some(symbol) => {
                cursor.advance();
                MatchResult::Failed(
                    cursor,
                    format!(
                        "Unexpected symbol `{}`, expected `{}` instead.",
                        symbol, self.0
                    ),
                )
            }
            None => {
                cursor.advance();
                MatchResult::Failed(cursor, "Unexpected end of input.".into())
            }
        }
    }
}

impl<S> MatcherExt for Symbol<S> {}

/// Function to create a symbol matcher.
pub const fn symbol<S>(symbol: S) -> Symbol<S> {
    Symbol(symbol)
}

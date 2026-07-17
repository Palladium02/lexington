use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher},
};

/// A trait that enables the usage of just over a chain of .then calls by converting
/// the argument passed to just into an iterator of symbols.
pub trait IntoSymbols<S> {
    type Iter: Iterator<Item = S>;

    fn into_symbols(self) -> Self::Iter;
}

impl<'a> IntoSymbols<char> for &'a str {
    type Iter = std::str::Chars<'a>;

    fn into_symbols(self) -> Self::Iter {
        self.chars()
    }
}

pub struct Just<S>(S);

impl<I: Input, S: Clone + IntoSymbols<I::Symbol>> Matcher<I> for Just<S> {
    fn try_match<'a>(&self, mut cursor: Cursor<'a, I>) -> MatchResult<'a, I> {
        for expected in self.0.clone().into_symbols() {
            match cursor.peek() {
                Some(actual) if actual == expected => cursor.advance(),
                Some(actual) => {
                    cursor.advance();
                    return MatchResult::Failed(
                        cursor,
                        format!(
                            "Unexpected symbol `{}`, expected `{}` instead.",
                            actual, expected
                        ),
                    );
                }
                None => {
                    cursor.advance();
                    return MatchResult::Failed(cursor, "Unexpected end of input.".into());
                }
            }
        }
        MatchResult::Matched(cursor)
    }
}

pub fn just<S>(s: S) -> Just<S> {
    Just(s)
}

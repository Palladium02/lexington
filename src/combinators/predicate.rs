use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{self, MatchResult, Matcher, MatcherBase},
};

pub struct Predicate<P>(P);

impl<P> Predicate<P> {
    pub fn new(predicate: P) -> Self {
        Self(predicate)
    }
}

impl<I: Input, P: Fn(I::Symbol) -> bool> Matcher<I> for Predicate<P> {
    fn try_match<'a>(&self, mut cursor: Cursor<'a, I>) -> matcher::MatchResult<'a, I> {
        match cursor.peek() {
            Some(symbol) if self.0(symbol) => {
                cursor.advance();
                MatchResult::Matched(cursor)
            }
            Some(symbol) => {
                cursor.advance();
                MatchResult::Failed(cursor, format!("Predicate failed on symbol `{}`.", symbol))
            }
            None => {
                cursor.advance();
                MatchResult::Failed(cursor, "Unexpected end of input.".into())
            }
        }
    }
}

impl<P> MatcherBase for Predicate<P> {}

pub fn predicate<P>(predicate: P) -> Predicate<P> {
    Predicate(predicate)
}

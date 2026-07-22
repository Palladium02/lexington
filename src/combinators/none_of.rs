use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher, MatcherExt},
};

pub struct NoneOf<S>(S);

impl<I, S> Matcher<I> for NoneOf<S>
where
    I: Input,
    S: AsRef<[I::Symbol]>,
{
    fn try_match<'a>(&self, mut cursor: Cursor<'a, I>) -> MatchResult<'a, I> {
        let Some(actual) = cursor.peek() else {
            return MatchResult::Failed(cursor, "Unexpected end of input.".into());
        };

        if self.0.as_ref().iter().all(|&expected| actual != expected) {
            cursor.advance();
            MatchResult::Matched(cursor)
        } else {
            MatchResult::Failed(cursor, "".into())
        }
    }
}

impl<S> MatcherExt for NoneOf<S> {}

use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher, MatcherExt},
};

pub struct OneOf<S> {
    symbols: S,
}

impl<I, S> Matcher<I> for OneOf<S>
where
    I: Input,
    S: AsRef<[I::Symbol]>,
{
    fn try_match<'a>(&self, mut cursor: Cursor<'a, I>) -> MatchResult<'a, I> {
        for symbol in self.symbols.as_ref() {
            match cursor.peek() {
                Some(actual) if actual == *symbol => {
                    cursor.advance();
                    return MatchResult::Matched(cursor);
                }
                Some(_) => continue,
                None => return MatchResult::Failed(cursor, "Unexpected end of input.".into()),
            }
        }

        MatchResult::Failed(cursor, "".into())
    }
}

impl<S> MatcherExt for OneOf<S> {}

pub fn one_of<I: Input, S: AsRef<[I::Symbol]>>(symbols: S) -> OneOf<S> {
    OneOf { symbols }
}

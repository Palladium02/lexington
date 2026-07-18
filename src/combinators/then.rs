use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher, MatcherBase},
};

/// Chaining matcher that succeeds if left and right succeed in order.
pub struct Then<A, B> {
    left: A,
    right: B,
}

impl<A, B> Then<A, B> {
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

impl<I: Input, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Then<A, B> {
    fn try_match<'a>(&self, cursor: Cursor<'a, I>) -> MatchResult<'a, I> {
        match self.left.try_match(cursor) {
            MatchResult::Matched(cursor) => match self.right.try_match(cursor) {
                MatchResult::Matched(cursor) => MatchResult::Matched(cursor),
                MatchResult::Failed(cursor, message) => MatchResult::Failed(cursor, message),
            },
            MatchResult::Failed(cursor, message) => MatchResult::Failed(cursor, message),
        }
    }
}

impl<A, B> MatcherBase for Then<A, B> {}

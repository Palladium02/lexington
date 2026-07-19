use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher, MatcherBase},
};

pub struct Or<A, B> {
    left: A,
    right: B,
}

impl<A, B> Or<A, B> {
    pub const fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

impl<I: Input, A: Matcher<I>, B: Matcher<I>> Matcher<I> for Or<A, B> {
    fn try_match<'a>(&self, cursor: Cursor<'a, I>) -> MatchResult<'a, I> {
        match self.left.try_match(cursor) {
            MatchResult::Matched(cursor) => MatchResult::Matched(cursor),
            MatchResult::Failed(_, left_message) => match self.right.try_match(cursor) {
                MatchResult::Matched(cursor) => MatchResult::Matched(cursor),
                MatchResult::Failed(cursor, right_message) => {
                    let mut message = String::new();
                    message.push_str(&left_message);
                    message.push('\n');
                    message.push_str(&right_message);
                    MatchResult::Failed(cursor, message)
                }
            },
        }
    }
}

impl<A, B> MatcherBase for Or<A, B> {}

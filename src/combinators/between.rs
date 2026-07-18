use std::ops::{Bound, RangeBounds};

use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher, MatcherBase},
};

pub struct Between<M, R> {
    matcher: M,
    bounds: R,
}

impl<M, R> Between<M, R> {
    pub fn new(matcher: M, bounds: R) -> Self {
        Self { matcher, bounds }
    }
}

impl<I: Input, M, R> Matcher<I> for Between<M, R>
where
    M: Matcher<I>,
    R: RangeBounds<usize>,
{
    fn try_match<'a>(&self, cursor: Cursor<'a, I>) -> MatchResult<'a, I> {
        let min = match self.bounds.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => 0,
        };

        let max = match self.bounds.end_bound() {
            Bound::Included(&n) => Some(n),
            Bound::Excluded(&n) => Some(n - 1),
            Bound::Unbounded => None,
        };

        let mut matches = 0;
        let mut cursor = cursor;

        loop {
            if let Some(max) = max
                && max == matches
            {
                return MatchResult::Matched(cursor);
            }

            match self.matcher.try_match(cursor) {
                MatchResult::Matched(new_cursor) => {
                    matches += 1;
                    cursor = new_cursor;
                }
                MatchResult::Failed(_new_cursor, _) => {
                    // cursor = new_cursor;
                    break;
                }
            };
        }

        if matches < min {
            return MatchResult::Failed(
                cursor,
                format!(
                    "Expected to match at least {} times, matched {} times instead",
                    min, matches
                ),
            );
        }

        MatchResult::Matched(cursor)
    }
}

impl<M, R> MatcherBase for Between<M, R> {}

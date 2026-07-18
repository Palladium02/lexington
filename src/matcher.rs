use crate::{
    combinators::{between::Between, or::Or, then::Then},
    cursor::Cursor,
    input::Input,
    rule::{Action, Rule},
};

/// The result of attempting to match input.
///
/// A matcher either succeeds and returns the cursor after the matched input,
/// or fails and return the cursor describing how far matching progressed.
pub enum MatchResult<'a, I: Input> {
    /// The matcher successfully consumed input.
    Matched(Cursor<'a, I>),
    /// The matcher failed.
    ///
    /// The returned cursor may be further advanced than the starting cursor,
    /// allowing called to provide better diagnostics.
    Failed(Cursor<'a, I>, String), // Failed.1 is the reason for why it failed.
}

/// A component capable of recognizing a sequence of symbols.
///
/// Matchers are the building blocks of lexer rules. They describe *what input
/// to recognize* but do not decide what token should be produced.
pub trait Matcher<I: Input> {
    /// Attempts to match input starting at the provided cursor.
    ///
    /// The cursor is passed by value, allowing matchers to freely consume it
    /// without affecting the caller unless the match succeeds.
    fn try_match<'a>(&self, cursor: Cursor<'a, I>) -> MatchResult<'a, I>;
}

pub trait MatcherBase {}

pub trait MatcherExt: Sized + 'static {
    /// Creates a matcher that succeeds if the two underlying matchers succeed in order.
    fn then<A>(self, other: A) -> Then<Self, A> {
        Then::new(self, other)
    }

    /// Creates a matcher that tests whether two matchers succeeds. A match is chosen in a short-circuiting manner.
    fn or<A>(self, other: A) -> Or<Self, A> {
        Or::new(self, other)
    }

    /// Creates a matcher which is executed multiple times to satisfy the given range bounds. Matches are computed greedily.
    fn between<A>(self, bounds: A) -> Between<Self, A> {
        Between::new(self, bounds)
    }

    /// Captures the matcher into a rule that when chosen by the lexer will result in a token
    /// being produced.
    fn kind<K: Copy>(self, kind: K) -> Rule<Self, K>;

    /// Captures the matcher into a rule that when chose by the lexer will result in no token being
    /// produced for this rule.
    fn skip<K: Copy>(self) -> Rule<Self, K>;
}

impl<M: MatcherBase> MatcherExt for M
where
    M: Sized + 'static,
{
    fn kind<K: Copy>(self, kind: K) -> Rule<Self, K> {
        Rule::new(self, Action::Emit(kind))
    }

    fn skip<K: Copy>(self) -> Rule<Self, K> {
        Rule::new(self, Action::Skip)
    }
}

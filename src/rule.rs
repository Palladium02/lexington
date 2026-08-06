use std::marker::PhantomData;

use crate::{
    cursor::Cursor,
    input::Input,
    matcher::{MatchResult, Matcher},
};

/// An action performed when a rule matches.
///
/// Actions determine whether a successful match produces a token or is ignored.
#[derive(Clone, Copy)]
pub enum Action<K: Copy> {
    /// Produce a token with the given kind.
    Emit(K),
    /// Consume the match input without producing a token.
    Skip,
}

/// A lexer rule connecting a matcher with an action.
///
/// Rules define how recognized input is handled by the lexer.
///
/// A rule consists of:
/// - a matcher deciding whether input matches
/// - an action describing what happens after a successful match
/// - a name for better error reporting
pub struct Rule<'a, M, K: Copy> {
    matcher: M,
    action: Action<K>,
    name: String,
    _marker: PhantomData<&'a ()>,
}

/// The result of attempting to apply a rule to an input.
///
/// A rule is either applied successfully and returns an action and the advanced cursor,
/// or fails and returns the cursor describing how far the underlying matcher got together
/// with an error message.
pub enum RuleResult<'a, I: Input, K: Copy> {
    Applied(Action<K>, Cursor<'a, I>),
    Failed(String, Cursor<'a, I>), // Failed.0 is the reason bubbled up from MatchResult::Failed extended with "{Rule.name}: "
}

impl<'a, M, K: Copy> Rule<'a, M, K> {
    pub const fn new(matcher: M, action: Action<K>) -> Self {
        Self {
            matcher,
            action,
            name: String::new(),
            _marker: PhantomData,
        }
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
}

impl<'a, M, K: Copy> Rule<'a, M, K> {
    pub fn erase<I>(self) -> ErasedRule<'a, I, K>
    where
        I: Input,
        M: Matcher<I> + 'a,
    {
        ErasedRule {
            matcher: Box::new(self.matcher),
            action: self.action,
            name: self.name,
        }
    }
}

pub struct ErasedRule<'a, I, K: Copy> {
    matcher: Box<dyn Matcher<I> + 'a>,
    action: Action<K>,
    name: String,
}

impl<'a, I: Input, K: Copy> ErasedRule<'a, I, K> {
    pub fn try_apply<'b>(&self, cursor: Cursor<'b, I>) -> RuleResult<'b, I, K> {
        match self.matcher.try_match(cursor) {
            MatchResult::Matched(cursor) => RuleResult::Applied(self.action, cursor),
            MatchResult::Failed(cursor, message) => {
                RuleResult::Failed(format!("{}: {}", &self.name, message), cursor)
            }
        }
    }
}

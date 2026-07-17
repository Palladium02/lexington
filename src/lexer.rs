use std::fmt::Debug;

use crate::{
    cursor::Cursor,
    input::Input,
    matcher::Matcher,
    output::{Span, Token},
    recovery::Recovery,
    rule::{Action, ErasedRule, Rule, RuleResult},
};

/// An event emitted by the lexer.
///
/// A lexer can emit either successfully recognized tokens or errors produced
/// during lexing.
#[derive(Debug)]
pub enum Event<K: Debug> {
    Token(Token<K>),
    Error(Span, String),
}

/// A streaming lexer over an input source.
///
/// The lexer applies registered rules at the current cursor position and emits
/// events as input is consumed.
///
/// Rules are tested whenever the lexer advances. If multiple rules match, the
/// lexer selects the best match according to its configured selection strategy.
pub struct Lexer<'a, I: Input, K: Copy> {
    cursor: Cursor<'a, I>,
    rules: Vec<ErasedRule<I, K>>,
    recovery: Recovery,
}

impl<'a, I: Input, K: Copy> Lexer<'a, I, K> {
    pub fn new(rules: Vec<ErasedRule<I, K>>, cursor: Cursor<'a, I>, recovery: Recovery) -> Self {
        Self {
            rules,
            cursor,
            recovery,
        }
    }

    /// Returns an builder instance.
    pub fn builder() -> LexerBuilder<I, K> {
        LexerBuilder::new()
    }
}

impl<'a, I: Input, K: Debug + Copy> Iterator for Lexer<'a, I, K> {
    type Item = Event<K>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.eof() {
            return None;
        }

        let start = self.cursor;
        let mut best_match = None;
        for rule in self.rules.iter() {
            let result = rule.try_apply(self.cursor);
            if best_match.is_none() {
                best_match = Some(result);
                continue;
            }

            match result {
                RuleResult::Applied(_, result_cursor) => match best_match {
                    Some(RuleResult::Failed(_, _)) => best_match = Some(result),
                    Some(RuleResult::Applied(_, best_cursor))
                        if result_cursor.offset() > best_cursor.offset() =>
                    {
                        best_match = Some(result)
                    }
                    _ => {}
                },
                RuleResult::Failed(_, result_cursor) => match best_match {
                    Some(RuleResult::Failed(_, best_cursor))
                        if result_cursor.offset() > best_cursor.offset() =>
                    {
                        best_match = Some(result)
                    }
                    _ => {}
                },
            }
        }

        match best_match.expect("This should always be a Some-value") {
            RuleResult::Applied(action, cursor) => {
                self.cursor = cursor;
                match action {
                    Action::Emit(kind) => Some(Event::Token(Token::new(
                        kind,
                        Span::new(start.offset(), cursor.offset()),
                    ))),
                    Action::Skip => self.next(),
                }
            }
            RuleResult::Failed(message, cursor) => {
                match self.recovery {
                    Recovery::ConsumeUntilError => self.cursor = cursor,
                    Recovery::ConsumeOneRestart => self.cursor.advance(),
                }
                // self.cursor = cursor;
                Some(Event::Error(
                    Span::new(start.offset(), self.cursor.offset()),
                    message,
                ))
            }
        }
    }
}

/// Builder for constructing a [`Lexer`].
///
/// The builder provides a fluent API for registering rules before creating
/// the final lexer instance.
pub struct LexerBuilder<I: Input, K: Copy> {
    rules: Vec<ErasedRule<I, K>>,
    recovery: Recovery,
}

impl<I: Input, K: Copy> LexerBuilder<I, K> {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            recovery: Recovery::ConsumeUntilError,
        }
    }

    /// Adds a rule with the given name to the set of existing rules.
    pub fn rule<M>(mut self, name: impl Into<String>, mut rule: Rule<M, K>) -> Self
    where
        M: Matcher<I> + 'static,
    {
        rule.set_name(name);
        self.rules.push(rule.erase());
        self
    }

    /// Let's the user override the default recovery strategy.
    pub fn recovery(mut self, recovery: Recovery) -> Self {
        self.recovery = recovery;
        self
    }

    /// Assembles the collected rules together with a given input into a lexer.
    pub fn build<'a>(self, input: &'a I) -> Lexer<'a, I, K> {
        Lexer::new(self.rules, Cursor::new(input, 0), self.recovery)
    }
}

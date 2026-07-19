use crate::{
    combinators::predicate::predicate,
    input::Input,
    matcher::{Matcher, MatcherBase},
};

#[must_use]
pub fn digit<I>() -> impl Matcher<I> + MatcherBase + Sized + 'static
where
    I: Input<Symbol = char> + Sized + 'static,
{
    predicate(|symbol: char| symbol.is_ascii_digit())
}

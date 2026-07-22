#[allow(deprecated)]
// Suppressing the warning on digit::digit for the time being until it's removed
pub use crate::combinators::{
    ascii::{
        ascii_alpha, ascii_alphanumeric, ascii_digit, ascii_hex, ascii_lower, ascii_upper,
        ascii_whitespace,
    },
    digit::digit,
    just::{IntoSymbols, Just, just},
    one_of::{OneOf, one_of},
    symbol::{Symbol, symbol},
    then::Then,
};
pub use crate::input::{Input, Utf8};
pub use crate::lexer::{Event, Lexer, LexerBuilder};
pub use crate::matcher::{Matcher, MatcherExt};
pub use crate::output::{Span, Token};

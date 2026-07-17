//! Lexington is a lexer combinator library.
//!
//! The entry point is [`Lexer::builder`].
//!
//! ```
//! use lexington::prelude::*;
//!
//! #[derive(Debug, Clone, Copy)]
//! enum Kind {
//!     Let,
//!     Identifier,
//!     Int,
//!     Eq,
//!     Semicolon,
//! }
//!
//! fn main() {
//! let lexer = Lexer::builder()
//!         .rule("let", just("let").kind(Kind::Let))
//!         .rule(
//!             "identifier",
//!             ascii_lower().between(1..).kind(Kind::Identifier),
//!         )
//!         .rule("integer", digit().between(1..).kind(Kind::Int))
//!         .rule("equals", symbol('=').kind(Kind::Eq))
//!         .rule("semicolon", symbol(';').kind(Kind::Semicolon))
//!         .rule("space", symbol(' ').skip())
//!         .build(&Utf8("let x = 42;"));
//!
//! let events = lexer.collect::<Vec<Event<Kind>>>();
//! println!("{events:?}")
//! }
//! ```

pub mod combinators;
pub mod cursor;
pub mod input;
pub mod lexer;
pub mod matcher;
pub mod output;
pub mod prelude;
pub mod recovery;
pub mod rule;

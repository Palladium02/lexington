use lexington::prelude::*;

#[test]
fn between_does_not_consume_failed_match() {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Kind {
        Let,
        Identifier,
        Int,
        Eq,
        Semicolon,
    }

    let lexer = Lexer::builder()
        .rule("let", just("let").kind(Kind::Let))
        .rule(
            "identifier",
            ascii_lower().between(1..).kind(Kind::Identifier),
        )
        .rule("integer", ascii_digit().between(1..).kind(Kind::Int))
        .rule("equals", symbol('=').kind(Kind::Eq))
        .rule("semicolon", symbol(';').kind(Kind::Semicolon))
        .rule("space", symbol(' ').skip())
        .build(&Utf8("let x = 42;"));

    let kinds = lexer
        .filter_map(|event| match event {
            Event::Token(token) => Some(token.kind()),
            Event::Error(_, _) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(
        kinds,
        vec![
            Kind::Let,
            Kind::Identifier,
            Kind::Eq,
            Kind::Int,
            Kind::Semicolon
        ]
    )
}

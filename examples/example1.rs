use lexington::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Kind {
    A,
    Let,
    Digit,
    ManyA,
}

fn main() {
    let mut lexer = Lexer::builder()
        .rule("a", symbol('a').kind(Kind::A))
        .rule("ab", symbol('a').then(symbol('b')).skip())
        .rule("let", just("let").kind(Kind::Let))
        .rule("space", symbol(' ').skip())
        .rule("digit", ascii_digit().kind(Kind::Digit))
        .rule("range of a", symbol('a').between(1..10).kind(Kind::ManyA))
        .build(&Utf8("le let 1 aaaaaaaaa"));

    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
    println!("{:?}", lexer.next());
}

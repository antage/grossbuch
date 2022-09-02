use std::path::Path;

use chumsky::prelude::*;
use chumsky::Stream;

use crate::ast::span::{Span, Spanned};
use crate::ast::token::Token;

pub fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char, Span>> {
    let float_num =
        text::digits(10)
            .chain::<char, _, _>(
                just('.')
                    .chain(
                        text::digits(10)
                    )
            )
            .collect()
            .map(Token::Float)
            .labelled("float token");
    let int_num =
        text::digits(10)
            .map(Token::Int)
            .labelled("int token");

    let str_ =
        just('"')
            .ignore_then(
                filter(|c| *c != '"')
                    .repeated()
            )
            .then_ignore(just('"'))
            .collect()
            .map(Token::Str)
            .labelled("string token");

    let minus =
        just('-')
            .ignored()
            .to(Token::Minus)
            .labelled("minus token");

    let slash =
        just('/')
            .ignored()
            .to(Token::Slash)
            .labelled("slash token");

    let semicolon =
        just(";")
            .ignored()
            .to(Token::Semicolon)
            .labelled("semicolon");

    let left_curly_par =
        just("{")
            .ignored()
            .to(Token::LeftCurlyPar)
            .labelled("left curly par");

    let right_curly_par =
        just("}")
            .ignored()
            .to(Token::RightCurlyPar)
            .labelled("right curly par");

    let left_sq_par =
        just("[")
            .ignored()
            .to(Token::LeftSqPar)
            .labelled("left square par");

    let right_sq_par =
        just("]")
            .ignored()
            .to(Token::RightSqPar)
            .labelled("right square par");

    let left_thin_arrow =
        just("<-")
            .ignored()
            .to(Token::LeftThinArrow)
            .labelled("left thin arrow");

    let right_thin_arrow =
        just("->")
            .ignored()
            .to(Token::RightThinArrow)
            .labelled("right thin arrow");

    let kw_import =
        just("import")
            .ignored()
            .to(Token::KwImport)
            .labelled("import");

    let kw_commodity =
        just("commodity")
            .ignored()
            .to(Token::KwCommodity)
            .labelled("commodity");

    let kw_account =
        just("account")
            .ignored()
            .to(Token::KwAccount)
            .labelled("account");

    let colon_ident =
        text::ident()
            .chain(
                just(":")
                    .ignore_then(
                        text::ident()
                    )
                    .repeated().at_least(1)
            )
            .map(|parts| {
                Token::ColonIdent(parts)
            })
            .labelled("colon ident token");

    let ident =
        text::ident()
            .map(Token::Ident)
            .labelled("ident token");

    let comment_oneline =
        just("//")
            .then_ignore(
                take_until(
                    text::newline()
                    .or(end())
                )
            )
            .ignored()
            .padded();

    let comment_multiline =
        just("/*")
            .then_ignore(take_until(just("*/")))
            .ignored()
            .padded();

    let token =
        float_num
            .or(int_num)
            .or(str_)
            .or(semicolon)
            .or(left_curly_par)
            .or(right_curly_par)
            .or(left_sq_par)
            .or(right_sq_par)
            .or(left_thin_arrow)
            .or(right_thin_arrow)
            .or(minus)
            .or(slash)
            .or(kw_import)
            .or(kw_commodity)
            .or(kw_account)
            .or(colon_ident)
            .or(ident);

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment_oneline.or(comment_multiline).repeated())
        .padded()
        .repeated()
}

pub fn token_stream<'a, S>(filename: S, src: &str, toks: Vec<(Token, Span)>) -> Stream<'a, Token, Span, Box<dyn Iterator<Item = (Token, Span)> + 'a>>
where S: AsRef<Path> {
    let len = src.chars().count();
    let eoi = Span::new(filename, len..len+1);
    Stream::from_iter(eoi, Box::new(toks.into_iter()))
}

use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn str() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just('"')
        .ignore_then(
            filter(|c| *c != '"')
                .repeated()
        )
        .then_ignore(just('"'))
        .collect()
        .map(Token::Str)
        .labelled("string token")
}

#[cfg(test)]
mod tests {
    use super::str;

    #[test]
    fn test_str_1() {
        let s = "abc";
        let qs = format!("\"{}\"", s);
        test_lexer_token!(str(), &qs, Token::Str(s.into()));
    }

    #[test]
    fn test_str_2() {
        let s = "998877str_221100";
        let qs = format!("\"{}\"", s);
        test_lexer_token!(str(), &qs, Token::Str(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_str_3() {
        let s = "42.001\"";
        test_lexer_token!(str(), s, Token::Str("42.001".into()));
    }

    #[test]
    #[should_panic]
    fn test_str_4() {
        let s = "\".test_42";
        test_lexer_token!(str(), s, Token::Str(".test_42".into()));
    }

    #[test]
    #[should_panic]
    fn test_str_5() {
        let s = "'abc'";
        test_lexer_token!(str(), s, Token::Str("abc".into()));
    }
}

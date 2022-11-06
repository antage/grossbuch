use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn colon_ident() -> impl Parser<char, Token, Error = Simple<char, Span>> {
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
        .labelled("colon ident token")
}

#[cfg(test)]
mod tests {
    use super::colon_ident;

    #[test]
    #[should_panic]
    fn test_colon_ident_1() {
        let s = "A";
        test_lexer_token!(colon_ident(), s, Token::ColonIdent(vec![s.into()]));
    }

    #[test]
    fn test_colon_ident_2() {
        let s = "A:B";
        test_lexer_token!(colon_ident(), s, Token::ColonIdent(vec!["A".into(), "B".into()]));
    }

    #[test]
    fn test_colon_ident_3() {
        let s = "A:B:C";
        test_lexer_token!(
            colon_ident(),
            s,
            Token::ColonIdent(
                vec![
                    "A".into(),
                    "B".into(),
                    "C".into()
                ]
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_colon_ident_4() {
        let s = "A::B";
        test_lexer_token!(colon_ident(), s, Token::ColonIdent(vec!["A".into(), "B".into()]));
    }

    #[test]
    #[should_panic]
    fn test_colon_ident_5() {
        let s = "A:::B";
        test_lexer_token!(colon_ident(), s, Token::ColonIdent(vec!["A".into(), "B".into()]));
    }
}

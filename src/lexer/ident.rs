use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn ident() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    text::ident()
        .map(Token::Ident)
        .labelled("ident token")
}

#[cfg(test)]
mod tests {
    use super::ident;

    #[test]
    fn test_ident_1() {
        let s = "A";
        test_lexer_token!(ident(), s, Token::Ident(s.into()));
    }

    #[test]
    fn test_ident_2() {
        let s = "_";
        test_lexer_token!(ident(), s, Token::Ident(s.into()));
    }

    #[test]
    fn test_ident_3() {
        let s = "_abc00";
        test_lexer_token!(ident(), s, Token::Ident(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_ident_4() {
        let s = "A:B";
        test_lexer_token!(ident(), s, Token::Ident(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_ident_5() {
        let s = "0";
        test_lexer_token!(ident(), s, Token::Ident(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_ident_6() {
        let s = ".";
        test_lexer_token!(ident(), s, Token::Ident(s.into()));
    }
}

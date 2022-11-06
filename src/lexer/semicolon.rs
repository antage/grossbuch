use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn semicolon() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just(';')
        .ignored()
        .to(Token::Semicolon)
        .labelled("semicolon")
}

#[cfg(test)]
mod tests {
    use super::semicolon;

    #[test]
    fn test_semicolon_1() {
        let s = ";";
        test_lexer_token!(semicolon(), s, Token::Semicolon);
    }

    #[test]
    #[should_panic]
    fn test_semicolon_2() {
        let s = "";
        test_lexer_token!(semicolon(), s, Token::Semicolon);
    }

    #[test]
    #[should_panic]
    fn test_semicolon_3() {
        let s = "+";
        test_lexer_token!(semicolon(), s, Token::Semicolon);
    }
}

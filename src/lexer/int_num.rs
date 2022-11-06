use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn int_num() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    text::digits(10)
        .map(Token::Int)
        .labelled("int token")
}

#[cfg(test)]
mod tests {
    use super::int_num;

    #[test]
    fn test_int_num_1() {
        let s = "125";
        test_lexer_token!(int_num(), s, Token::Int(s.into()));
    }

    #[test]
    fn test_int_num_2() {
        let s = "99887766554433221100";
        test_lexer_token!(int_num(), s, Token::Int(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_int_num_3() {
        let s = "42.001";
        test_lexer_token!(int_num(), s, Token::Int(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_int_num_4() {
        let s = ".42";
        test_lexer_token!(int_num(), s, Token::Int(s.into()));
    }
}

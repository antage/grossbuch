use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn float_num() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    text::digits(10)
        .chain::<char, _, _>(
            just('.')
                .chain(
                    text::digits(10)
                )
        )
        .collect()
        .map(Token::Float)
        .labelled("float token")
}

#[cfg(test)]
mod tests {
    use super::float_num;

    #[test]
    fn test_float_num_1() {
        let s = "125.99";
        test_lexer_token!(float_num(), s, Token::Float(s.into()));
    }

    #[test]
    fn test_float_num_2() {
        let s = "0.99887766554433221100";
        test_lexer_token!(float_num(), s, Token::Float(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_float_num_3() {
        let s = "42";
        test_lexer_token!(float_num(), s, Token::Float(s.into()));
    }

    #[test]
    #[should_panic]
    fn test_float_num_4() {
        let s = ".42";
        test_lexer_token!(float_num(), s, Token::Float(s.into()));
    }
}

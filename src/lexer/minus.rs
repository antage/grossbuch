use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn minus() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just('-')
        .ignored()
        .to(Token::Minus)
        .labelled("minus token")
}

#[cfg(test)]
mod tests {
    use super::minus;

    #[test]
    fn test_minus_1() {
        let s = "-";
        test_lexer_token!(minus(), s, Token::Minus);
    }

    #[test]
    #[should_panic]
    fn test_minus_2() {
        let s = "";
        test_lexer_token!(minus(), s, Token::Minus);
    }

    #[test]
    #[should_panic]
    fn test_minus_3() {
        let s = "+";
        test_lexer_token!(minus(), s, Token::Minus);
    }
}

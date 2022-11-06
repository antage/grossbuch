use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn left_sq_par() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just('[')
        .ignored()
        .to(Token::LeftSqPar)
        .labelled("left square par")
}

#[cfg(test)]
mod tests {
    use super::left_sq_par;

    #[test]
    fn test_left_sq_par_1() {
        let s = "[";
        test_lexer_token!(left_sq_par(), s, Token::LeftSqPar);
    }

    #[test]
    #[should_panic]
    fn test_left_sq_par_2() {
        let s = "";
        test_lexer_token!(left_sq_par(), s, Token::LeftSqPar);
    }

    #[test]
    #[should_panic]
    fn test_left_sq_par_3() {
        let s = "+";
        test_lexer_token!(left_sq_par(), s, Token::LeftSqPar);
    }
}

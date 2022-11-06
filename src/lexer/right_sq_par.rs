use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn right_sq_par() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just(']')
        .ignored()
        .to(Token::RightSqPar)
        .labelled("right square par")
}

#[cfg(test)]
mod tests {
    use super::right_sq_par;

    #[test]
    fn test_right_sq_par_1() {
        let s = "]";
        test_lexer_token!(right_sq_par(), s, Token::RightSqPar);
    }

    #[test]
    #[should_panic]
    fn test_right_sq_par_2() {
        let s = "";
        test_lexer_token!(right_sq_par(), s, Token::RightSqPar);
    }

    #[test]
    #[should_panic]
    fn test_right_sq_par_3() {
        let s = "+";
        test_lexer_token!(right_sq_par(), s, Token::RightSqPar);
    }
}

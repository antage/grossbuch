use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn left_curly_par() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just('{')
        .ignored()
        .to(Token::LeftCurlyPar)
        .labelled("left curly par")
}

#[cfg(test)]
mod tests {
    use super::left_curly_par;

    #[test]
    fn test_left_curly_par_1() {
        let s = "{";
        test_lexer_token!(left_curly_par(), s, Token::LeftCurlyPar);
    }

    #[test]
    #[should_panic]
    fn test_left_curly_par_2() {
        let s = "";
        test_lexer_token!(left_curly_par(), s, Token::LeftCurlyPar);
    }

    #[test]
    #[should_panic]
    fn test_left_curly_par_3() {
        let s = "+";
        test_lexer_token!(left_curly_par(), s, Token::LeftCurlyPar);
    }
}

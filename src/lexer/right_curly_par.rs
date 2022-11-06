use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn right_curly_par() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just('}')
        .ignored()
        .to(Token::RightCurlyPar)
        .labelled("right curly par")
}

#[cfg(test)]
mod tests {
    use super::right_curly_par;

    #[test]
    fn test_right_curly_par_1() {
        let s = "}";
        test_lexer_token!(right_curly_par(), s, Token::RightCurlyPar);
    }

    #[test]
    #[should_panic]
    fn test_right_curly_par_2() {
        let s = "";
        test_lexer_token!(right_curly_par(), s, Token::RightCurlyPar);
    }

    #[test]
    #[should_panic]
    fn test_right_curly_par_3() {
        let s = "+";
        test_lexer_token!(right_curly_par(), s, Token::RightCurlyPar);
    }
}

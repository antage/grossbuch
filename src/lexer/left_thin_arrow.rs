use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn left_thin_arrow() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just("<-")
        .ignored()
        .to(Token::LeftThinArrow)
        .labelled("left thin arrow")
}

#[cfg(test)]
mod tests {
    use super::left_thin_arrow;

    #[test]
    fn test_left_thin_arrow_1() {
        let s = "<-";
        test_lexer_token!(left_thin_arrow(), s, Token::LeftThinArrow);
    }

    #[test]
    #[should_panic]
    fn test_left_thin_arrow_2() {
        let s = "";
        test_lexer_token!(left_thin_arrow(), s, Token::LeftThinArrow);
    }

    #[test]
    #[should_panic]
    fn test_left_thin_arrow_3() {
        let s = "+";
        test_lexer_token!(left_thin_arrow(), s, Token::LeftThinArrow);
    }
}

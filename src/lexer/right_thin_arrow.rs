use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn right_thin_arrow() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just("->")
        .ignored()
        .to(Token::RightThinArrow)
        .labelled("right thin arrow")
}

#[cfg(test)]
mod tests {
    use super::right_thin_arrow;

    #[test]
    fn test_right_thin_arrow_1() {
        let s = "->";
        test_lexer_token!(right_thin_arrow(), s, Token::RightThinArrow);
    }

    #[test]
    #[should_panic]
    fn test_right_thin_arrow_2() {
        let s = "";
        test_lexer_token!(right_thin_arrow(), s, Token::RightThinArrow);
    }

    #[test]
    #[should_panic]
    fn test_right_thin_arrow_3() {
        let s = "+";
        test_lexer_token!(right_thin_arrow(), s, Token::RightThinArrow);
    }
}

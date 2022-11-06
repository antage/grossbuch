use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn slash() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just('/')
        .ignored()
        .to(Token::Slash)
        .labelled("slash token")
}

#[cfg(test)]
mod tests {
    use super::slash;

    #[test]
    fn test_slash_1() {
        let s = "/";
        test_lexer_token!(slash(), s, Token::Slash);
    }

    #[test]
    #[should_panic]
    fn test_slash_2() {
        let s = "";
        test_lexer_token!(slash(), s, Token::Slash);
    }

    #[test]
    #[should_panic]
    fn test_slash_3() {
        let s = "+";
        test_lexer_token!(slash(), s, Token::Slash);
    }
}

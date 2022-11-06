use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn kw_commodity() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just("commodity")
        .ignored()
        .to(Token::KwCommodity)
        .labelled("commodity")
}

#[cfg(test)]
mod tests {
    use super::kw_commodity;

    #[test]
    fn test_kw_commodity_1() {
        let s = "commodity";
        test_lexer_token!(kw_commodity(), s, Token::KwCommodity);
    }

    #[test]
    #[should_panic]
    fn test_kw_commodity_2() {
        let s = "";
        test_lexer_token!(kw_commodity(), s, Token::KwCommodity);
    }

    #[test]
    #[should_panic]
    fn test_kw_commodity_3() {
        let s = "import";
        test_lexer_token!(kw_commodity(), s, Token::KwCommodity);
    }
}

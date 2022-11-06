use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn kw_account() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just("account")
        .ignored()
        .to(Token::KwAccount)
        .labelled("account")
}

#[cfg(test)]
mod tests {
    use super::kw_account;

    #[test]
    fn test_kw_account_1() {
        let s = "account";
        test_lexer_token!(kw_account(), s, Token::KwAccount);
    }

    #[test]
    #[should_panic]
    fn test_kw_account_2() {
        let s = "";
        test_lexer_token!(kw_account(), s, Token::KwAccount);
    }

    #[test]
    #[should_panic]
    fn test_kw_account_3() {
        let s = "import";
        test_lexer_token!(kw_account(), s, Token::KwAccount);
    }
}

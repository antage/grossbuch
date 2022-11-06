use chumsky::prelude::*;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub(crate) fn kw_import() -> impl Parser<char, Token, Error = Simple<char, Span>> {
    just("import")
        .ignored()
        .to(Token::KwImport)
        .labelled("import")
}

#[cfg(test)]
mod tests {
    use super::kw_import;

    #[test]
    fn test_kw_import_1() {
        let s = "import";
        test_lexer_token!(kw_import(), s, Token::KwImport);
    }

    #[test]
    #[should_panic]
    fn test_kw_import_2() {
        let s = "";
        test_lexer_token!(kw_import(), s, Token::KwImport);
    }

    #[test]
    #[should_panic]
    fn test_kw_import_3() {
        let s = "account";
        test_lexer_token!(kw_import(), s, Token::KwImport);
    }
}

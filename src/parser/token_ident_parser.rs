use chumsky::prelude::*;
use crate::ast::{Token, Span, Spanned};

pub(crate) fn token_ident_parser() -> impl Parser<Token, Spanned<String>, Error = Simple<Token, Span>> {
    filter_map(|span, tok| {
        match tok {
            Token::Ident(s) => Ok((s, span)),
            _ => Err(Simple::custom(span, format!("Token::Ident is expected but found '{}'", tok))),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::token_ident_parser as parser;

    #[test]
    fn test_1() {
        test_parser_value!(parser(), "_ABC0", "_ABC0", 0..5);
    }
}

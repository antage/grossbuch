use chumsky::prelude::*;
use crate::ast::{Token, Span, Spanned};

pub(crate) fn token_str_parser() -> impl Parser<Token, Spanned<String>, Error = Simple<Token, Span>> {
    filter_map(|span, tok| {
        match tok {
            Token::Str(s) => Ok((s, span)),
            _ => Err(Simple::custom(span, format!("Token::Str is expected but found '{}'", tok))),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::token_str_parser as parser;

    #[test]
    fn test_1() {
        test_parser_value!(parser(), "\"123\"", "123", 0..5);
    }
}

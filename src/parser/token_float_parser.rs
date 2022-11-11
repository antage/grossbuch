use chumsky::prelude::*;
use crate::ast::{Token, Span, Spanned};

pub(crate) fn token_float_parser() -> impl Parser<Token, Spanned<String>, Error = Simple<Token, Span>> {
    filter_map(|span, tok| {
        match tok {
            Token::Float(s) => Ok((s, span)),
            _ => Err(Simple::custom(span, format!("Token::Float is expected but found '{}'", tok))),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::token_float_parser as parser;

    #[test]
    fn test_1() {
        test_parser_value!(parser(), "123.55", "123.55", 0..6);
    }
}

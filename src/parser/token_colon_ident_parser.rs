use chumsky::prelude::*;
use crate::ast::{Token, Span, Spanned};

pub(crate) fn token_colon_ident_parser() -> impl Parser<Token, Spanned<Vec<String>>, Error = Simple<Token, Span>> {
    filter_map(|span, tok| {
        match tok {
            Token::ColonIdent(s) => Ok((s, span)),
            _ => Err(Simple::custom(span, format!("Token::ColonIdent is expected but found '{}'", tok))),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::token_colon_ident_parser as parser;

    #[test]
    fn test_1() {
        test_parser_value!(parser(), "_ABC0:XXX", vec!["_ABC0", "XXX"], 0..9);
    }

    #[test]
    fn test_2() {
        test_parser_value!(parser(), "_ABC0:XXX:YYY", vec!["_ABC0", "XXX", "YYY"], 0..13);
    }
}

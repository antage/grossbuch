use chumsky::prelude::*;
use rust_decimal::Decimal;

use crate::ast::{Token, Span, Spanned};
use super::{
    token_int_parser,
    token_float_parser,
};

pub(crate) fn token_decimal_parser() -> impl Parser<Token, Spanned<Decimal>, Error = Simple<Token, Span>> {
    token_float_parser()
        .or(token_int_parser())
        .map(|tok| tok.0)
        .from_str()
        .unwrapped()
        .map_with_span(|tok, span| (tok, span))
        .labelled("decimal")
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::token_decimal_parser as parser;

    #[test]
    fn test_1() {
        test_parser_value!(parser(), "123.55", "123.55".parse().unwrap(), 0..6);
    }

    #[test]
    fn test_2() {
        test_parser_value!(parser(), "123", Decimal::from(123), 0..3);
    }
}

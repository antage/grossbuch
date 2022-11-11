use chumsky::prelude::*;

use crate::ast::{Token, Span, Spanned, CommodityName};

use super::{
    token_ident_parser,
    token_colon_ident_parser,
};

pub(crate) fn commodity_name_parser() -> impl Parser<Token, Spanned<CommodityName>, Error = Simple<Token, Span>> {
    token_colon_ident_parser()
        .or(
            token_ident_parser()
                .map(|(tok, span)| (vec![tok], span))
        )
        .map(|(parts, span)| {
            (
                CommodityName {
                    parts,
                },
                span
            )
        })
        .labelled("account name")
}

#[cfg(test)]
mod tests {
    use crate::ast::CommodityName;
    use super::commodity_name_parser as parser;

    #[test]
    fn test_1() {
        let f = move |r: CommodityName| {
            assert_eq!(r.parts, vec!["Stock", "US", "KO"]);
        };
        test_parser_fn!(parser(), "Stock:US:KO", f, 0..11);
    }

    #[test]
    fn test_2() {
        let f = move |r: CommodityName| {
            assert_eq!(r.parts, vec!["USD"]);
        };
        test_parser_fn!(parser(), "USD", f, 0..3);
    }
}

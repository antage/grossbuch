use chumsky::prelude::*;

use crate::ast::{Token, Span, Spanned, Posting, PostingDirection};

use super::{
    token_decimal_parser,
    account_name_parser,
    commodity_name_parser,
};

pub(crate) fn posting_parser() -> impl Parser<Token, Spanned<Posting>, Error = Simple<Token, Span>> {
    account_name_parser()
        .then(
            just(Token::LeftThinArrow)
                .or(just(Token::RightThinArrow))
                .map_with_span(|tok, span| {
                    (
                        match tok {
                            Token::LeftThinArrow => PostingDirection::Debit,
                            Token::RightThinArrow => PostingDirection::Credit,
                            _ => unreachable!(),
                        },
                        span
                    )
                })
        )
        .then(token_decimal_parser())
        .then(commodity_name_parser())
        .then_ignore(just(Token::Semicolon))
        .map_with_span(|(((account, dir), amount), commodity), span| {
            (
                Posting {
                    account,
                    dir,
                    amount,
                    commodity,
                },
                span
            )
        })
        .labelled("posting")
}

#[cfg(test)]
mod tests {
    use crate::ast::{Posting, PostingDirection};
    use super::posting_parser as parser;

    #[test]
    fn test_1() {
        let f = move |r: Posting| {
            assert_eq!(r.account.0, "Equity:Part1:Part2".parse().unwrap());
            assert_eq!(r.account.1, Span::new("inline", 0..18));
            assert_eq!(r.dir.0, PostingDirection::Credit);
            assert_eq!(r.dir.1, Span::new("inline", 19..21));
            assert_eq!(r.amount.0, "500.00".parse().unwrap());
            assert_eq!(r.amount.1, Span::new("inline", 22..28));
            assert_eq!(r.commodity.0, "USD".parse().unwrap());
            assert_eq!(r.commodity.1, Span::new("inline", 29..32));
        };
        test_parser_fn!(parser(), "Equity:Part1:Part2 -> 500.00 USD;", f, 0..33);
    }

    #[test]
    fn test_2() {
        let f = move |r: Posting| {
            assert_eq!(r.account.0, "Equity:Part1:Part2".parse().unwrap());
            assert_eq!(r.account.1, Span::new("inline", 0..18));
            assert_eq!(r.dir.0, PostingDirection::Debit);
            assert_eq!(r.dir.1, Span::new("inline", 19..21));
            assert_eq!(r.amount.0, "500.00".parse().unwrap());
            assert_eq!(r.amount.1, Span::new("inline", 22..28));
            assert_eq!(r.commodity.0, "USD".parse().unwrap());
            assert_eq!(r.commodity.1, Span::new("inline", 29..32));
        };
        test_parser_fn!(parser(), "Equity:Part1:Part2 <- 500.00 USD;", f, 0..33);
    }
}

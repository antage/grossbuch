use chumsky::prelude::*;

use crate::ast::{Token, Span, Spanned, Transaction};

use super::{
    token_str_parser,
    date_parser,
    posting_parser,
};

pub(crate) fn transaction_parser() -> impl Parser<Token, Spanned<Transaction>, Error = Simple<Token, Span>> {
    date_parser()
        .then(
            token_str_parser()
        )
        .then(
            token_str_parser()
        )
        .then_ignore(just(Token::LeftCurlyPar))
        .then(
            posting_parser()
                .repeated()
                .at_least(1)
        )
        .then_ignore(just(Token::RightCurlyPar))
        .map_with_span(|(((date, payee), narrow), postings), span| {
            (
                Transaction {
                    date,
                    payee,
                    narrow,
                    postings,
                },
                span
            )
        })
        .labelled("transaction")
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use crate::ast::{PostingDirection, Transaction};
    use super::transaction_parser as parser;

    #[test]
    fn test_1() {
        let f = move |r: Transaction| {
            assert_eq!(r.date.0.content(), (2022, 1, 1));
            assert_eq!(r.date.1, Span::new("inline", 0..12));
            assert_eq!(r.payee.0, "bank");
            assert_eq!(r.payee.1, Span::new("inline", 13..19));
            assert_eq!(r.narrow.0, "grocery");
            assert_eq!(r.narrow.1, Span::new("inline", 20..29));
            assert_eq!(r.postings.len(), 2);

            assert_eq!(r.postings[0].0.account.0, "Expenses:Food".parse().unwrap());
            assert_eq!(r.postings[0].0.account.1, Span::new("inline", 32..45));
            assert_eq!(r.postings[0].0.dir.0, PostingDirection::Debit);
            assert_eq!(r.postings[0].0.dir.1, Span::new("inline", 46..48));
            assert_eq!(r.postings[0].0.amount.0, Decimal::from(50));
            assert_eq!(r.postings[0].0.amount.1, Span::new("inline", 49..54));
            assert_eq!(r.postings[0].0.commodity.0, "USD".parse().unwrap());
            assert_eq!(r.postings[0].0.commodity.1, Span::new("inline", 55..58));

            assert_eq!(r.postings[1].0.account.0, "Assets:Bank".parse().unwrap());
            assert_eq!(r.postings[1].0.account.1, Span::new("inline", 60..71));
            assert_eq!(r.postings[1].0.dir.0, PostingDirection::Credit);
            assert_eq!(r.postings[1].0.dir.1, Span::new("inline", 72..74));
            assert_eq!(r.postings[1].0.amount.0, Decimal::from(50));
            assert_eq!(r.postings[1].0.amount.1, Span::new("inline", 75..80));
            assert_eq!(r.postings[1].0.commodity.0, "USD".parse().unwrap());
            assert_eq!(r.postings[1].0.commodity.1, Span::new("inline", 81..84));
        };
        let src = r#"
            [2022-01-01] "bank" "grocery" {
                Expenses:Food <- 50.00 USD;
                Assets:Bank -> 50.00 USD;
            }
        "#.trim().lines().map(|line| line.trim()).collect::<Vec<&str>>().join("\n");
        test_parser_fn!(parser(), &src, f, 0..87);
    }
}

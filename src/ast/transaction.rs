use std::fmt::{Display, Formatter};

use chumsky::prelude::*;

use crate::ast::{Date, Span, Spanned, Token, Posting};

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    date: Spanned<Date>,
    payee: Spanned<String>,
    narrow: Spanned<String>,
    postings: Vec<Spanned<Posting>>,
}

impl Transaction {
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token, Span>> {
        Date::parser()
            .then(
                Token::str_parser()
            )
            .then(
                Token::str_parser()
            )
            .then_ignore(just(Token::LeftCurlyPar))
            .then(
                Posting::parser()
                    .repeated()
                    .at_least(1)
            )
            .then_ignore(just(Token::RightCurlyPar))
            .map_with_span(|(((date, payee), narrow), postings), span| {
                (
                    Self {
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
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} \"{}\" \"{}\" {{\n",
            &self.date.0,
            &self.payee.0,
            &self.narrow.0
        )?;
        for (p, _) in &self.postings {
            write!(f, "\t{}\n", p)?;
        }
        write!(f, "}}\n")
    }
}


#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use rust_decimal::Decimal;

    use crate::ast::{Span, stream_from_str, PostingDirection};
    use crate::lexer::lexer;

    use super::Transaction;

    #[test]
    fn test_transaction() {
        let src = "[2022-01-01] \"bank\" \"grocery\" { Expenses:Food <- 50.00 USD; Assets:Bank -> 50.00 USD; }";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let transaction_res = Transaction::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &transaction_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (transaction, span) = transaction_res.unwrap();
        assert_eq!(transaction.date.0.content(), (2022, 1, 1));
        assert_eq!(transaction.date.1, Span::new("inline", 0..12));
        assert_eq!(transaction.payee.0, "bank");
        assert_eq!(transaction.payee.1, Span::new("inline", 13..19));
        assert_eq!(transaction.narrow.0, "grocery");
        assert_eq!(transaction.narrow.1, Span::new("inline", 20..29));
        assert_eq!(transaction.postings.len(), 2);

        assert_eq!(transaction.postings[0].0.account.0, "Expenses:Food".parse().unwrap());
        assert_eq!(transaction.postings[0].0.account.1, Span::new("inline", 32..45));
        assert_eq!(transaction.postings[0].0.dir.0, PostingDirection::Debit);
        assert_eq!(transaction.postings[0].0.dir.1, Span::new("inline", 46..48));
        assert_eq!(transaction.postings[0].0.amount.0, Decimal::from(50));
        assert_eq!(transaction.postings[0].0.amount.1, Span::new("inline", 49..54));
        assert_eq!(transaction.postings[0].0.commodity.0, "USD".parse().unwrap());
        assert_eq!(transaction.postings[0].0.commodity.1, Span::new("inline", 55..58));

        assert_eq!(transaction.postings[1].0.account.0, "Assets:Bank".parse().unwrap());
        assert_eq!(transaction.postings[1].0.account.1, Span::new("inline", 60..71));
        assert_eq!(transaction.postings[1].0.dir.0, PostingDirection::Credit);
        assert_eq!(transaction.postings[1].0.dir.1, Span::new("inline", 72..74));
        assert_eq!(transaction.postings[1].0.amount.0, Decimal::from(50));
        assert_eq!(transaction.postings[1].0.amount.1, Span::new("inline", 75..80));
        assert_eq!(transaction.postings[1].0.commodity.0, "USD".parse().unwrap());
        assert_eq!(transaction.postings[1].0.commodity.1, Span::new("inline", 81..84));

        assert_eq!(span, Span::new("inline", 0..87));
    }
}

use std::fmt::{Display, Formatter};

use chumsky::prelude::*;
use rust_decimal::Decimal;

use crate::ast::token::Token;
use crate::ast::span::{Span, Spanned};
use crate::ast::account_name::AccountName;
use crate::ast::commodity_name::CommodityName;

#[derive(Clone, Debug, PartialEq)]
pub enum PostingDirection {
    Credit,
    Debit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Posting {
    pub account: Spanned<AccountName>,
    pub dir: Spanned<PostingDirection>,
    pub amount: Spanned<Decimal>,
    pub commodity: Spanned<CommodityName>,
}

impl Posting {
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token, Span>> {
        AccountName::parser()
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
            .then(Token::decimal_parser())
            .then(CommodityName::parser())
            .then_ignore(just(Token::Semicolon))
            .map_with_span(|(((account, dir), amount), commodity), span| {
                (
                    Self {
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
}

impl Display for PostingDirection {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Credit => write!(f, "->"),
            Self::Debit => write!(f, "<-"),
        }
    }
}

impl Display for Posting {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {};",
            self.account.0,
            self.dir.0,
            self.amount.0,
            self.commodity.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use crate::ast::span::{Span, stream_from_str};
    use crate::lexer::lexer;

    use super::{Posting, PostingDirection};

    #[test]
    fn test_posting_credit() {
        let src = "Equity:Part1:Part2 -> 500.00 USD;";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let posting_res = Posting::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &posting_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (posting, span) = posting_res.unwrap();
        assert_eq!(posting.account.0, "Equity:Part1:Part2".parse().unwrap());
        assert_eq!(posting.account.1, Span::new("inline", 0..18));
        assert_eq!(posting.dir.0, PostingDirection::Credit);
        assert_eq!(posting.dir.1, Span::new("inline", 19..21));
        assert_eq!(posting.amount.0, "500.00".parse().unwrap());
        assert_eq!(posting.amount.1, Span::new("inline", 22..28));
        assert_eq!(posting.commodity.0, "USD".parse().unwrap());
        assert_eq!(posting.commodity.1, Span::new("inline", 29..32));
        assert_eq!(span, Span::new("inline", 0..33));
    }

    #[test]
    fn test_posting_debit() {
        let src = "Equity:Part1:Part2 <- 500.00 USD;";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let posting_res = Posting::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &posting_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (posting, span) = posting_res.unwrap();
        assert_eq!(posting.account.0, "Equity:Part1:Part2".parse().unwrap());
        assert_eq!(posting.account.1, Span::new("inline", 0..18));
        assert_eq!(posting.dir.0, PostingDirection::Debit);
        assert_eq!(posting.dir.1, Span::new("inline", 19..21));
        assert_eq!(posting.amount.0, "500.00".parse().unwrap());
        assert_eq!(posting.amount.1, Span::new("inline", 22..28));
        assert_eq!(posting.commodity.0, "USD".parse().unwrap());
        assert_eq!(posting.commodity.1, Span::new("inline", 29..32));
        assert_eq!(span, Span::new("inline", 0..33));
    }
}

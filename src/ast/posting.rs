use std::fmt::{Display, Formatter};

use rust_decimal::Decimal;

use crate::ast::span::Spanned;
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

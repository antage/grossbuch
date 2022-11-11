use std::fmt::{Display, Formatter};

use crate::ast::{Date, Spanned, Posting};

#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub date: Spanned<Date>,
    pub payee: Spanned<String>,
    pub narrow: Spanned<String>,
    pub postings: Vec<Spanned<Posting>>,
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

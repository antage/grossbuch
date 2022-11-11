use std::fmt::{Display, Formatter};

use crate::ast::{Import, Spanned, Transaction};

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub imports: Vec<Spanned<Import>>,
    pub transactions: Vec<Spanned<Transaction>>,
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for tx in &self.transactions {
            write!(f, "{}", tx.0)?;
        }
        Ok(())
    }
}

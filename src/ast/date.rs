use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

use crate::ast::span::Spanned;

#[derive(Clone, Debug)]
pub struct Date {
    pub year: Spanned<usize>,
    pub month: Spanned<usize>,
    pub day: Spanned<usize>,
}

impl Date {
    pub fn content(&self) -> (usize, usize, usize) {
        (self.year.0, self.month.0, self.day.0)
    }
}

impl PartialEq for Date {
    fn eq(&self, rhs: &Self) -> bool {
        (self.year.0 == rhs.year.0)
            && (self.month.0 == rhs.month.0)
            && (self.day.0 == rhs.day.0)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{:04}-{:02}-{:02}]", self.year.0, self.month.0, self.day.0)
    }
}

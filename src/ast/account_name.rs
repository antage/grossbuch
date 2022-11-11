use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::Context;

#[derive(Clone, Debug, PartialEq)]
pub struct AccountName {
    pub parts: Vec<String>,
}

impl Display for AccountName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.parts.join(":"))
    }
}

impl FromStr for AccountName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::parser::utils::parse_str;
        use crate::parser::account_name_parser;

        parse_str(account_name_parser(), s)
            .with_context(|| format!("Can't parse account name in '{}'", s))
    }
}

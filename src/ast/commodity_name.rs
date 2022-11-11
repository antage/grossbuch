use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::Context;

#[derive(Clone, Debug, PartialEq)]
pub struct CommodityName {
    pub parts: Vec<String>,
}

impl Display for CommodityName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.parts.join(":"))
    }
}

impl FromStr for CommodityName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::parser::utils::parse_str;
        use crate::parser::commodity_name_parser;

        parse_str(commodity_name_parser(), s)
            .with_context(|| format!("Can't parse commodity name in '{}'", s))
    }
}

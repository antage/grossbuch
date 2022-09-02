use std::fmt::{Display, Formatter};
use std::str::FromStr;

use anyhow::anyhow;
use chumsky::prelude::*;
use chumsky::Stream;

use crate::ast::token::Token;
use crate::ast::span::{Span, Spanned, stream_from_str};
use crate::lexer::lexer;

#[derive(Clone, Debug, PartialEq)]
pub struct CommodityName {
    pub parts: Vec<String>,
}

impl CommodityName {
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token, Span>> {
        Token::colon_ident_parser()
            .or(
                Token::ident_parser()
                    .map(|(tok, span)| (vec![tok], span))
            )
            .map(|(parts, span)| {
                (
                    Self {
                        parts,
                    },
                    span
                )
            })
            .labelled("commodity name")
    }
}

impl Display for CommodityName {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.parts.join(":"))
    }
}

impl FromStr for CommodityName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks = lexer().parse(stream_from_str(s))
            .map_err(|_| anyhow!("Can't parse account name '{}'", s))?;
        let len = s.chars().count();
        let eoi = Span::new("inline", len..len+1);
        CommodityName::parser().parse(Stream::from_iter(eoi, toks.into_iter()))
            .map(|name| name.0)
            .map_err(|_| anyhow!("Can't parse account name '{}'", s))
    }
}

#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use crate::ast::span::{Span, stream_from_str};
    use crate::lexer::lexer;

    use super::CommodityName;

    #[test]
    fn test_commodity_name_parser_colon_ident() {
        let src = "Equity:Part1:Part2";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let name_res = CommodityName::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &name_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (name, span) = name_res.unwrap();
        assert_eq!(name.parts, vec!["Equity", "Part1", "Part2"]);
        assert_eq!(span, Span::new("inline", 0..18));
    }

    #[test]
    fn test_commodity_name_parser_ident() {
        let src = "Equity";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let name_res = CommodityName::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &name_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (name, span) = name_res.unwrap();
        assert_eq!(name.parts, vec!["Equity"]);
        assert_eq!(span, Span::new("inline", 0..6));
    }
}

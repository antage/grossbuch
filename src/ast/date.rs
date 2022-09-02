use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

use chumsky::prelude::*;

use crate::ast::token::Token;
use crate::ast::span::{Span, Spanned};

#[derive(Clone, Debug)]
pub struct Date {
    pub year: Spanned<usize>,
    pub month: Spanned<usize>,
    pub day: Spanned<usize>,
}

impl Date {
    pub fn parser() -> impl Parser<Token, Spanned<Date>, Error = Simple<Token, Span>> {
        let minus = || just(Token::Minus);
        let slash = || just(Token::Slash);
        let int =
            || Token::int_parser()
                .map(|(tok, span)| (tok.parse().unwrap(), span));

        let date =
            just(Token::LeftSqPar)
                .ignore_then(
                    int()
                        .then_ignore(minus().or(slash()))
                        .then(int())
                        .then_ignore(minus().or(slash()))
                        .then(int())
                        .then_ignore(just(Token::RightSqPar))
                )
                .map_with_span(|((t1, t2), t3), span| {
                    (Date {
                        year: (t1.0, t1.1),
                        month: (t2.0, t2.1),
                        day: (t3.0, t3.1),
                    }, span)
                })
                .labelled("date");

        date
    }

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

#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use crate::ast::span::{Span, stream_from_str};
    use crate::lexer::lexer;

    use super::Date;

    #[test]
    fn test_date_parser_slash() {
        let src = "[2021/12/12]";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let date_res = Date::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &date_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (date, span) = date_res.unwrap();
        assert_eq!(date.content(), (2021, 12, 12));
        assert_eq!(span, Span::new("inline", 0..12));
        assert_eq!(date.year.1, Span::new("inline", 1..5));
        assert_eq!(date.month.1, Span::new("inline", 6..8));
        assert_eq!(date.day.1, Span::new("inline", 9..11));
    }

    #[test]
    fn test_date_parser_minus() {
        let src = "[2021-12-12]";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let date_res = Date::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &date_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (date, span) = date_res.unwrap();
        assert_eq!(date.content(), (2021, 12, 12));
        assert_eq!(span, Span::new("inline", 0..12));
        assert_eq!(date.year.1, Span::new("inline", 1..5));
        assert_eq!(date.month.1, Span::new("inline", 6..8));
        assert_eq!(date.day.1, Span::new("inline", 9..11));
    }
}

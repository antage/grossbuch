use chumsky::prelude::*;

use crate::ast::{Token, Span, Spanned, Date};

use super::{
    token_int_parser
};

pub(crate) fn date_parser() -> impl Parser<Token, Spanned<Date>, Error = Simple<Token, Span>> {
    let minus = || just(Token::Minus);
    let slash = || just(Token::Slash);
    let int =
        || token_int_parser()
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

#[cfg(test)]
mod tests {
    use crate::ast::Date;
    use super::date_parser as parser;

    #[test]
    fn test_1() {
        let f = move |r: Date| {
            assert_eq!(r.content(), (2021, 12, 12));
            assert_eq!(r.year.1, Span::new("inline", 1..5));
            assert_eq!(r.month.1, Span::new("inline", 6..8));
            assert_eq!(r.day.1, Span::new("inline", 9..11));
        };
        test_parser_fn!(parser(), "[2021/12/12]", f, 0..12);
    }

    #[test]
    fn test_2() {
        let f = move |r: Date| {
            assert_eq!(r.content(), (2021, 12, 12));
            assert_eq!(r.year.1, Span::new("inline", 1..5));
            assert_eq!(r.month.1, Span::new("inline", 6..8));
            assert_eq!(r.day.1, Span::new("inline", 9..11));
        };
        test_parser_fn!(parser(), "[2021-12-12]", f, 0..12);
    }
}

use chumsky::prelude::*;

use crate::ast::{Token, Span, Spanned, AccountName};

use super::{
    token_ident_parser,
    token_colon_ident_parser,
};

pub(crate) fn account_name_parser() -> impl Parser<Token, Spanned<AccountName>, Error = Simple<Token, Span>> {
    token_colon_ident_parser()
        .or(
            token_ident_parser()
                .map(|(tok, span)| (vec![tok], span))
        )
        .map(|(parts, span)| {
            (
                AccountName {
                    parts,
                },
                span
            )
        })
        .labelled("account name")
}

#[cfg(test)]
mod tests {
    use crate::ast::AccountName;
    use super::account_name_parser as parser;

    #[test]
    fn test_1() {
        let f = move |r: AccountName| {
            assert_eq!(r.parts, vec!["Equity", "Part1", "Part2"]);
        };
        test_parser_fn!(parser(), "Equity:Part1:Part2", f, 0..18);
    }

    #[test]
    fn test_2() {
        let f = move |r: AccountName| {
            assert_eq!(r.parts, vec!["Equity"]);
        };
        test_parser_fn!(parser(), "Equity", f, 0..6);
    }
}

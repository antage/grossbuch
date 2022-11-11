use chumsky::prelude::*;

use crate::ast::{Token, Span, Spanned, Module};

use super::{
    import_parser,
    transaction_parser,
};

pub(crate) fn module_parser() -> impl Parser<Token, Spanned<Module>, Error = Simple<Token, Span>> {
    import_parser()
        .repeated()
        .then(
            transaction_parser()
                .repeated()
                .then_ignore(end())
        )
        .map_with_span(|(imports, transactions), span| {
            (
                Module {
                    imports: imports,
                    transactions: transactions,
                },
                span
            )
        })
        .labelled("module")
}

#[cfg(test)]
mod tests {
    use crate::ast::Module;
    use super::module_parser as parser;

    #[test]
    fn test_1() {
        let f = move |r: Module| {
            assert_eq!(r.imports.len(), 1);
            assert_eq!(r.transactions.len(), 1);
        };
        let src = r#"
            import "defs.gb";

            [2022-01-01] "bank" "grocery" {
                Expenses:Food <- 50.00 USD;
                Assets:Bank -> 50.00 USD;
            }
        "#.trim().lines().map(|line| line.trim()).collect::<Vec<&str>>().join("\n");
        test_parser_fn!(parser(), &src, f, 0..106);
    }
}

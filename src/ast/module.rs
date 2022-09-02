use std::fmt::{Display, Formatter};

use chumsky::prelude::*;

use crate::ast::{Import, Span, Spanned, Token, Transaction};

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub imports: Vec<Spanned<Import>>,
    transactions: Vec<Spanned<Transaction>>,
}

impl Module {
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token, Span>> {
        Import::parser()
            .repeated()
            .then(
                Transaction::parser()
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
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for tx in &self.transactions {
            write!(f, "{}", tx.0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chumsky::{Parser, Stream};

    use crate::lexer::lexer;
    use crate::ast::{Span, stream_from_str};

    use super::Module;

    #[test]
    fn test_transaction() {
        let src = "import \"defs.gb\"; [2022-01-01] \"bank\" \"grocery\" { Expenses:Food <- 50.00 USD; Assets:Bank -> 50.00 USD; }";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let module_res = Module::parser().parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &module_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (module, span) = module_res.unwrap();
        assert_eq!(module.imports.len(), 1);
        assert_eq!(module.transactions.len(), 1);
        assert_eq!(span, Span::new("inline", 0..105));
    }
}

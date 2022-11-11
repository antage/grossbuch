use std::path::PathBuf;
use chumsky::prelude::*;

use crate::ast::{Token, Span, Spanned, Import};

use super::{
    token_str_parser
};

pub(crate) fn import_parser() -> impl Parser<Token, Spanned<Import>, Error = Simple<Token, Span>> {
    just(Token::KwImport)
        .then(token_str_parser())
        .then_ignore(just(Token::Semicolon))
        .map_with_span(|(_, path), span| {
            (
                Import {
                    path: (PathBuf::from(path.0), path.1),
                },
                span
            )
        })
        .labelled("import")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::ast::Import;
    use super::import_parser as parser;

    #[test]
    fn test_1() {
        let f = move |r: Import| {
            assert_eq!(r.path.0, PathBuf::from("../defs.gb"));
            assert_eq!(r.path.1, Span::new("inline", 7..19));
        };
        test_parser_fn!(parser(), "import \"../defs.gb\";", f, 0..20);
    }
}

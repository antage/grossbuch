use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use chumsky::prelude::*;

use crate::ast::token::Token;
use crate::ast::span::{Span, Spanned};

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub path: Spanned<PathBuf>,
}

impl Import {
    pub fn parser() -> impl Parser<Token, Spanned<Self>, Error = Simple<Token, Span>> {
        just(Token::KwImport)
            .then(Token::str_parser())
            .then_ignore(just(Token::Semicolon))
            .map_with_span(|(_, path), span| {
                (
                    Self {
                        path: (PathBuf::from(path.0), path.1),
                    },
                    span
                )
            })
            .labelled("import")
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "import \"{}\";",
            self.path.0.to_str().unwrap_or("[can't display]"),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use chumsky::prelude::*;

    use crate::ast::span::{Span, stream_from_str};
    use crate::lexer::{lexer, token_stream};

    use super::Import;

    #[test]
    fn test_import() {
        let src = "import \"../defs.gb\";";
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let import_res = Import::parser().parse(token_stream("inline", src, spanned_tokens));
        if let Err(errs) = &import_res {
            for err in errs {
                println!("ERROR: {:?}", err);
            }
        }
        let (import, span) = import_res.unwrap();
        assert_eq!(import.path.0, PathBuf::from("../defs.gb"));
        assert_eq!(import.path.1, Span::new("inline", 7..19));
        assert_eq!(span, Span::new("inline", 0..20));
    }
}

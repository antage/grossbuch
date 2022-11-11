use chumsky::{
    Parser,
    error::Simple,
};

use crate::ast::token::Token;
use crate::ast::span::{Span, Spanned};
use crate::parser::ParseError;

pub(crate) fn parse_str<R, P>(parser: P, input: &str) -> Result<R, ParseError>
where P: Parser<Token, Spanned<R>, Error = Simple<Token, Span>>
{
    use chumsky::prelude::*;
    use chumsky::Stream;

    use crate::ast::span::stream_from_str;
    use crate::lexer::lexer;

    let toks = lexer().parse(stream_from_str(input))
        .map_err(|err| ParseError::LexerError(err))?;
    let len = input.chars().count();
    let eoi = Span::new("inline", len..len+1);
    parser.parse(Stream::from_iter(eoi, toks.into_iter()))
        .map(|r| r.0)
        .map_err(|err| ParseError::ParserError(err))
}

#[cfg(test)]
macro_rules! test_parser_value {
    ($parser:expr, $input:expr, $output:expr, $output_span:expr) => {
        use chumsky::{Parser, Stream};

        use crate::ast::span::{Span, stream_from_str};
        use crate::lexer::lexer;

        let src = { $input };
        let target = { $output };
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let res = { $parser }.parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &res {
            assert_eq!(*errs, vec![]);
        }
        let (ures, span) = res.unwrap();
        assert_eq!(ures, target);
        assert_eq!(span, Span::new("inline", $output_span));
    };
}

#[cfg(test)]
macro_rules! test_parser_fn {
    ($parser:expr, $input:expr, $assert_fn:expr, $output_span:expr) => {
        use chumsky::{Parser, Stream};

        use crate::ast::span::{Span, stream_from_str};
        use crate::lexer::lexer;

        let src = { $input };
        let toks = lexer().parse(stream_from_str(src));
        assert!(toks.is_ok());
        let spanned_tokens = toks.unwrap();
        let len = src.chars().count();
        let eoi = Span::new("inline", len..len+1);
        let res = { $parser }.parse(Stream::from_iter(eoi, spanned_tokens.into_iter()));
        if let Err(errs) = &res {
            assert_eq!(*errs, vec![]);
        }
        let (ures, span) = res.unwrap();
        $assert_fn(ures);
        assert_eq!(span, Span::new("inline", $output_span));
    };
}

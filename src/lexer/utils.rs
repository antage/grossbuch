use std::path::Path;

use chumsky::Stream;

use crate::ast::span::Span;
use crate::ast::token::Token;

pub fn token_stream<'a, S>(filename: S, src: &str, toks: Vec<(Token, Span)>) -> Stream<'a, Token, Span, Box<dyn Iterator<Item = (Token, Span)> + 'a>>
where S: AsRef<Path> {
    let len = src.chars().count();
    let eoi = Span::new(filename, len..len+1);
    Stream::from_iter(eoi, Box::new(toks.into_iter()))
}

#[cfg(test)]
macro_rules! test_lexer_token {
    ($parser:expr, $input:expr, $output:expr) => {
        use chumsky::Parser;
        use crate::ast::span::stream_from_str;
        #[allow(unused_imports)]
        use crate::ast::token::Token;

        let (token_option, errs) =
            $parser.parse_recovery(stream_from_str($input));
        assert_eq!(errs, Vec::new());
        let Some(token) = token_option else {
            panic!("Expected Some(token) but got None");
        };
        assert_eq!(token, $output);
    };
}

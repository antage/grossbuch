use thiserror::Error;
use chumsky::error::Simple;

use crate::ast::{Span, Token};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("lexer parsing errors: {0:?}")]
    LexerError(Vec<Simple<char, Span>>),

    #[error("parser errors: {0:?}")]
    ParserError(Vec<Simple<Token, Span>>),
}

use std::fmt::{Display, Formatter};

use strum_macros::EnumDiscriminants;

#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumDiscriminants)]
pub enum Token {
    Int(String),
    Float(String),
    Str(String),
    Ident(String),
    ColonIdent(Vec<String>),
    Minus,
    Slash,
    Semicolon,
    LeftCurlyPar,
    RightCurlyPar,
    LeftSqPar,
    RightSqPar,
    LeftThinArrow,
    RightThinArrow,
    KwImport,
    KwCommodity,
    KwAccount,
}

impl Token {
    pub fn get_int(&self) -> &str {
        match self {
            Self::Int(n) => n,
            _ => panic!("can't get Token::Int inner value"),
        }
    }

    pub fn get_float(&self) -> &str {
        match self {
            Self::Float(n) => n,
            _ => panic!("can't get Token::Float inner value"),
        }
    }

    pub fn get_str(&self) -> &str {
        match self {
            Self::Str(n) => n,
            _ => panic!("can't get Token::Str inner value"),
        }
    }

    pub fn get_ident(&self) -> &str {
        match self {
            Self::Ident(n) => n,
            _ => panic!("can't get Token::Ident inner value"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Token::Int(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "\"{}\"", s),
            Token::Ident(s) => write!(f, "\"{}\"", s),
            Token::ColonIdent(p) => write!(f, "{}", p.join(":")),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Semicolon => write!(f, ";"),
            Token::LeftCurlyPar => write!(f, "{{"),
            Token::RightCurlyPar => write!(f, "}}"),
            Token::LeftSqPar => write!(f, "["),
            Token::RightSqPar => write!(f, "]"),
            Token::LeftThinArrow => write!(f, "<-"),
            Token::RightThinArrow => write!(f, "->"),
            Token::KwImport => write!(f, "import"),
            Token::KwCommodity => write!(f, "commodity"),
            Token::KwAccount => write!(f, "account"),
        }
    }
}

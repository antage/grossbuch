use chumsky::Parser;

use crate::lexer::lexer;
use crate::ast::span::{Span, stream_from_str};
use crate::ast::token::Token;


macro_rules! test_lexer {
    ($input:expr; Token::Int, $output:expr, $span:expr) => {
        let (tokens, errs) =
            lexer().parse_recovery(stream_from_str($input));
        assert_eq!(errs, vec![]);
        assert!(tokens.is_some());
        assert_eq!(
            tokens.unwrap(),
            vec![
                (Token::Int($output.into()), Span::new("inline", $span))
            ]
        );
    };

    ($input:expr; Token::Float, $output:expr, $span:expr) => {
        let (tokens, errs) =
            lexer().parse_recovery(stream_from_str($input));
        assert_eq!(errs, vec![]);
        assert!(tokens.is_some());
        assert_eq!(
            tokens.unwrap(),
            vec![
                (Token::Float($output.into()), Span::new("inline", $span))
            ]
        );
    };

    ($input:expr; Token::Str, $output:expr, $span:expr) => {
        let (tokens, errs) =
            lexer().parse_recovery(stream_from_str($input));
        assert_eq!(errs, vec![]);
        assert!(tokens.is_some());
        assert_eq!(
            tokens.unwrap(),
            vec![
                (Token::Str($output.into()), Span::new("inline", $span))
            ]
        );
    };

    ($input:expr; Token::Ident, $($output:expr, $span:expr);+) => {
        let (tokens, errs) =
            lexer().parse_recovery(stream_from_str($input));
        assert_eq!(errs, vec![]);
        assert!(tokens.is_some());
        assert_eq!(
            tokens.unwrap(),
            vec![
                $(
                (Token::Ident($output.into()), Span::new("inline", $span)),
                )+
            ]
        );
    };

    ($input:expr; Token::ColonIdent, $($output:expr, $span:expr);+) => {
        let (tokens, errs) =
            lexer().parse_recovery(stream_from_str($input));
        assert_eq!(errs, vec![]);
        assert!(tokens.is_some());
        assert_eq!(
            tokens.unwrap(),
            vec![
            $(
                (
                    Token::ColonIdent($output.iter().map(|s| s.to_string()).collect()),
                    Span::new("inline", $span)
                ),
            )+
            ]
        );
    };

    ($input:expr; $tok:expr, $span:expr) => {
        let (tokens, errs) =
            lexer().parse_recovery(stream_from_str($input));
        assert_eq!(errs, vec![]);
        assert!(tokens.is_some());
        assert_eq!(
            tokens.unwrap(),
            vec![
                ($tok, Span::new("inline", $span))
            ]
        );
    };
}

#[test]
fn test_lexer_int_num() {
    test_lexer!("125"; Token::Int, "125", 0..3);
    test_lexer!("/* comment */ 125"; Token::Int, "125", 14..17);
    test_lexer!("125 // comment"; Token::Int, "125", 0..3);
}

#[test]
fn test_lexer_float_num() {
    test_lexer!("125.99"; Token::Float, "125.99", 0..6);
    test_lexer!("/* comment */ 125.99"; Token::Float, "125.99", 14..20);
    test_lexer!("125.99 // comment "; Token::Float, "125.99", 0..6);
}

#[test]
fn test_lexer_string() {
    test_lexer!("\"abc\""; Token::Str, "abc", 0..5);
    test_lexer!("/* comment */ \"a\n\nb\nc\""; Token::Str, "a\n\nb\nc", 14..22);
    test_lexer!("\"ab\nc\" // comment "; Token::Str, "ab\nc", 0..6);
}

#[test]
fn test_lexer_minus() {
    test_lexer!("-"; Token::Minus, 0..1);
    test_lexer!("/* comment */ - "; Token::Minus, 14..15);
    test_lexer!(" - // comment "; Token::Minus, 1..2);
}

#[test]
fn test_lexer_slash() {
    test_lexer!("/"; Token::Slash, 0..1);
    test_lexer!("/* comment */ / "; Token::Slash, 14..15);
    test_lexer!(" / // comment "; Token::Slash, 1..2);
}

#[test]
fn test_lexer_semicolon() {
    test_lexer!(";"; Token::Semicolon, 0..1);
    test_lexer!("/* comment */ ; "; Token::Semicolon, 14..15);
    test_lexer!(" ; // comment "; Token::Semicolon, 1..2);
}

#[test]
fn test_lexer_left_curly_par() {
    test_lexer!("{"; Token::LeftCurlyPar, 0..1);
    test_lexer!("/* comment */ { "; Token::LeftCurlyPar, 14..15);
    test_lexer!(" { // comment "; Token::LeftCurlyPar, 1..2);
}

#[test]
fn test_lexer_right_curly_par() {
    test_lexer!("}"; Token::RightCurlyPar, 0..1);
    test_lexer!("/* comment */ } "; Token::RightCurlyPar, 14..15);
    test_lexer!(" } // comment "; Token::RightCurlyPar, 1..2);
}

#[test]
fn test_lexer_left_thin_arrow() {
    test_lexer!("<-"; Token::LeftThinArrow, 0..2);
    test_lexer!("/* comment */ <- "; Token::LeftThinArrow, 14..16);
    test_lexer!(" <- // comment "; Token::LeftThinArrow, 1..3);
}

#[test]
fn test_lexer_right_thin_arrow() {
    test_lexer!("->"; Token::RightThinArrow, 0..2);
    test_lexer!("/* comment */ -> "; Token::RightThinArrow, 14..16);
    test_lexer!(" -> // comment "; Token::RightThinArrow, 1..3);
}

#[test]
fn test_lexer_left_sq_par() {
    test_lexer!("["; Token::LeftSqPar, 0..1);
    test_lexer!("/* comment */ [ "; Token::LeftSqPar, 14..15);
    test_lexer!(" [ // comment "; Token::LeftSqPar, 1..2);
}

#[test]
fn test_lexer_right_sq_par() {
    test_lexer!("]"; Token::RightSqPar, 0..1);
    test_lexer!("/* comment */ ] "; Token::RightSqPar, 14..15);
    test_lexer!(" ] // comment "; Token::RightSqPar, 1..2);
}

#[test]
fn test_lexer_commodity() {
    test_lexer!("commodity"; Token::KwCommodity, 0..9);
    test_lexer!("/* comment */ commodity "; Token::KwCommodity, 14..23);
    test_lexer!(" commodity // comment "; Token::KwCommodity, 1..10);
}

#[test]
fn test_lexer_account() {
    test_lexer!("account"; Token::KwAccount, 0..7);
    test_lexer!("/* comment */ account "; Token::KwAccount, 14..21);
    test_lexer!(" account // comment "; Token::KwAccount, 1..8);
}

#[test]
fn test_lexer_import() {
    test_lexer!("import"; Token::KwImport, 0..6);
    test_lexer!("/* comment */ import "; Token::KwImport, 14..20);
    test_lexer!(" import // comment "; Token::KwImport, 1..7);
}

#[test]
fn test_lexer_ident() {
    test_lexer!("_Abc019"; Token::Ident, "_Abc019", 0..7);
    test_lexer!("_Abc019 xyz"; Token::Ident, "_Abc019", 0..7; "xyz", 8..11);
    test_lexer!("/* comment */ _000 "; Token::Ident, "_000", 14..18);
    test_lexer!(" X_Y_Z // comment "; Token::Ident, "X_Y_Z", 1..6);
}

#[test]
fn test_lexer_colon_ident() {
    test_lexer!("_Abc019:XYZ"; Token::ColonIdent, vec!["_Abc019", "XYZ"], 0..11);
    test_lexer!("_Abc019:XYZ xyz:abc"; Token::ColonIdent, vec!["_Abc019", "XYZ"], 0..11; vec!["xyz", "abc"], 12..19);
    test_lexer!("/* comment */ _000:XYZ "; Token::ColonIdent, vec!["_000", "XYZ"], 14..22);
    test_lexer!(" X_Y_Z:XYZ // comment "; Token::ColonIdent, vec!["X_Y_Z", "XYZ"], 1..10);
}

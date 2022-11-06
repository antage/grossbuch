
use chumsky::prelude::*;

use crate::ast::span::{Span, Spanned};
use crate::ast::token::Token;

use super::float_num::float_num;
use super::int_num::int_num;
use super::str::str;
use super::minus::minus;
use super::slash::slash;
use super::semicolon::semicolon;
use super::left_curly_par::left_curly_par;
use super::right_curly_par::right_curly_par;
use super::left_sq_par::left_sq_par;
use super::right_sq_par::right_sq_par;
use super::left_thin_arrow::left_thin_arrow;
use super::right_thin_arrow::right_thin_arrow;
use super::kw_import::kw_import;
use super::kw_commodity::kw_commodity;
use super::kw_account::kw_account;
use super::colon_ident::colon_ident;
use super::ident::ident;
use super::comment_oneline::comment_oneline;
use super::comment_multiline::comment_multiline;

pub fn lexer() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char, Span>> {
    let token =
        float_num()
            .or(int_num())
            .or(str())
            .or(semicolon())
            .or(left_curly_par())
            .or(right_curly_par())
            .or(left_sq_par())
            .or(right_sq_par())
            .or(left_thin_arrow())
            .or(right_thin_arrow())
            .or(minus())
            .or(slash())
            .or(kw_import())
            .or(kw_commodity())
            .or(kw_account())
            .or(colon_ident())
            .or(ident());

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment_oneline().or(comment_multiline()).repeated())
        .padded()
        .repeated()
}

use chumsky::prelude::*;

use crate::ast::span::Span;

pub(crate) fn comment_multiline() -> impl Parser<char, (), Error = Simple<char, Span>> + Clone {
    just("/*")
        .then_ignore(take_until(just("*/")))
        .ignored()
        .padded()
}

#[cfg(test)]
mod tests {
    use super::comment_multiline;

    #[test]
    fn test_comment_multiline_1() {
        let s = "/* abc */";
        test_lexer_token!(comment_multiline(), s, ());
    }

    #[test]
    fn test_comment_multiline_2() {
        let s = "/**/";
        test_lexer_token!(comment_multiline(), s, ());
    }

    #[test]
    #[should_panic]
    fn test_comment_multiline_3() {
        let s = "";
        test_lexer_token!(comment_multiline(), s, ());
    }

    #[test]
    #[should_panic]
    fn test_comment_multiline_4() {
        let s = "// abc";
        test_lexer_token!(comment_multiline(), s, ());
    }
}

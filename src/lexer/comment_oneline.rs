use chumsky::prelude::*;

use crate::ast::span::Span;

pub(crate) fn comment_oneline() -> impl Parser<char, (), Error = Simple<char, Span>> + Clone {
    just("//")
        .then_ignore(
            take_until(
                text::newline()
                .or(end())
            )
        )
        .ignored()
        .padded()
}

#[cfg(test)]
mod tests {
    use super::comment_oneline;

    #[test]
    fn test_comment_oneline_1() {
        let s = "// abc";
        test_lexer_token!(comment_oneline(), s, ());
    }

    #[test]
    fn test_comment_oneline_2() {
        let s = "//";
        test_lexer_token!(comment_oneline(), s, ());
    }

    #[test]
    #[should_panic]
    fn test_comment_oneline_3() {
        let s = "";
        test_lexer_token!(comment_oneline(), s, ());
    }

    #[test]
    #[should_panic]
    fn test_comment_oneline_4() {
        let s = "/* abc */";
        test_lexer_token!(comment_oneline(), s, ());
    }
}

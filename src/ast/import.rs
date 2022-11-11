use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::ast::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub path: Spanned<PathBuf>,
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

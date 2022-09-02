use std::path::{Path, PathBuf};
use std::ops::Range;

use chumsky::{Stream, Span as SpanT};
use ariadne::Span as SpanA;

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    filename: PathBuf,
    range: Range<usize>,
}

impl Span {
    pub fn new<S>(filename: S, range: Range<usize>) -> Self
    where S: AsRef<Path> {
        Span {
            filename: filename.as_ref().to_path_buf(),
            range,
        }
    }

    pub fn filename(&self) -> &Path {
        self.filename.as_path()
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }
}

pub type Spanned<T> = (T, Span);

impl SpanT for Span {
    type Context = PathBuf;
    type Offset = usize;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Span::new(context, range)
    }

    fn context(&self) -> Self::Context {
        self.filename.clone()
    }

    fn start(&self) -> Self::Offset {
        self.range.start
    }

    fn end(&self) -> Self::Offset {
        self.range.end
    }
}

impl SpanA for Span {
    type SourceId = Path;

    fn source(&self) -> &Self::SourceId {
        &self.filename
    }

    fn start(&self) -> usize {
        self.range.start
    }

    fn end(&self) -> usize {
        self.range.end
    }
}

pub fn stream_from_str<'a>(s: &'a str) -> Stream<'a, char, Span, Box<dyn Iterator<Item = (char, Span)> + 'a>> {
    let len = s.chars().count();
    Stream::from_iter(
        Span::new("inline", len..len+1),
        Box::new(s.chars().enumerate().map(|(i, c)| (c, Span::new("inline", i..i+1))))
    )
}

pub fn stream_from_file<'a, S>(filename: S, s: &'a str) -> Stream<'a, char, Span, Box<dyn Iterator<Item = (char, Span)> + 'a>>
where S: 'a + AsRef<Path> + Clone {
    let len = s.chars().count();
    Stream::from_iter(
        Span::new(filename.clone(), len..len+1),
        Box::new(s.chars().enumerate().map(move |(i, c)| (c, Span::new(filename.clone(), i..i+1))))
    )
}

use std::path::PathBuf;

use anyhow::Result;
use ariadne::{Color, Fmt, Label, Report, ReportKind, FileCache};
use chumsky::prelude::*;
use chumsky::error::{Simple, SimpleReason};
use structopt::StructOpt;

use grossbuch::book::{Book, ParseFileError};
use grossbuch::ast::Span;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn display_errors(errs: Vec<Simple<String, Span>>, filename: &PathBuf) -> Result<()> {
    for e in errs {
        // eprint!("ERR: {:?}", e);
        let report = Report::build(ReportKind::Error, filename, e.span().start());
        let report =
            match e.reason() {
                SimpleReason::Unclosed { span, delimiter } =>
                    report
                        .with_message(
                            format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            )
                        )
                        .with_label(
                            Label::new(span.clone())
                                .with_message(
                                    format!(
                                        "Unclosed delimiter {}",
                                        delimiter.fg(Color::Yellow)
                                    )
                                )
                                .with_color(Color::Yellow)
                        )
                        .with_label(
                            Label::new(e.span())
                                .with_message(
                                    format!(
                                        "Must be closed before this {}",
                                        e.found()
                                            .unwrap_or(&"end of file".to_string())
                                            .fg(Color::Red)
                                    )
                                )
                                .with_color(Color::Red)
                        ),
                SimpleReason::Unexpected =>
                    report
                        .with_message(
                            format!(
                                "{}, expected {}",
                                if e.found().is_some() {
                                    "Unexpected token in input"
                                } else {
                                    "Unexpected end of input"
                                },
                                if e.expected().len() == 0 {
                                    "something else".to_string()
                                } else {
                                    e.expected()
                                        .map(|expected| {
                                            match expected {
                                                Some(expected) => expected.to_string(),
                                                None => "end of input".to_string(),
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                        .join(", ")
                                }
                            )
                        )
                        .with_label(
                            Label::new(e.span())
                                .with_message(
                                    format!(
                                        "Unexpected token {}",
                                        e.found()
                                            .unwrap_or(&"end of file".to_string())
                                            .fg(Color::Red)
                                    )
                                )
                                .with_color(Color::Red)
                        ),
                SimpleReason::Custom(msg) =>
                    report
                        .with_message(msg)
                        .with_label(
                            Label::new(e.span())
                                .with_message(
                                    format!(
                                        "{}",
                                        msg.fg(Color::Red)
                                    )
                                )
                                .with_color(Color::Red)
                        ),
            };

        report.finish().eprint(FileCache::default()).unwrap();
    }

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let mut book = Book::new();

    let parse_res = book.parse_file(&opt.input);
    if let Err(parse_err) = parse_res {
        match parse_err {
            ParseFileError::LexerError(errs) => {
                let normalized_errs: Vec<_> =
                    errs
                        .into_iter()
                        .map(|e| {
                            e.map(|c| c.to_string())
                        })
                        .collect();
                display_errors(normalized_errs, &opt.input)?;
            },
            ParseFileError::ParserError(errs) => {
                let normalized_errs: Vec<_> =
                    errs
                        .into_iter()
                        .map(|e| {
                            e.map(|t| t.to_string())
                        })
                        .collect();
                display_errors(normalized_errs, &opt.input)?;
            },
            ParseFileError::IOError(err) => eprintln!("I/O error: {}", err),
            ParseFileError::ImportsCycleError => eprintln!("Error: imports cycle is detected"),
        };
    }



    Ok(())
}

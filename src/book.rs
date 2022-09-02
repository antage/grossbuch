use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;

use thiserror::Error;
use solvent::{DepGraph, SolventError};
use chumsky::{Parser, error::Simple};

use crate::ast::{Module, Span, Spanned, stream_from_file, Token};
use crate::lexer::{lexer, token_stream};

#[derive(Error, Debug)]
pub enum ParseFileError {
    #[error("can't open file")]
    IOError(#[from] std::io::Error),

    #[error("lexer parsing errors: {0:?}")]
    LexerError(Vec<Simple<char, Span>>),

    #[error("parser errors: {0:?}")]
    ParserError(Vec<Simple<Token, Span>>),

    #[error("imports cycle is detected")]
    ImportsCycleError,
}

pub struct Book {
    deps: DepGraph<PathBuf>,
    modules: HashMap<PathBuf, Spanned<Module>>,
}

impl Book {
    pub fn new() -> Self {
        Book {
            deps: DepGraph::new(),
            modules: HashMap::new(),
        }
    }

    pub fn parse_file<S>(&mut self, path: S) -> Result<(), ParseFileError>
    where S: AsRef<Path> {
        let path_owned = path.as_ref().canonicalize()?;
        if self.modules.contains_key(&path_owned) {
            Ok(())
        } else {
            eprintln!("Importing '{}'", path_owned.display());

            let file = File::open(&path_owned)?;
            let mut buf_file = BufReader::new(file);
            let mut src = String::new();
            buf_file.read_to_string(&mut src)?;

            let lexer_stream = stream_from_file(&path_owned, &src);
            let tokens =
                lexer().parse(lexer_stream)
                    .map_err(|err| ParseFileError::LexerError(err))?;

            let parser_stream = token_stream(&path_owned, &src, tokens);
            let ast =
                Module::parser()
                    .parse(parser_stream)
                    .map_err(|err| ParseFileError::ParserError(err))?;

            self.modules.insert(path_owned.clone(), ast.clone());
            self.deps.register_node(path_owned.clone());

            let imports: Vec<PathBuf> =
                ast.0.imports
                    .iter()
                    .map(|(import, _)| {
                        let mut parent =
                            path_owned
                                .clone()
                                .parent()
                                .expect("can't get base directory")
                                .to_path_buf();
                        parent.push(&import.path.0);
                        parent
                    })
                    .collect();

            for import in imports {
                self.deps.register_dependency(path_owned.clone(), import);
            }

            let cloned_deps = self.deps.clone();
            let deps_iter =
                cloned_deps
                    .dependencies_of(&path_owned)
                    .map_err(|err| {
                        match err {
                            SolventError::CycleDetected => ParseFileError::ImportsCycleError,
                            _ => panic!("{}", err),
                        }
                    })?;
            let dep_imports: Vec<_> =
                deps_iter
                    .map(|dep| {
                        dep
                            .map_err(|err| {
                                match err {
                                    SolventError::CycleDetected => ParseFileError::ImportsCycleError,
                                    _ => panic!("{}", err),
                                }
                            })
                    })
                    .collect();
            for dep_import in dep_imports {
                self.parse_file(dep_import?)?;
            }

            self.deps.mark_as_satisfied(&[path_owned.clone()]).unwrap();

            Ok(())
        }
    }
}

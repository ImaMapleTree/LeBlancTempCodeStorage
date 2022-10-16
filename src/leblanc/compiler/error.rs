pub(crate) mod snippet_collection;
pub(crate) mod snippet;
use std::fs;
use std::mem::take;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use crate::leblanc::compiler::error::snippet::ErrorSnippet;
use crate::leblanc::rustblanc::path::ZCPath;


#[derive(Default, Debug)]
pub struct ErrorReporter {
    errors: Vec<ErrorSnippet>
}

impl ErrorReporter {

    pub fn add_snippet(&mut self, snippet: ErrorSnippet) {
        if self.errors.contains(&snippet) { return; }
        self.errors.push(snippet);
    }

    pub fn parse_error(&mut self, file: ZCPath, error: ParseError<usize, Token<'_>, &str>) {
        let (start, end, error_type, reason) = match error {
            ParseError::InvalidToken { location } => {
                (location, location, String::from("Invalid Token"), String::from("Invalid Token"))
            },
            ParseError::UnrecognizedEOF { location: loc, expected: _expected } => {
                (loc, loc, String::from("Unrecognized EOF"), "Invalid Token".to_string())
            },
            ParseError::UnrecognizedToken { token: t, expected: _expected } => {
                (t.0, t.2, String::from("Unrecognized Token"), "Invalid Token".to_string())
            },
            ParseError::ExtraToken { token } => {
                (token.0, token.2, String::from("Extra Token"), String::from("Extra Token"))
            },
            ParseError::User { error } => {
                (0, 0, String::from("N/A"), String::from("N/A"))
            }
        };
        let snippet = ErrorSnippet::new(file, "Parse Error: ".to_string() + &error_type);
        self.add_snippet(snippet.add_primary(start, end, reason));
    }

    pub fn report(&mut self) {
        for i in 0..self.errors.len() {
            let snippet = take(&mut self.errors[i]);
            let file = SimpleFile::new(&snippet.file, fs::read_to_string(snippet.file).unwrap());
            let mut diagnostic = Diagnostic::error()
                .with_message(&snippet.error_type)
                .with_labels(
                    snippet.comments
                        .into_iter()
                        .map(|c| Label::new(c.3,(), c.0..c.1)
                            .with_message(c.2))
                        .collect()
                );

            diagnostic = diagnostic.with_notes(snippet.diagnostics);

            let writer = StandardStream::stderr(ColorChoice::Always);
            let config = term::Config::default();


            term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();
        }
        self.errors.clear();
    }

    pub fn should_exit(&self) -> bool {
        self.has_errors() && !crate::leblanc::configuration::REPORT_ALL_ERRORS
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}



use std::fs;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;

pub fn report(error: ParseError<usize, Token<'_>, &str>) {
    println!("Real error: {:#?}", error);
    let (start, end, expected) = match error {
        ParseError::InvalidToken { location } => (location, location, vec![]),
        ParseError::UnrecognizedEOF { location: loc, expected } => (loc, loc, expected),
        ParseError::UnrecognizedToken { token: t, expected } => (t.0, t.2, expected),
        ParseError::ExtraToken { token } => (token.0, token.2, vec![]),
        ParseError::User { error } => (0, 0, vec![])
    };

    let file = SimpleFile::new("test.lb", fs::read_to_string("test.lb").unwrap());

    let mut diagnostic = Diagnostic::error()
        .with_message("parse error")
        .with_labels(vec![
            Label::primary((), start..end).with_message("parse error")
        ]);
    if !expected.is_empty() {
        diagnostic = diagnostic.with_notes(vec![format!("expected: {}", expected[0])]);
    }

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut writer.lock(), &config, &file, &diagnostic).unwrap();



}
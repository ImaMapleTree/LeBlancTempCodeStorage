use core::fmt::Display;
use codespan_reporting::diagnostic::LabelStyle;
use crate::leblanc::rustblanc::path::ZCPath;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ErrorSnippet {
    pub file: ZCPath,
    pub comments: Vec<(usize, usize, String, LabelStyle)>,
    pub diagnostics: Vec<String>,
    pub error_type: String
}

impl ErrorSnippet {
    pub fn new<S: Display>(file: ZCPath, error_type: S) -> ErrorSnippet {
        ErrorSnippet {
            file,
            comments: Vec::new(),
            diagnostics: Vec::new(),
            error_type: error_type.to_string()
        }
    }

    pub fn add_primary<T: Display>(mut self, start: usize, end: usize, tidbit: T) -> ErrorSnippet {
        self.comments.push((start, end, tidbit.to_string(), LabelStyle::Primary));
        self
    }

    pub fn add_secondary<T: Display>(mut self, start: usize, end: usize, tidbit: T) -> ErrorSnippet {
        self.comments.push((start, end, tidbit.to_string(), LabelStyle::Secondary));
        self
    }

    pub fn add_diagnostic(mut self, diagnostic: String) -> ErrorSnippet {
        self.diagnostics.push(diagnostic);
        self
    }
}

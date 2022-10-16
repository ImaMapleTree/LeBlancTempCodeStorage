use crate::leblanc::compiler::error::snippet::ErrorSnippet;

#[derive(Default)]
pub struct SnippetCollection {
    snippets: Vec<ErrorSnippet>
}

impl SnippetCollection {
    pub fn add_snippet(&mut self, snippet: ErrorSnippet) {
        self.snippets.push(snippet);
    }
}
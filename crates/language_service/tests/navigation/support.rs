use language_service::{dto::SourceLocation, InMemoryWorkspace};
use semantic_core::{SysmlDocument, SysmlDocumentSourceKind, TextPosition};

pub fn document(path: &str, content: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "test",
        path,
        content.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document")
}

pub fn workspace_from_docs(docs: Vec<SysmlDocument>) -> InMemoryWorkspace {
    InMemoryWorkspace::from_documents(docs).expect("workspace")
}

pub fn single_doc(path: &str, content: &str) -> InMemoryWorkspace {
    workspace_from_docs(vec![document(path, content)])
}

pub fn multi_doc(paths_and_content: &[(&str, &str)]) -> InMemoryWorkspace {
    let docs = paths_and_content
        .iter()
        .map(|(path, content)| document(path, content))
        .collect();
    workspace_from_docs(docs)
}

pub fn position_for(content: &str, needle: &str) -> TextPosition {
    for (line_index, line) in content.lines().enumerate() {
        if let Some(byte_offset) = line.find(needle) {
            let character = line[..byte_offset].chars().count() as u32;
            return TextPosition {
                line: line_index as u32,
                character,
            };
        }
    }
    panic!("needle not found: {needle}");
}

pub fn position_for_within(content: &str, needle: &str, inner: &str) -> TextPosition {
    let base = position_for(content, needle);
    let line = content
        .lines()
        .nth(base.line as usize)
        .expect("line for within position");
    let inner_offset = line
        .find(inner)
        .unwrap_or_else(|| panic!("inner needle not found: {inner}"));
    let inner_char = line[..inner_offset].chars().count() as u32;
    TextPosition {
        line: base.line,
        character: inner_char,
    }
}

pub fn position_at(line: u32, character: u32) -> TextPosition {
    TextPosition { line, character }
}

pub fn any_on_line(locations: &[SourceLocation], line: u32) -> bool {
    locations
        .iter()
        .any(|loc| loc.range.start.line == line)
}

pub fn count_on_line(locations: &[SourceLocation], line: u32) -> usize {
    locations
        .iter()
        .filter(|loc| loc.range.start.line == line)
        .count()
}

pub fn any_on_line_at(locations: &[SourceLocation], line: u32, character: u32) -> bool {
    locations.iter().any(|loc| {
        loc.range.start.line == line && loc.range.start.character == character
    })
}

pub fn paths(locations: &[SourceLocation]) -> Vec<&str> {
    locations.iter().map(|loc| loc.path.as_str()).collect()
}

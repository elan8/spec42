use sysml_query::resolved_slice::{TextPosition, TextRange};
use tower_lsp::lsp_types::{Position, Range};

pub fn to_core_position(position: Position) -> TextPosition {
    TextPosition::new(position.line, position.character)
}

pub fn to_core_range(range: Range) -> TextRange {
    TextRange::new(to_core_position(range.start), to_core_position(range.end))
}

pub fn to_lsp_range(range: TextRange) -> Range {
    Range::new(
        Position::new(range.start.line, range.start.character),
        Position::new(range.end.line, range.end.character),
    )
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn roundtrip_position_conversion() {
        let lsp = Position::new(12, 34);
        let core = to_core_position(lsp);
        assert_eq!(core.line, 12);
        assert_eq!(core.character, 34);
    }

    #[test]
    fn roundtrip_range_conversion() {
        let lsp = Range::new(Position::new(1, 2), Position::new(3, 4));
        let core = to_core_range(lsp);
        let back = to_lsp_range(core);
        assert_eq!(back.start.line, 1);
        assert_eq!(back.start.character, 2);
        assert_eq!(back.end.line, 3);
        assert_eq!(back.end.character, 4);
    }

    /// Every `TextRange`/`SyntaxRange` -> LSP `Range`/`RangeDto` projection declared in this crate.
    ///
    /// There are exactly two legitimate targets: the protocol `Range` (this module's
    /// `to_lsp_range`) and the wire `RangeDto` (`views::dto::range_to_dto`). A third declaration
    /// is a re-derivation of a projection that already has an owner. The earlier form of this
    /// guard named a single file and missed five copies; this one scans the whole crate, so any
    /// new copy fails the test wherever it is written.
    #[test]
    fn one_range_projection_per_target() {
        let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut found: Vec<String> = Vec::new();
        for file in rust_files(&src) {
            let content = std::fs::read_to_string(&file).expect("read source file");
            for signature in fn_signatures(&content) {
                let Some((params, ret)) = signature.split_once("->") else {
                    continue;
                };
                let takes_range = params.contains("TextRange") || params.contains("SyntaxRange");
                if takes_range && ret.contains("Range") {
                    let name = file
                        .strip_prefix(&src)
                        .unwrap_or(&file)
                        .display()
                        .to_string();
                    found.push(format!("{name}: {}", signature.trim()));
                }
            }
        }
        found.sort();
        assert!(
            found.len() <= 2,
            "range projection has one owner per target (`to_lsp_range`, `views::dto::range_to_dto`); \
             found {} declarations: {found:#?}",
            found.len()
        );
    }

    fn rust_files(dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir).expect("read source directory") {
            let path = entry.expect("read directory entry").path();
            if path.is_dir() {
                out.extend(rust_files(&path));
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
        out
    }

    /// Signature text (parameter list plus return type) of every `fn` declaration in `content`.
    fn fn_signatures(content: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut rest = content;
        while let Some(at) = rest.find("fn ") {
            rest = &rest[at + 3..];
            let Some(open) = rest.find('(') else { break };
            let mut depth = 0usize;
            let mut end = None;
            for (index, ch) in rest[open..].char_indices() {
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(open + index + 1);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let Some(end) = end else { break };
            let tail = &rest[end..];
            let stop = tail.find(['{', ';']).unwrap_or(tail.len().min(120));
            out.push(format!("{}{}", &rest[open..end], &tail[..stop]));
            rest = &rest[end..];
        }
        out
    }
}

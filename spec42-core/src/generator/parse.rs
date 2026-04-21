use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use super::types::{BlockKind, ModelBlock};

pub(crate) fn parse_model_blocks(paths: &[PathBuf]) -> Result<Vec<ModelBlock>, String> {
    let mut blocks = Vec::new();
    for path in paths {
        let content = fs::read_to_string(path)
            .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
        blocks.extend(parse_blocks_in_source(&content));
    }
    Ok(blocks)
}

pub(crate) fn parse_blocks_in_source(content: &str) -> Vec<ModelBlock> {
    let mut blocks = Vec::new();
    let mut current: Option<ModelBlock> = None;
    let mut depth: i32 = 0;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if current.is_none() {
            if let Some((kind, name, base, starts_block, initial_depth)) = parse_header(line) {
                if starts_block {
                    current = Some(ModelBlock {
                        kind,
                        name,
                        base,
                        attrs: BTreeMap::new(),
                    });
                    depth = initial_depth;
                    if depth <= 0 {
                        blocks.push(current.take().unwrap_or_else(|| ModelBlock {
                            kind,
                            name: String::new(),
                            base: None,
                            attrs: BTreeMap::new(),
                        }));
                    }
                } else {
                    blocks.push(ModelBlock {
                        kind,
                        name,
                        base,
                        attrs: BTreeMap::new(),
                    });
                }
            }
            continue;
        }

        if let Some(block) = current.as_mut() {
            if depth == 1 {
                if let Some((name, value)) = parse_attribute_assignment(line) {
                    block.attrs.insert(name, value);
                }
            }
        }

        depth += brace_delta(line);
        if depth <= 0 {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
        }
    }
    blocks
}

fn parse_header(line: &str) -> Option<(BlockKind, String, Option<String>, bool, i32)> {
    if let Some((name, base, starts_block, depth)) = parse_definition(line, "part def ") {
        return Some((BlockKind::PartDef, name, base, starts_block, depth));
    }
    if let Some((name, base, starts_block, depth)) = parse_definition(line, "item def ") {
        return Some((BlockKind::ItemDef, name, base, starts_block, depth));
    }
    None
}

fn parse_definition(line: &str, prefix: &str) -> Option<(String, Option<String>, bool, i32)> {
    if !line.starts_with(prefix) {
        return None;
    }
    let remainder = line.trim_start_matches(prefix).trim();
    let name = take_symbol(remainder)?;
    let base = remainder
        .find(":>")
        .and_then(|idx| take_symbol(remainder[idx + 2..].trim()));
    let starts_block = line.contains('{');
    let depth = brace_delta(line);
    Some((name, base, starts_block, depth))
}

fn parse_attribute_assignment(line: &str) -> Option<(String, String)> {
    if !(line.starts_with("attribute :>> ") || line.starts_with("attribute ")) {
        return None;
    }
    let (left, right) = line.split_once('=')?;
    let mut lhs = left.trim_start_matches("attribute").trim();
    lhs = lhs.trim_start_matches(":>>").trim();
    let attr_name = take_symbol(lhs)?;
    let value = right.trim().trim_end_matches(';').trim().trim_matches('"');
    Some((attr_name, value.to_string()))
}

fn take_symbol(input: &str) -> Option<String> {
    let mut symbol = String::new();
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '/' || ch == '.' {
            symbol.push(ch);
        } else {
            break;
        }
    }
    if symbol.is_empty() {
        None
    } else {
        Some(symbol)
    }
}

fn brace_delta(line: &str) -> i32 {
    let open = line.chars().filter(|ch| *ch == '{').count() as i32;
    let close = line.chars().filter(|ch| *ch == '}').count() as i32;
    open - close
}

#[cfg(test)]
mod tests {
    use super::{parse_blocks_in_source, parse_model_blocks};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn parse_model_block_attributes_from_direct_scope() {
        let source = r#"
            part def Example :> RosParameter {
                attribute :>> name = "use_sim_time";
                part nested {
                    attribute :>> name = "ignore_this";
                }
                attribute :>> defaultBool = true;
            }
        "#;
        let blocks = parse_blocks_in_source(source);
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0].attrs.get("name").map(|value| value.as_str()),
            Some("use_sim_time")
        );
        assert_eq!(
            blocks[0].attrs.get("defaultBool").map(|value| value.as_str()),
            Some("true")
        );
    }

    #[test]
    fn parse_model_blocks_reads_all_inputs() {
        let dir = tempdir().expect("tempdir");
        let first = dir.path().join("a.sysml");
        let second = dir.path().join("b.sysml");
        fs::write(&first, "part def A :> RosPackage { attribute :>> name = \"a\"; }\n")
            .expect("write");
        fs::write(
            &second,
            "item def Msg :> RosMessageType { attribute :>> name = \"x/Y\"; }\n",
        )
        .expect("write");
        let blocks = parse_model_blocks(&[first, second]).expect("parse");
        assert_eq!(blocks.len(), 2);
    }
}

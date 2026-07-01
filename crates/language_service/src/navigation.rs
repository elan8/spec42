use sysml_model::{
    resolve_expression_endpoint_strict, resolve_type_reference_targets, ResolveResult,
    TextPosition, TextRange, UnitRegistry,
};

use crate::dto::HoverResult;
use crate::keywords::keyword_hover_markdown;
use crate::lookup::collect_symbol_matches_for_lookup;
use crate::presentation_hover::hover_markdown_for_node;
use crate::references::TYPE_LOOKUP_ELEMENT_KINDS;
use crate::symbol::symbol_hover_markdown;
use crate::text::{unit_value_suffix_at_position, word_at_position};
use crate::workspace::WorkspaceSnapshot;

pub fn hover_at_position(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<HoverResult> {
    let uri = workspace.resolve_uri_for_path(path)?;
    let uri_norm = workspace.normalize_uri(&uri);
    let text = workspace.document_text(&uri_norm)?.to_string();
    let (line, char_start, char_end, word) =
        word_at_position(&text, position.line, position.character)?;
    let lookup_name = word
        .rsplit("::")
        .next()
        .map(str::to_string)
        .unwrap_or_else(|| word.clone());
    let qualifier = word.rsplit_once("::").map(|(q, _)| q.to_string());
    let range = Some(TextRange {
        start: TextPosition {
            line,
            character: char_start,
        },
        end: TextPosition {
            line,
            character: char_end,
        },
    });

    if let Some(md) = keyword_hover_markdown(&lookup_name.to_lowercase()) {
        return Some(HoverResult {
            contents: md,
            range,
        });
    }

    if let Some(md) = unit_literal_hover_markdown(workspace, &text, position) {
        return Some(HoverResult {
            contents: md,
            range,
        });
    }

    let graph = workspace.semantic_graph();
    if let Some(node) = graph.find_deepest_node_at_position(&uri_norm, position) {
        let target_match = graph
            .outgoing_typing_or_specializes_targets(node)
            .into_iter()
            .find(|target| {
                target.name == lookup_name
                    || target
                        .id
                        .qualified_name
                        .ends_with(&format!("::{}", lookup_name))
            });
        let markdown = if let Some(target) = target_match.as_ref() {
            hover_markdown_for_node(graph, target, target.id.uri != uri_norm)
        } else {
            hover_markdown_for_node(graph, node, node.id.uri != uri_norm)
        };
        let markdown = if target_match.is_none() && word != node.name {
            match resolve_hover_type_reference_target(workspace, node, &word, &lookup_name) {
                Some(target) => {
                    hover_markdown_for_node(graph, target, target.id.uri != uri_norm)
                }
                None => unit_literal_hover_markdown(workspace, &text, position).unwrap_or_else(
                    || {
                        format!(
                            "**Unresolved reference** `{}`\n\nSpec42 could not resolve this name in the current scope, imports, or indexed workspace symbols.",
                            lookup_name
                        )
                    },
                ),
            }
        } else {
            markdown
        };
        return Some(HoverResult {
            contents: markdown,
            range,
        });
    }

    if let Some(target) = resolve_hover_reference_target(workspace, &uri_norm, position, &word) {
        return Some(HoverResult {
            contents: hover_markdown_for_node(
                graph,
                target,
                target.id.uri != uri_norm,
            ),
            range,
        });
    }

    let (same_file, other_files) =
        collect_symbol_matches_for_lookup(workspace, &uri_norm, &lookup_name, qualifier.as_deref());
    let all_matches = if same_file.is_empty() {
        &other_files
    } else {
        &same_file
    };
    if let Some(entry) = all_matches.first() {
        let value = if all_matches.len() > 1 {
            let mut md = format!(
                "**{}** - {} definitions (use Go to Definition to choose):\n\n",
                lookup_name,
                all_matches.len()
            );
            for entry in all_matches {
                let kind = entry.detail.as_deref().unwrap_or("element");
                let container = entry.container_name.as_deref().unwrap_or("(top level)");
                md.push_str(&format!("- `{}` in `{}`\n", kind, container));
            }
            md.push('\n');
            md.push_str(&symbol_hover_markdown(entry, entry.uri != uri_norm));
            md
        } else {
            symbol_hover_markdown(entry, entry.uri != uri_norm)
        };
        return Some(HoverResult {
            contents: value,
            range,
        });
    }

    if let Some(md) = unit_literal_hover_markdown(workspace, &text, position) {
        return Some(HoverResult {
            contents: md,
            range,
        });
    }

    Some(HoverResult {
        contents: format!(
            "**Unresolved reference** `{}`\n\nSpec42 could not resolve this name in the current scope, imports, or indexed workspace symbols.",
            lookup_name
        ),
        range,
    })
}

fn resolve_hover_type_reference_target<'a>(
    workspace: &'a impl WorkspaceSnapshot,
    node: &sysml_model::SemanticNode,
    word: &str,
    lookup_name: &str,
) -> Option<&'a sysml_model::SemanticNode> {
    let graph = workspace.semantic_graph();
    let mut candidates = Vec::<String>::new();
    let mut push_candidate = |candidate: String| {
        if !candidate.is_empty() && !candidates.iter().any(|existing| existing == &candidate) {
            candidates.push(candidate);
        }
    };

    push_candidate(word.to_string());
    if lookup_name != word {
        push_candidate(lookup_name.to_string());
    }

    if word.contains("::") {
        for ancestor in graph.ancestors_of(node) {
            push_candidate(format!("{}::{}", ancestor.id.qualified_name, word));
        }
    }

    for candidate in candidates {
        if let Some(target_id) = resolve_type_reference_targets(
            graph,
            node,
            &candidate,
            TYPE_LOOKUP_ELEMENT_KINDS,
        )
        .into_iter()
        .next()
        {
            if let Some(target) = graph.get_node(&target_id) {
                return Some(target);
            }
        }
    }

    None
}

fn resolve_hover_reference_target<'a>(
    workspace: &'a impl WorkspaceSnapshot,
    uri: &url::Url,
    pos: TextPosition,
    word: &str,
) -> Option<&'a sysml_model::SemanticNode> {
    let graph = workspace.semantic_graph();
    let context_node = graph
        .find_deepest_node_at_position(uri, pos)
        .or_else(|| {
            graph
                .nodes_for_uri(uri)
                .into_iter()
                .find(|n| n.name == word)
        })?;

    let mut prefixes = Vec::<Option<String>>::new();
    prefixes.push(Some(context_node.id.qualified_name.clone()));
    if let Some(parent_id) = &context_node.parent_id {
        prefixes.push(Some(parent_id.qualified_name.clone()));
    }
    for ancestor in graph.ancestors_of(context_node) {
        prefixes.push(Some(ancestor.id.qualified_name.clone()));
    }
    prefixes.push(None);

    for prefix in prefixes {
        let resolved = resolve_expression_endpoint_strict(graph, uri, prefix.as_deref(), word);
        if let ResolveResult::Resolved(target_id) = resolved {
            if let Some(target) = graph.get_node(&target_id) {
                return Some(target);
            }
        }
    }

    None
}

fn unit_registry_for_hover(workspace: &impl WorkspaceSnapshot) -> UnitRegistry {
    UnitRegistry::from_graph(workspace.semantic_graph())
}

fn unit_literal_hover_markdown(
    workspace: &impl WorkspaceSnapshot,
    text: &str,
    pos: TextPosition,
) -> Option<String> {
    let unit_expr = unit_value_suffix_at_position(text, pos.line, pos.character)?;
    let registry = unit_registry_for_hover(workspace);
    Some(
        registry
            .hover_markdown_for_unit_literal(&unit_expr)
            .unwrap_or_else(|| UnitRegistry::hover_markdown_for_unknown_unit_literal(&unit_expr)),
    )
}

pub fn hover(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> Option<HoverResult> {
    hover_at_position(workspace, path, position)
}

pub fn goto_definition(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
) -> crate::dto::DefinitionResult {
    crate::references::goto_definition_at_position(workspace, path, position)
}

pub fn find_references(
    workspace: &impl WorkspaceSnapshot,
    path: &str,
    position: TextPosition,
    include_declaration: bool,
) -> crate::dto::ReferencesResult {
    crate::references::find_references_at_position(workspace, path, position, include_declaration)
}

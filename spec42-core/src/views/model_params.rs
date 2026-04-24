use std::time::Instant;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::Url;

use crate::common::util;
use crate::views::dto::{SysmlGraphDto, SysmlModelResultDto, SysmlModelStatsDto};

/// Parse sysml/model params from JSON-RPC value.
pub fn parse_sysml_model_params(v: &serde_json::Value) -> Result<(Url, Vec<String>)> {
    let (uri_str, scope_value) = if let Some(arr) = v.as_array() {
        let first = arr.first().ok_or_else(|| {
            tower_lsp::jsonrpc::Error::invalid_params(
                "sysml/model params array must have at least one element",
            )
        })?;
        let uri_str = if let Some(s) = first.as_str() {
            Some(s.to_string())
        } else if let Some(obj) = first.as_object() {
            obj.get("uri")
                .and_then(|u| u.as_str())
                .map(String::from)
                .or_else(|| {
                    obj.get("textDocument")
                        .and_then(|td| td.get("uri"))
                        .and_then(|u| u.as_str())
                        .map(String::from)
                })
        } else {
            None
        };
        let scope_value = arr.get(1);
        (uri_str, scope_value)
    } else if let Some(obj) = v.as_object() {
        let uri_str = obj
            .get("uri")
            .and_then(|u| u.as_str())
            .map(String::from)
            .or_else(|| {
                obj.get("textDocument")
                    .and_then(|td| td.get("uri"))
                    .and_then(|u| u.as_str())
                    .map(String::from)
            });
        let scope_value = obj.get("scope");
        (uri_str, scope_value)
    } else {
        return Err(tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/model params must be an object or array",
        ));
    };

    let uri = uri_str.as_ref().ok_or_else(|| {
        tower_lsp::jsonrpc::Error::invalid_params(
            "sysml/model requires 'uri' or 'textDocument.uri'",
        )
    })?;
    let uri = Url::parse(uri)
        .map_err(|_| tower_lsp::jsonrpc::Error::invalid_params("sysml/model: invalid URI"))?;
    let uri = util::normalize_file_uri(&uri);

    let scope: Vec<String> = scope_value
        .and_then(|s| serde_json::from_value(s.clone()).ok())
        .unwrap_or_default();

    Ok((uri, scope))
}

pub fn empty_model_response(build_start: Instant) -> SysmlModelResultDto {
    SysmlModelResultDto {
        version: 0,
        graph: Some(SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        }),
        software_architecture: None,
        package_groups: None,
        general_view_graph: Some(SysmlGraphDto {
            nodes: vec![],
            edges: vec![],
        }),
        workspace_model: None,
        activity_diagrams: None,
        sequence_diagrams: None,
        ibd: None,
        stats: Some(SysmlModelStatsDto {
            total_elements: 0,
            resolved_elements: 0,
            unresolved_elements: 0,
            parse_time_ms: 0,
            model_build_time_ms: build_start.elapsed().as_millis().max(1) as u32,
            parse_cached: false,
        }),
    }
}

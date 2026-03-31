use crate::config::Spec42Config;
use tower_lsp::lsp_types::*;

pub(crate) fn server_capabilities(config: &Spec42Config) -> ServerCapabilities {
    let mut capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions::default()),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec![":".to_string(), ",".to_string()]),
            retrigger_characters: None,
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        })),
        document_symbol_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
        document_link_provider: Some(DocumentLinkOptions {
            resolve_provider: Some(false),
            work_done_progress_options: WorkDoneProgressOptions::default(),
        }),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
        code_lens_provider: Some(CodeLensOptions {
            resolve_provider: Some(false),
        }),
        inlay_hint_provider: None,
        linked_editing_range_provider: Some(LinkedEditingRangeServerCapabilities::Simple(true)),
        document_formatting_provider: Some(OneOf::Left(true)),
        moniker_provider: Some(OneOf::Left(true)),
        call_hierarchy_provider: Some(CallHierarchyServerCapability::Simple(true)),
        experimental: Some(serde_json::json!({
            "typeHierarchyProvider": true
        })),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                work_done_progress_options: WorkDoneProgressOptions::default(),
                legend: crate::semantic_tokens::legend(),
                range: Some(true),
                full: Some(SemanticTokensFullOptions::Bool(true)),
            },
        )),
        ..ServerCapabilities::default()
    };

    if let Some(existing) = capabilities.experimental.take() {
        let extra_methods = config.extra_custom_method_names();
        capabilities.experimental = Some(serde_json::json!({
            "typeHierarchyProvider": existing.get("typeHierarchyProvider").and_then(|v| v.as_bool()).unwrap_or(false),
            "host": {
                "extraCustomMethods": extra_methods
            }
        }));
    }

    for augmenter in &config.capability_augmenters {
        augmenter.augment_capabilities(&mut capabilities);
    }

    capabilities
}

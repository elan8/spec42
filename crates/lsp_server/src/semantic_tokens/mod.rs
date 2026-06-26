//! LSP adapter over the shared sysml_tokens crate.

pub use sysml_tokens::{
    ast_semantic_ranges, legend_token_types, semantic_tokens_full as semantic_tokens_full_dto,
    semantic_tokens_range as semantic_tokens_range_dto, SemanticTokensDto, SourceRange,
};

use tower_lsp::lsp_types::{
    SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensLegend,
};

pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: legend_token_types()
            .iter()
            .map(|name| SemanticTokenType::new(name))
            .collect(),
        token_modifiers: vec![],
    }
}

pub fn semantic_tokens_full(
    text: &str,
    ast_ranges: Option<&[(SourceRange, u32)]>,
) -> (SemanticTokens, Vec<String>) {
    let (dto, logs) = semantic_tokens_full_dto(text, ast_ranges);
    (dto_to_lsp(dto), logs)
}

pub fn semantic_tokens_range(
    text: &str,
    start_line: u32,
    start_character: u32,
    end_line: u32,
    end_character: u32,
    ast_ranges: Option<&[(SourceRange, u32)]>,
) -> (SemanticTokens, Vec<String>) {
    let (dto, logs) = semantic_tokens_range_dto(
        text,
        start_line,
        start_character,
        end_line,
        end_character,
        ast_ranges,
    );
    (dto_to_lsp(dto), logs)
}

fn dto_to_lsp(dto: SemanticTokensDto) -> SemanticTokens {
    SemanticTokens {
        result_id: None,
        data: flat_to_semantic_tokens(&dto.data),
    }
}

fn flat_to_semantic_tokens(data: &[u32]) -> Vec<SemanticToken> {
    data.chunks(5)
        .map(|chunk| SemanticToken {
            delta_line: chunk[0],
            delta_start: chunk[1],
            length: chunk[2],
            token_type: chunk[3],
            token_modifiers_bitset: chunk.get(4).copied().unwrap_or(0),
        })
        .collect()
}

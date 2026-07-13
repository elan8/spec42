use super::*;

// ---------------------------------------------------------------------------
// Sequence diagrams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "SequenceDiagramDTO", optional_fields)]
pub struct SequenceDiagramDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub package_path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    pub source_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub lifelines: Vec<SequenceLifelineDto>,
    pub messages: Vec<SequenceMessageDto>,
    pub activations: Vec<SequenceActivationDto>,
    pub fragments: Vec<SequenceFragmentDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "SequenceLifelineDTO", optional_fields)]
pub struct SequenceLifelineDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "SequenceMessageDTO", optional_fields)]
pub struct SequenceMessageDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub order: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "SequenceActivationDTO", optional_fields)]
pub struct SequenceActivationDto {
    pub id: String,
    pub name: String,
    pub on_lifeline: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "SequenceOperandDTO", optional_fields)]
pub struct SequenceOperandDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    pub message_ids: Vec<String>,
    pub fragments: Vec<SequenceFragmentDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "SequenceFragmentDTO", optional_fields)]
pub struct SequenceFragmentDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    pub message_ids: Vec<String>,
    pub operands: Vec<SequenceOperandDto>,
    pub fragments: Vec<SequenceFragmentDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}


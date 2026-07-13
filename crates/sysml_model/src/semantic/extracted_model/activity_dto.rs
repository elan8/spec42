use super::*;

// ---------------------------------------------------------------------------
// Activity diagrams
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "ActivityDiagramDTO", optional_fields)]
pub struct ActivityDiagramDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub package_path: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    pub source_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub actions: Vec<ActivityActionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<ActivityInterfaceDto>,
    pub decisions: Vec<DecisionNodeDto>,
    pub flows: Vec<ControlFlowDto>,
    pub states: Vec<ActivityStateDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "ActivityActionDTO", optional_fields)]
pub struct ActivityActionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub action_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swim_lane: Option<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "ActivityInterfaceDTO")]
pub struct ActivityInterfaceDto {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "DecisionNodeDTO")]
pub struct DecisionNodeDto {
    pub name: String,
    pub condition: String,
    pub branches: Vec<BranchDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(rename = "BranchDTO")]
pub struct BranchDto {
    pub condition: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "ControlFlowDTO", optional_fields)]
pub struct ControlFlowDto {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "ActivityStateDTO")]
pub struct ActivityStateDto {
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
    pub range: RangeDto,
}

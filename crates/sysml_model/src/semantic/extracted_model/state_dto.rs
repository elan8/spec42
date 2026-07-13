use super::*;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "StateMachineDTO", optional_fields)]
pub struct StateMachineDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    pub package_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub states: Vec<StateNodeDto>,
    pub transitions: Vec<StateTransitionDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<RegionDto>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "RegionDTO", optional_fields)]
pub struct RegionDto {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "StateNodeDTO", optional_fields)]
pub struct StateNodeDto {
    pub id: String,
    pub name: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    #[serde(rename = "do", skip_serializing_if = "Option::is_none")]
    pub do_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit: Option<String>,
    pub element: StateNodeElementDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "StateNodeElementDTO", optional_fields)]
pub struct StateNodeElementDto {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub element_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "StateTransitionDTO", optional_fields)]
pub struct StateTransitionDto {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send: Option<String>,
    pub self_loop: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub range: RangeDto,
}

//! IBD DTO types shared across extraction, merge, and visualization.

use serde::Serialize;
use std::collections::HashMap;
use ts_rs::TS;

use crate::semantic::dto::RangeDto;

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdPartDTO")]
pub struct IbdPartDto {
    pub id: String,
    pub node_id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub container_id: Option<String>,
    #[serde(rename = "type")]
    pub element_type: String,
    pub attributes: HashMap<String, serde_json::Value>,
    /// Source location of the declaring element, for click-to-source navigation. Absent for
    /// synthetic/expanded nodes with no single declaring occurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdPortDTO")]
pub struct IbdPortDto {
    pub id: String,
    pub port_id: String,
    pub name: String,
    pub parent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub port_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub port_side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub uri: Option<String>,
    /// Source location of the declaring port, for click-to-source navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub range: Option<RangeDto>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdConnectorDTO")]
pub struct IbdConnectorDto {
    pub source: String,
    pub target: String,
    pub source_id: String,
    pub target_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub source_part_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub target_part_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub source_port_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub target_port_id: Option<String>,
    #[serde(rename = "type")]
    pub rel_type: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdContainerGroupDTO")]
pub struct IbdContainerGroupDto {
    pub id: String,
    pub label: String,
    pub depth: usize,
    pub qualified_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub parent_id: Option<String>,
    pub member_part_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdPackageContainerGroupDTO")]
pub struct IbdPackageContainerGroupDto {
    pub id: String,
    pub label: String,
    pub qualified_package: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub parent_id: Option<String>,
    pub member_part_ids: Vec<String>,
}

/// A definition-to-instance root mapping derived from real typing edges (not name heuristics),
/// e.g. `("PhysicalArchitecture.AutonomousFloorCleaningRobot", "Architecture.CleaningRobotSystemOfInterest.physical")`.
/// Lets an exposed view id that names a definition-nested member be translated to the concrete
/// instance path(s) actually carrying mirrored connectors.
#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdDefInstanceMappingDTO")]
pub struct DefInstanceMappingDto {
    pub def_root: String,
    pub instance_root: String,
}

#[derive(Debug, Clone, Default, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdDataDTO")]
pub struct IbdDataDto {
    pub parts: Vec<IbdPartDto>,
    pub ports: Vec<IbdPortDto>,
    pub connectors: Vec<IbdConnectorDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub def_instance_mappings: Vec<DefInstanceMappingDto>,
    pub container_groups: Vec<IbdContainerGroupDto>,
    pub package_container_groups: Vec<IbdPackageContainerGroupDto>,
    pub root_candidates: Vec<String>,
    pub default_root: Option<String>,
    pub root_views: HashMap<String, IbdRootViewDto>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename = "IbdRootViewDTO")]
pub struct IbdRootViewDto {
    pub parts: Vec<IbdPartDto>,
    pub ports: Vec<IbdPortDto>,
    pub connectors: Vec<IbdConnectorDto>,
    pub container_groups: Vec<IbdContainerGroupDto>,
    pub package_container_groups: Vec<IbdPackageContainerGroupDto>,
}

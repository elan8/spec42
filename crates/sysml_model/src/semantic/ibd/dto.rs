//! IBD DTO types shared across extraction, merge, and visualization.

use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdPartDto {
    pub id: String,
    pub node_id: String,
    pub name: String,
    pub qualified_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "type")]
    pub element_type: String,
    pub attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdPortDto {
    pub id: String,
    pub port_id: String,
    pub name: String,
    pub parent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_side: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdConnectorDto {
    pub source: String,
    pub target: String,
    pub source_id: String,
    pub target_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_part_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_part_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_port_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port_id: Option<String>,
    #[serde(rename = "type")]
    pub rel_type: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdContainerGroupDto {
    pub id: String,
    pub label: String,
    pub depth: usize,
    pub qualified_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub member_part_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdPackageContainerGroupDto {
    pub id: String,
    pub label: String,
    pub qualified_package: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub member_part_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdDataDto {
    pub parts: Vec<IbdPartDto>,
    pub ports: Vec<IbdPortDto>,
    pub connectors: Vec<IbdConnectorDto>,
    pub container_groups: Vec<IbdContainerGroupDto>,
    pub package_container_groups: Vec<IbdPackageContainerGroupDto>,
    pub root_candidates: Vec<String>,
    pub default_root: Option<String>,
    pub root_views: HashMap<String, IbdRootViewDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IbdRootViewDto {
    pub parts: Vec<IbdPartDto>,
    pub ports: Vec<IbdPortDto>,
    pub connectors: Vec<IbdConnectorDto>,
    pub container_groups: Vec<IbdContainerGroupDto>,
    pub package_container_groups: Vec<IbdPackageContainerGroupDto>,
}

use super::*;

// ---------------------------------------------------------------------------
// Extraction
// ---------------------------------------------------------------------------

pub(crate) fn join_segments(segments: &[String]) -> String {
    segments
        .iter()
        .filter(|segment| !segment.trim().is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join("::")
}

pub(crate) fn with_segment(base: &[String], segment: String) -> Vec<String> {
    if segment.trim().is_empty() {
        base.to_vec()
    } else {
        let mut next = base.to_vec();
        next.push(segment);
        next
    }
}

pub(crate) fn activity_diagram_id(qualified_segments: &[String], source_kind: &str) -> String {
    let qualified = join_segments(qualified_segments);
    if qualified.is_empty() {
        source_kind.to_string()
    } else {
        format!("{qualified}::{source_kind}")
    }
}

pub(crate) fn package_path_from_segments(package_segments: &[String]) -> String {
    join_segments(package_segments)
}

pub(crate) fn extract_performer_diagram_from_performs(
    name: &str,
    qualified_segments: &[String],
    package_segments: &[String],
    performs: &[sysml_v2_parser::Node<sysml_v2_parser::ast::Perform>],
    range: RangeDto,
) -> Option<ActivityDiagramDto> {
    if performs.is_empty() {
        return None;
    }

    let actions = performs
        .iter()
        .enumerate()
        .map(|(index, perform)| {
            let perform_name = if perform.value.action_name.trim().is_empty() {
                perform
                    .value
                    .type_name
                    .clone()
                    .unwrap_or_else(|| format!("perform_{}", index + 1))
            } else {
                perform.value.action_name.clone()
            };
            ActivityActionDto {
                id: Some(format!(
                    "{}::{}",
                    activity_diagram_id(qualified_segments, "performer"),
                    perform_name
                )),
                name: perform_name,
                action_type: "action".to_string(),
                kind: Some("perform".to_string()),
                inputs: None,
                outputs: None,
                range: Some(span_to_range_dto(&perform.span)),
                uri: None,
                swim_lane: None,
            }
        })
        .collect::<Vec<_>>();

    let flows = actions
        .windows(2)
        .enumerate()
        .map(|(index, window)| ControlFlowDto {
            from: window[0].name.clone(),
            to: window[1].name.clone(),
            condition: None,
            guard: Some("perform-sequence".to_string()),
            range: performs
                .get(index + 1)
                .map(|perform| span_to_range_dto(&perform.span))
                .unwrap_or_else(|| range.clone()),
        })
        .collect::<Vec<_>>();

    Some(ActivityDiagramDto {
        id: activity_diagram_id(qualified_segments, "performer"),
        name: if name.trim().is_empty() {
            "performer".to_string()
        } else {
            name.to_string()
        },
        package_path: package_path_from_segments(package_segments),
        label: String::new(),
        source_kind: "performer".to_string(),
        uri: None,
        actions,
        interface: None,
        decisions: vec![],
        flows,
        states: vec![],
        range,
    })
}

pub(crate) fn extract_performer_diagram_from_part_def(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartDef>,
    package_segments: &[String],
    parent_segments: &[String],
) -> Option<ActivityDiagramDto> {
    let name = identification_name(&node.identification);
    let qualified_segments = with_segment(parent_segments, name.clone());
    let elements = match &node.body {
        PartDefBody::Brace { elements } => elements,
        PartDefBody::Semicolon => return None,
    };
    let performs = elements
        .iter()
        .filter_map(|element| match &element.value {
            PartDefBodyElement::Perform(perform) => Some(perform.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    extract_performer_diagram_from_performs(
        &name,
        &qualified_segments,
        package_segments,
        &performs,
        span_to_range_dto(&node.span),
    )
}

pub(crate) fn extract_performer_diagram_from_part_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    package_segments: &[String],
    parent_segments: &[String],
) -> Option<ActivityDiagramDto> {
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());
    let elements = match &node.body {
        PartUsageBody::Brace { elements } => elements,
        PartUsageBody::Semicolon => return None,
    };
    let performs = elements
        .iter()
        .filter_map(|element| match &element.value {
            PartUsageBodyElement::Perform(perform) => Some(perform.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    extract_performer_diagram_from_performs(
        &name,
        &qualified_segments,
        package_segments,
        &performs,
        span_to_range_dto(&node.span),
    )
}

pub(crate) fn collect_activity_diagrams_from_part_def_body(
    elements: &[sysml_v2_parser::Node<PartDefBodyElement>],
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    for element in elements {
        if let PartDefBodyElement::PartUsage(part_usage) = &element.value {
            collect_activity_diagrams_from_part_usage(
                part_usage,
                package_segments,
                parent_segments,
                out,
            );
        }
    }
}

pub(crate) fn collect_activity_diagrams_from_part_usage_body(
    elements: &[sysml_v2_parser::Node<PartUsageBodyElement>],
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    for element in elements {
        if let PartUsageBodyElement::PartUsage(part_usage) = &element.value {
            collect_activity_diagrams_from_part_usage(
                part_usage,
                package_segments,
                parent_segments,
                out,
            );
        }
    }
}

pub(crate) fn collect_activity_diagrams_from_part_def(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartDef>,
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    let name = identification_name(&node.identification);
    let qualified_segments = with_segment(parent_segments, name.clone());

    if let Some(diagram) =
        extract_performer_diagram_from_part_def(node, package_segments, parent_segments)
    {
        out.push(diagram);
    }

    if let PartDefBody::Brace { elements } = &node.body {
        collect_activity_diagrams_from_part_def_body(
            elements,
            package_segments,
            &qualified_segments,
            out,
        );
    }
}

pub(crate) fn collect_activity_diagrams_from_part_usage(
    node: &sysml_v2_parser::Node<sysml_v2_parser::ast::PartUsage>,
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    let name = node.value.name.clone();
    let qualified_segments = with_segment(parent_segments, name.clone());

    if let Some(diagram) =
        extract_performer_diagram_from_part_usage(node, package_segments, parent_segments)
    {
        out.push(diagram);
    }

    if let PartUsageBody::Brace { elements } = &node.body {
        collect_activity_diagrams_from_part_usage_body(
            elements,
            package_segments,
            &qualified_segments,
            out,
        );
    }
}

pub(crate) fn collect_activity_diagrams_from_package_elements(
    elements: &[sysml_v2_parser::Node<PackageBodyElement>],
    package_segments: &[String],
    parent_segments: &[String],
    out: &mut Vec<ActivityDiagramDto>,
) {
    use sysml_v2_parser::ast::PackageBodyElement as PBE;
    for node in elements {
        match &node.value {
            PBE::ActionDef(action) => out.push(extract_activity_from_action(
                action,
                package_segments,
                parent_segments,
            )),
            PBE::PartDef(part_def) => {
                collect_activity_diagrams_from_part_def(
                    part_def,
                    package_segments,
                    parent_segments,
                    out,
                );
            }
            PBE::PartUsage(part_usage) => {
                collect_activity_diagrams_from_part_usage(
                    part_usage,
                    package_segments,
                    parent_segments,
                    out,
                );
            }
            PBE::Package(package) => {
                if let PackageBody::Brace { elements: inner } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let next_package_segments =
                        with_segment(package_segments, package_name.clone());
                    let next_parent_segments = with_segment(parent_segments, package_name);
                    collect_activity_diagrams_from_package_elements(
                        inner,
                        &next_package_segments,
                        &next_parent_segments,
                        out,
                    );
                }
            }
            PBE::LibraryPackage(package) => {
                if let PackageBody::Brace { elements: inner } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let next_package_segments =
                        with_segment(package_segments, package_name.clone());
                    let next_parent_segments = with_segment(parent_segments, package_name);
                    collect_activity_diagrams_from_package_elements(
                        inner,
                        &next_package_segments,
                        &next_parent_segments,
                        out,
                    );
                }
            }
            _ => {}
        }
    }
}

/// Extracts activity diagrams from ActionDef nodes.
/// Each ActionDef or performer context becomes one ActivityDiagramDto.
pub fn extract_activity_diagrams(root: &RootNamespace) -> Vec<ActivityDiagramDto> {
    let mut out = Vec::new();
    for node in &root.elements {
        match &node.value {
            RootElement::Package(package) => {
                if let PackageBody::Brace { elements } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let package_segments = if package_name.is_empty() {
                        vec![]
                    } else {
                        vec![package_name]
                    };
                    collect_activity_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::Namespace(namespace) => {
                if let PackageBody::Brace { elements } = &namespace.body {
                    let namespace_name = identification_name(&namespace.identification);
                    let package_segments = if namespace_name.is_empty() {
                        vec![]
                    } else {
                        vec![namespace_name]
                    };
                    collect_activity_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::LibraryPackage(package) => {
                if let PackageBody::Brace { elements } = &package.body {
                    let package_name = identification_name(&package.identification);
                    let package_segments = if package_name.is_empty() {
                        vec![]
                    } else {
                        vec![package_name]
                    };
                    collect_activity_diagrams_from_package_elements(
                        elements,
                        &package_segments,
                        &package_segments,
                        &mut out,
                    );
                }
            }
            RootElement::Import(_) => {}
        }
    }
    out
}

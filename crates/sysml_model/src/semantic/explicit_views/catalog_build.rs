use super::*;

pub(crate) fn walk_root_namespace(
    root: &RootNamespace,
    content: &str,
    container: Option<&str>,
    definitions: &mut HashMap<String, ViewDefinitionSpec>,
    usages: &mut Vec<ViewUsageSpec>,
) {
    for element in &root.elements {
        match &element.value {
            RootElement::Package(package) => {
                walk_package_body(
                    &package.identification,
                    &package.body,
                    content,
                    container,
                    definitions,
                    usages,
                );
            }
            RootElement::Namespace(namespace) => {
                walk_package_body(
                    &namespace.identification,
                    &namespace.body,
                    content,
                    container,
                    definitions,
                    usages,
                );
            }
            _ => {}
        }
    }
}

pub(crate) fn walk_package_body(
    identification: &Identification,
    body: &PackageBody,
    content: &str,
    container: Option<&str>,
    definitions: &mut HashMap<String, ViewDefinitionSpec>,
    usages: &mut Vec<ViewUsageSpec>,
) {
    let next_container = identification
        .name
        .as_deref()
        .map(|name| qualify_name(container, name))
        .or_else(|| container.map(str::to_string));

    let PackageBody::Brace { elements } = body else {
        return;
    };

    for element in elements {
        match &element.value {
            PackageBodyElement::Package(package) => walk_package_body(
                &package.identification,
                &package.body,
                content,
                next_container.as_deref(),
                definitions,
                usages,
            ),
            PackageBodyElement::ViewDef(view_def) => {
                let name = identification_name(&view_def.identification);
                let id = qualify_name(next_container.as_deref(), &name);
                let (filters, rendering_ref, rendering_type) = match &view_def.body {
                    ViewDefBody::Brace { elements } => {
                        let filters = elements
                            .iter()
                            .filter_map(|member| match &member.value {
                                ViewDefBodyElement::Filter(filter) => {
                                    Some(parse_filter_span(content, &filter.condition.span))
                                }
                                _ => None,
                            })
                            .collect();
                        let (rendering_ref, rendering_type) =
                            extract_rendering_from_view_def_body(elements);
                        (filters, rendering_ref, rendering_type)
                    }
                    ViewDefBody::Semicolon => (Vec::new(), None, None),
                };
                definitions.insert(
                    id.clone(),
                    ViewDefinitionSpec {
                        id,
                        name,
                        filters,
                        rendering_ref,
                        rendering_type,
                    },
                );
            }
            PackageBodyElement::ViewUsage(view_usage) => {
                let id = qualify_name(next_container.as_deref(), &view_usage.name);
                let mut filters = Vec::new();
                let mut exposes = Vec::new();
                let mut rendering_ref = None;
                let mut rendering_type = None;
                if let ViewBody::Brace { elements } = &view_usage.body {
                    for member in elements {
                        match &member.value {
                            ViewBodyElement::Filter(filter) => {
                                filters.push(parse_filter_span(content, &filter.condition.span));
                            }
                            ViewBodyElement::Expose(expose) => exposes.push(ExposeSpec {
                                target: expose.target.clone(),
                                filter: parse_expose_filter(content, &member.span),
                                range: span_to_range_dto(&member.span),
                            }),
                            ViewBodyElement::ViewRendering(rendering)
                                if rendering_ref.is_none() =>
                            {
                                rendering_ref = Some(rendering.value.name.clone());
                                rendering_type = rendering.value.type_name.clone();
                            }
                            _ => {}
                        }
                    }
                }
                usages.push(ViewUsageSpec {
                    id,
                    name: view_usage.name.clone(),
                    definition_ref: view_usage.type_name.clone(),
                    definition_id: None,
                    filters,
                    exposes,
                    conforms_to: Vec::new(),
                    rendering_ref,
                    rendering_type,
                    range: span_to_range_dto(&view_usage.span),
                    issues: Vec::new(),
                });
            }
            _ => {}
        }
    }
}

pub(crate) fn identification_name(identification: &Identification) -> String {
    identification
        .name
        .clone()
        .or_else(|| identification.short_name.clone())
        .unwrap_or_else(|| "AnonymousView".to_string())
}

pub(crate) fn qualify_name(container: Option<&str>, name: &str) -> String {
    match container {
        Some(prefix) if !prefix.is_empty() => format!("{prefix}::{name}"),
        _ => name.to_string(),
    }
}

pub(crate) fn resolve_definition_id(
    reference: &str,
    definitions: &HashMap<String, ViewDefinitionSpec>,
) -> Option<String> {
    let normalized = normalize_path(reference);
    let mut matches: Vec<_> = definitions
        .keys()
        .filter(|id| {
            let candidate = normalize_path(id);
            candidate == normalized || candidate.ends_with(&format!("::{normalized}"))
        })
        .cloned()
        .collect();
    matches.sort();
    if matches.len() == 1 {
        matches.into_iter().next()
    } else {
        None
    }
}

pub(crate) fn extract_rendering_from_view_def_body(
    elements: &[sysml_v2_parser::ast::Node<ViewDefBodyElement>],
) -> (Option<String>, Option<String>) {
    for member in elements {
        if let ViewDefBodyElement::ViewRendering(rendering) = &member.value {
            return (
                Some(rendering.value.name.clone()),
                rendering.value.type_name.clone(),
            );
        }
    }
    (None, None)
}

pub(crate) fn resolve_explicit_view_type(usage: &ViewUsageSpec, _catalog: &ViewCatalog) -> Option<String> {
    if usage.definition_id.is_some() {
        return None;
    }
    let type_ref = usage.definition_ref.as_deref()?;
    if renderer_view_for_view_type(Some(type_ref)).is_some() {
        Some(type_ref.to_string())
    } else {
        None
    }
}

pub(crate) fn view_type_for_stdlib_rendering(
    rendering_ref: Option<&str>,
    rendering_type: Option<&str>,
) -> Option<&'static str> {
    let lookup = rendering_type.or(rendering_ref)?;
    match normalize_kind_name(lookup).as_str() {
        "asinterconnectiondiagram" => Some("InterconnectionView"),
        "astreediagram" => Some("BrowserView"),
        "aselementtable" => Some("GridView"),
        "astextualnotation" => Some("GeneralView"),
        _ => None,
    }
}

pub(crate) fn resolve_effective_view_type(usage: &ViewUsageSpec, catalog: &ViewCatalog) -> String {
    if let Some(explicit) = resolve_explicit_view_type(usage, catalog) {
        return explicit;
    }
    if usage.definition_id.is_none() {
        if let Some(type_ref) = usage.definition_ref.as_deref() {
            if crate::semantic::standard_views::is_non_standard_explicit_view_type(type_ref) {
                return type_ref.to_string();
            }
        }
    }
    view_type_for_stdlib_rendering(
        usage.rendering_ref.as_deref(),
        usage.rendering_type.as_deref(),
    )
    .or_else(|| {
        usage.definition_id.as_deref().and_then(|definition_id| {
            catalog
                .definitions
                .get(definition_id)
                .and_then(|definition| {
                    view_type_for_stdlib_rendering(
                        definition.rendering_ref.as_deref(),
                        definition.rendering_type.as_deref(),
                    )
                })
        })
    })
    .map(str::to_string)
    .unwrap_or_else(|| "GeneralView".to_string())
}

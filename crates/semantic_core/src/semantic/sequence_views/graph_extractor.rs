//! Graph-driven sequence diagram extraction.
//!
//! Scenarios, lifelines, messages, activations, and fragments are identified by
//! walking the merged `SemanticGraph` rather than inspecting per-file ASTs. The
//! specialization closure of well-known anchor types (e.g.
//! `InteractionScenario`, `Lifeline`, `SynchronousCall`) is derived from the
//! resolved `Specializes` edges, with the original `specializes` attribute
//! string used as a fallback when an anchor has no defining `part def` in the
//! workspace.

use std::collections::{HashMap, HashSet};

use tower_lsp::lsp_types::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind, SemanticNode};
use crate::semantic::extracted_model::{
    PositionDto, RangeDto, SequenceActivationDto, SequenceDiagramDto, SequenceFragmentDto,
    SequenceLifelineDto, SequenceMessageDto, SequenceOperandDto,
};

const SCENARIO_ANCHOR: &str = "InteractionScenario";
const LIFELINE_ANCHOR: &str = "Lifeline";
const ACTIVATION_ANCHOR: &str = "Activation";
const OPERAND_ANCHOR: &str = "InteractionOperand";

/// (anchor name, kind discriminator used in the DTO).
const MESSAGE_KINDS: &[(&str, &str)] = &[
    ("SynchronousCall", "sync"),
    ("AsynchronousMessage", "async"),
    ("ReturnMessage", "return"),
    ("CreationMessage", "create"),
];

const FRAGMENT_KINDS: &[(&str, &str)] = &[
    ("AltFragment", "alt"),
    ("OptFragment", "opt"),
    ("LoopFragment", "loop"),
    ("InteractionRef", "ref"),
    ("CombinedFragment", "group"),
];

/// Public entry point: build all sequence diagrams reachable from the workspace
/// URIs. Library/dependency URIs (anything outside `workspace_uris`) are
/// excluded from the scenario search but their nodes still participate in
/// closure construction so cross-package specialization is recognised.
pub(super) fn extract_sequence_diagrams(
    graph: &SemanticGraph,
    workspace_uris: &[Url],
) -> Vec<SequenceDiagramDto> {
    let closures = NameClosures::build(graph);
    let workspace: HashSet<&Url> = workspace_uris.iter().collect();

    let mut diagrams = Vec::new();
    let mut seen_scenarios: HashSet<NodeId> = HashSet::new();

    for uri in workspace_uris {
        for node in graph.nodes_for_uri(uri) {
            if !is_scenario_node(node, &closures) {
                continue;
            }
            if !seen_scenarios.insert(node.id.clone()) {
                continue;
            }
            if let Some(diagram) = build_diagram(graph, node, &closures, &workspace) {
                diagrams.push(diagram);
            }
        }
    }
    diagrams
}

// ---------------------------------------------------------------------------
// Name-based specialization closures
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct NameClosures {
    scenario: HashSet<String>,
    lifeline: HashSet<String>,
    activation: HashSet<String>,
    operand: HashSet<String>,
    /// `(closure, kind discriminator)` pairs for messages.
    messages: Vec<(HashSet<String>, &'static str)>,
    fragments: Vec<(HashSet<String>, &'static str)>,
}

impl NameClosures {
    fn build(graph: &SemanticGraph) -> Self {
        let part_def_specializes = collect_part_def_specializes(graph);
        let scenario = build_name_closure(&part_def_specializes, SCENARIO_ANCHOR);
        let lifeline = build_name_closure(&part_def_specializes, LIFELINE_ANCHOR);
        let activation = build_name_closure(&part_def_specializes, ACTIVATION_ANCHOR);
        let operand = build_name_closure(&part_def_specializes, OPERAND_ANCHOR);

        let messages = MESSAGE_KINDS
            .iter()
            .map(|(anchor, kind)| (build_name_closure(&part_def_specializes, anchor), *kind))
            .collect();
        let fragments = FRAGMENT_KINDS
            .iter()
            .map(|(anchor, kind)| (build_name_closure(&part_def_specializes, anchor), *kind))
            .collect();

        Self {
            scenario,
            lifeline,
            activation,
            operand,
            messages,
            fragments,
        }
    }

    fn message_kind_for(&self, type_name: &str) -> Option<&'static str> {
        let simple = simple_name(type_name);
        self.messages.iter().find_map(|(set, kind)| {
            if set.contains(&simple) {
                Some(*kind)
            } else {
                None
            }
        })
    }

    fn fragment_kind_for(&self, type_name: &str) -> Option<&'static str> {
        let simple = simple_name(type_name);
        self.fragments.iter().find_map(|(set, kind)| {
            if set.contains(&simple) {
                Some(*kind)
            } else {
                None
            }
        })
    }
}

/// Map of `part def` simple-name → the simple-name of its declared `specializes`
/// target (empty string if none). Names that occur as part defs but appear
/// unspecialised still need to be present as keys so the closure walk can
/// discover specialisation chains across multiple files.
fn collect_part_def_specializes(graph: &SemanticGraph) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    // Walk every node in the graph by iterating over uris_with_nodes is too
    // limited (it only returns the first 5). Instead, scan via nodes_named on
    // every distinct simple name we discover from edges and node iteration.
    // To stay simple and complete, fall back to a flat scan via node_index.
    for node in iter_all_nodes(graph) {
        if node.element_kind != "part def" {
            continue;
        }
        let key = simple_name(&node.name);
        if key.is_empty() {
            continue;
        }
        let mut bases: Vec<String> = Vec::new();

        // Original (string) `specializes` reference — works even when the base
        // type cannot be resolved into a concrete graph node.
        if let Some(spec) = node
            .attributes
            .get("specializes")
            .and_then(|v| v.as_str())
        {
            for segment in split_specializes_string(spec) {
                bases.push(segment);
            }
        }

        // Resolved Specializes edges from the graph. These also cover indirect
        // chains where the source `part def` lives in another file.
        for target in graph.outgoing_targets_by_kind(node, RelationshipKind::Specializes) {
            let base = simple_name(&target.name);
            if !base.is_empty() {
                bases.push(base);
            }
        }

        bases.sort();
        bases.dedup();
        map.entry(key).or_default().extend(bases);
    }
    for bases in map.values_mut() {
        bases.sort();
        bases.dedup();
    }
    map
}

/// Build the transitive closure of part-def names that specialize the anchor
/// name (inclusive). Operates purely on the simple-name graph.
fn build_name_closure(
    specializes: &HashMap<String, Vec<String>>,
    anchor: &str,
) -> HashSet<String> {
    let mut closure: HashSet<String> = HashSet::new();
    closure.insert(anchor.to_string());
    let mut changed = true;
    while changed {
        changed = false;
        for (name, bases) in specializes {
            if closure.contains(name) {
                continue;
            }
            if bases.iter().any(|base| closure.contains(base)) {
                closure.insert(name.clone());
                changed = true;
            }
        }
    }
    closure
}

// ---------------------------------------------------------------------------
// Scenario discovery and diagram building
// ---------------------------------------------------------------------------

fn is_scenario_node(node: &SemanticNode, closures: &NameClosures) -> bool {
    match node.element_kind.as_str() {
        "part def" => {
            // Direct match by simple name.
            let simple = simple_name(&node.name);
            if closures.scenario.contains(&simple) {
                return true;
            }
            // Fallback: declared `specializes` string reaches the closure.
            node.attributes
                .get("specializes")
                .and_then(|v| v.as_str())
                .map(|spec| {
                    split_specializes_string(spec)
                        .iter()
                        .any(|base| closures.scenario.contains(base))
                })
                .unwrap_or(false)
        }
        "part" => {
            let part_type = node
                .attributes
                .get("partType")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            !part_type.is_empty() && closures.scenario.contains(&simple_name(part_type))
        }
        _ => false,
    }
}

fn build_diagram(
    graph: &SemanticGraph,
    scenario: &SemanticNode,
    closures: &NameClosures,
    workspace: &HashSet<&Url>,
) -> Option<SequenceDiagramDto> {
    let scenario_id = scenario.id.qualified_name.clone();
    let package_path = package_path_for(graph, scenario);
    let source_kind = match scenario.element_kind.as_str() {
        "part def" => "partDef",
        "part" => "partUsage",
        _ => return None,
    };

    let scenario_uri = scenario.id.uri.as_str().to_string();
    let mut state = ExtractionState {
        graph,
        closures,
        workspace,
        scenario_uri: scenario_uri.clone(),
        scenario_id: scenario_id.clone(),
        messages: Vec::new(),
        activations: Vec::new(),
        next_order: 0,
    };

    let (lifelines, fragments) = state.collect_top_level(scenario);
    let mut messages = std::mem::take(&mut state.messages);
    let mut activations = std::mem::take(&mut state.activations);

    // Re-number messages in source order. The walk is depth-first, so messages
    // already arrive roughly in document order, but a stable resort keeps the
    // contract documented in the plan.
    messages.sort_by(|a, b| compare_range(&a.range, &b.range));
    for (idx, message) in messages.iter_mut().enumerate() {
        message.order = idx + 1;
    }
    activations.sort_by(|a, b| compare_range(&a.range, &b.range));

    let mut diagram = SequenceDiagramDto {
        id: format!("{scenario_id}::sequence"),
        name: scenario.name.clone(),
        package_path,
        source_kind: source_kind.to_string(),
        uri: Some(scenario_uri),
        lifelines,
        messages,
        activations,
        fragments,
        range: range_to_dto(&scenario.range),
    };
    normalize_references(&mut diagram);
    Some(diagram)
}

// ---------------------------------------------------------------------------
// Walking the scenario tree
// ---------------------------------------------------------------------------

struct ExtractionState<'a> {
    graph: &'a SemanticGraph,
    closures: &'a NameClosures,
    #[allow(dead_code)]
    workspace: &'a HashSet<&'a Url>,
    scenario_uri: String,
    #[allow(dead_code)]
    scenario_id: String,
    messages: Vec<SequenceMessageDto>,
    activations: Vec<SequenceActivationDto>,
    next_order: usize,
}

impl<'a> ExtractionState<'a> {
    fn collect_top_level(
        &mut self,
        scenario: &SemanticNode,
    ) -> (Vec<SequenceLifelineDto>, Vec<SequenceFragmentDto>) {
        let mut lifelines = Vec::new();
        let mut fragments = Vec::new();
        let children = sorted_children(self.graph, scenario);
        for child in children {
            if child.element_kind != "part" {
                continue;
            }
            let part_type = part_type_of(child);
            if self.closures.lifeline.contains(&simple_name(&part_type)) {
                lifelines.push(self.build_lifeline(child));
                continue;
            }
            if let Some(kind) = self.closures.message_kind_for(&part_type) {
                let message = self.build_message(child, kind);
                self.messages.push(message);
                continue;
            }
            if self
                .closures
                .activation
                .contains(&simple_name(&part_type))
            {
                let activation = self.build_activation(child);
                self.activations.push(activation);
                continue;
            }
            if let Some(kind) = self.closures.fragment_kind_for(&part_type) {
                fragments.push(self.build_fragment(child, kind));
                continue;
            }
            // Unknown sequence element: ignore, but an upstream caller may
            // have reasons to inspect later. Currently silent like the AST
            // path.
        }
        (lifelines, fragments)
    }

    fn build_lifeline(&self, node: &SemanticNode) -> SequenceLifelineDto {
        SequenceLifelineDto {
            id: node.id.qualified_name.clone(),
            name: node.name.clone(),
            uri: Some(self.scenario_uri.clone()),
            range: range_to_dto(&node.range),
        }
    }

    fn build_message(&mut self, node: &SemanticNode, kind: &'static str) -> SequenceMessageDto {
        self.next_order += 1;
        let from = ref_value(self.graph, node, &["from", "source"]).unwrap_or_default();
        let to = ref_value(self.graph, node, &["to", "target"]).unwrap_or_default();
        let label = attribute_value(self.graph, node, &["label", "message"])
            .or_else(|| (!node.name.trim().is_empty()).then(|| node.name.clone()));
        SequenceMessageDto {
            id: node.id.qualified_name.clone(),
            name: node.name.clone(),
            kind: kind.to_string(),
            from,
            to,
            label,
            order: self.next_order,
            uri: Some(self.scenario_uri.clone()),
            range: range_to_dto(&node.range),
        }
    }

    fn build_activation(&self, node: &SemanticNode) -> SequenceActivationDto {
        let on_lifeline = ref_value(self.graph, node, &["on", "lifeline"]).unwrap_or_default();
        let start_message = ref_value(self.graph, node, &["startMessage", "start"]);
        let finish_message = ref_value(self.graph, node, &["finishMessage", "finish"]);
        SequenceActivationDto {
            id: node.id.qualified_name.clone(),
            name: node.name.clone(),
            on_lifeline,
            start_message,
            finish_message,
            uri: Some(self.scenario_uri.clone()),
            range: range_to_dto(&node.range),
        }
    }

    fn build_fragment(&mut self, node: &SemanticNode, kind: &'static str) -> SequenceFragmentDto {
        let guard = attribute_value(self.graph, node, &["guard", "condition"]);
        let target_ref = if kind == "ref" {
            ref_value(self.graph, node, &["target", "interaction"])
        } else {
            None
        };

        let mut message_ids = Vec::new();
        let mut fragments = Vec::new();
        let mut operands = Vec::new();
        for child in sorted_children(self.graph, node) {
            if child.element_kind != "part" {
                continue;
            }
            let part_type = part_type_of(child);
            if let Some(operand) = self.try_build_operand(child, &part_type) {
                operands.push(operand);
                continue;
            }
            if let Some(child_kind) = self.closures.message_kind_for(&part_type) {
                let message = self.build_message(child, child_kind);
                message_ids.push(message.id.clone());
                self.messages.push(message);
                continue;
            }
            if let Some(child_kind) = self.closures.fragment_kind_for(&part_type) {
                fragments.push(self.build_fragment(child, child_kind));
                continue;
            }
            if self
                .closures
                .activation
                .contains(&simple_name(&part_type))
            {
                let activation = self.build_activation(child);
                self.activations.push(activation);
                continue;
            }
        }

        SequenceFragmentDto {
            id: node.id.qualified_name.clone(),
            name: node.name.clone(),
            kind: kind.to_string(),
            guard,
            message_ids,
            operands,
            fragments,
            target_ref,
            uri: Some(self.scenario_uri.clone()),
            range: range_to_dto(&node.range),
        }
    }

    fn try_build_operand(
        &mut self,
        node: &SemanticNode,
        part_type: &str,
    ) -> Option<SequenceOperandDto> {
        if !self.closures.operand.contains(&simple_name(part_type)) {
            return None;
        }
        let guard = attribute_value(self.graph, node, &["guard", "condition"]);

        let mut message_ids = Vec::new();
        let mut fragments = Vec::new();
        for child in sorted_children(self.graph, node) {
            if child.element_kind != "part" {
                continue;
            }
            let child_type = part_type_of(child);
            if let Some(child_kind) = self.closures.message_kind_for(&child_type) {
                let message = self.build_message(child, child_kind);
                message_ids.push(message.id.clone());
                self.messages.push(message);
                continue;
            }
            if let Some(child_kind) = self.closures.fragment_kind_for(&child_type) {
                fragments.push(self.build_fragment(child, child_kind));
                continue;
            }
            if self
                .closures
                .activation
                .contains(&simple_name(&child_type))
            {
                let activation = self.build_activation(child);
                self.activations.push(activation);
                continue;
            }
        }

        Some(SequenceOperandDto {
            id: node.id.qualified_name.clone(),
            name: node.name.clone(),
            guard,
            message_ids,
            fragments,
            uri: Some(self.scenario_uri.clone()),
            range: range_to_dto(&node.range),
        })
    }
}

// ---------------------------------------------------------------------------
// Reference normalization (resolve from/to/start/finish to local IDs)
// ---------------------------------------------------------------------------

fn normalize_references(diagram: &mut SequenceDiagramDto) {
    let mut local_ids: HashMap<String, String> = HashMap::new();
    for lifeline in &diagram.lifelines {
        local_ids.insert(lifeline.name.clone(), lifeline.id.clone());
        local_ids.insert(simple_name(&lifeline.id), lifeline.id.clone());
    }
    for message in &diagram.messages {
        local_ids.insert(message.name.clone(), message.id.clone());
        local_ids.insert(simple_name(&message.id), message.id.clone());
    }

    for message in &mut diagram.messages {
        message.from = normalize_reference(&message.from, &local_ids);
        message.to = normalize_reference(&message.to, &local_ids);
    }
    for activation in &mut diagram.activations {
        activation.on_lifeline = normalize_reference(&activation.on_lifeline, &local_ids);
        if let Some(start) = activation.start_message.as_mut() {
            *start = normalize_reference(start, &local_ids);
        }
        if let Some(finish) = activation.finish_message.as_mut() {
            *finish = normalize_reference(finish, &local_ids);
        }
    }
    for fragment in &mut diagram.fragments {
        normalize_fragment_references(fragment, &local_ids);
    }
}

fn normalize_fragment_references(
    fragment: &mut SequenceFragmentDto,
    local_ids: &HashMap<String, String>,
) {
    if let Some(target_ref) = fragment.target_ref.as_mut() {
        *target_ref = normalize_reference(target_ref, local_ids);
    }
    for operand in &mut fragment.operands {
        for nested in &mut operand.fragments {
            normalize_fragment_references(nested, local_ids);
        }
    }
    for nested in &mut fragment.fragments {
        normalize_fragment_references(nested, local_ids);
    }
}

fn normalize_reference(value: &str, local_ids: &HashMap<String, String>) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if let Some(resolved) = local_ids.get(trimmed) {
        return resolved.clone();
    }
    let last = simple_name(trimmed);
    if let Some(resolved) = local_ids.get(&last) {
        return resolved.clone();
    }
    last
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn iter_all_nodes(graph: &SemanticGraph) -> impl Iterator<Item = &SemanticNode> {
    graph
        .nodes_by_uri
        .values()
        .flatten()
        .filter_map(|id| graph.get_node(id))
}

fn sorted_children<'a>(graph: &'a SemanticGraph, parent: &SemanticNode) -> Vec<&'a SemanticNode> {
    let mut children = graph.children_of(parent);
    children.sort_by(|a, b| compare_range_lsp(&a.range, &b.range));
    children
}

fn part_type_of(node: &SemanticNode) -> String {
    node.attributes
        .get("partType")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

fn ref_value(graph: &SemanticGraph, node: &SemanticNode, names: &[&str]) -> Option<String> {
    for child in graph.children_of(node) {
        if child.element_kind != "ref" {
            continue;
        }
        if names.iter().any(|n| child.name.eq_ignore_ascii_case(n)) {
            return child
                .attributes
                .get("value")
                .and_then(|v| v.as_str())
                .map(strip_quotes);
        }
    }
    None
}

fn attribute_value(graph: &SemanticGraph, node: &SemanticNode, names: &[&str]) -> Option<String> {
    for child in graph.children_of(node) {
        if child.element_kind != "attribute" {
            continue;
        }
        if names.iter().any(|n| child.name.eq_ignore_ascii_case(n)) {
            return child
                .attributes
                .get("value")
                .and_then(|v| v.as_str())
                .map(strip_quotes);
        }
    }
    None
}

fn strip_quotes(value: &str) -> String {
    value.trim().trim_matches('"').to_string()
}

fn simple_name(value: &str) -> String {
    let normalized = value.replace('.', "::");
    normalized
        .rsplit("::")
        .find(|segment| !segment.trim().is_empty())
        .unwrap_or("")
        .trim()
        .to_string()
}

/// Splits a `specializes` string that may include multiple bases (parser
/// emits comma-separated entries on rare grammars). Returns simple names.
fn split_specializes_string(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(simple_name)
        .filter(|s| !s.is_empty())
        .collect()
}

fn range_to_dto(range: &tower_lsp::lsp_types::Range) -> RangeDto {
    RangeDto {
        start: PositionDto {
            line: range.start.line,
            character: range.start.character,
        },
        end: PositionDto {
            line: range.end.line,
            character: range.end.character,
        },
    }
}

fn compare_range(a: &RangeDto, b: &RangeDto) -> std::cmp::Ordering {
    (a.start.line, a.start.character, a.end.line, a.end.character).cmp(&(
        b.start.line,
        b.start.character,
        b.end.line,
        b.end.character,
    ))
}

fn compare_range_lsp(
    a: &tower_lsp::lsp_types::Range,
    b: &tower_lsp::lsp_types::Range,
) -> std::cmp::Ordering {
    (
        a.start.line,
        a.start.character,
        a.end.line,
        a.end.character,
    )
        .cmp(&(
            b.start.line,
            b.start.character,
            b.end.line,
            b.end.character,
        ))
}

fn package_path_for(graph: &SemanticGraph, scenario: &SemanticNode) -> String {
    let mut segments = Vec::new();
    for ancestor in graph.ancestors_of(scenario) {
        if matches!(ancestor.element_kind.as_str(), "package" | "library package") {
            segments.push(ancestor.name.clone());
        }
    }
    segments.reverse();
    segments
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>()
        .join("::")
}

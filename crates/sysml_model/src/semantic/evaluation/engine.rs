use super::*;

pub(crate) struct EvalEngine<'a> {
    pub(crate) graph: &'a SemanticGraph,
    pub(crate) units: UnitRegistry,
    pub(crate) memoized: HashMap<NodeId, EvalOutcome>,
    pub(crate) active_stack: HashSet<NodeId>,
    pub(crate) parameter_bindings: Vec<HashMap<String, BoundValue>>,
}

impl<'a> EvalEngine<'a> {
    pub(crate) fn new(graph: &'a SemanticGraph, units: UnitRegistry) -> Self {
        Self {
            graph,
            units,
            memoized: HashMap::new(),
            active_stack: HashSet::new(),
            parameter_bindings: Vec::new(),
        }
    }

    pub(crate) fn node_source_value(&self, node_id: &NodeId) -> Option<Value> {
        let node = self.graph.get_node(node_id)?;
        EVALUATION_SOURCE_KEYS
            .iter()
            .find_map(|key| node.attributes.get(*key).cloned())
    }

    pub(crate) fn evaluate_node(&mut self, node_id: &NodeId) -> EvalOutcome {
        if let Some(cached) = self.memoized.get(node_id) {
            return cached.clone();
        }
        if self.active_stack.contains(node_id) {
            return EvalOutcome::error(
                EvalStatus::Cycle,
                format!(
                    "cyclic dependency detected while evaluating '{}'",
                    node_id.qualified_name
                ),
            );
        }
        let Some(raw_value) = self.node_source_value(node_id) else {
            return EvalOutcome::error(
                EvalStatus::Incomplete,
                format!(
                    "no evaluable expression source found for '{}'",
                    node_id.qualified_name
                ),
            );
        };
        self.active_stack.insert(node_id.clone());
        let outcome = self.evaluate_json_value(node_id, &raw_value);
        self.active_stack.remove(node_id);
        self.memoized.insert(node_id.clone(), outcome.clone());
        outcome
    }

    pub(crate) fn evaluate_json_value(&mut self, node_id: &NodeId, value: &Value) -> EvalOutcome {
        match value {
            Value::Bool(v) => EvalOutcome::ok(Value::Bool(*v), None),
            Value::Number(v) => EvalOutcome::ok(Value::Number(v.clone()), None),
            Value::String(s) => self.evaluate_expression_text(node_id, s),
            Value::Null => EvalOutcome::error(EvalStatus::Unknown, "no expression value"),
            _ => EvalOutcome::error(
                EvalStatus::Unsupported,
                "expression value type is not supported",
            ),
        }
    }

    pub(crate) fn evaluate_expression_text(&mut self, node_id: &NodeId, raw: &str) -> EvalOutcome {
        let normalized = normalize_unit_brackets(raw.trim());
        let text = normalized.as_str();
        if text.is_empty() {
            return EvalOutcome::error(EvalStatus::Unknown, "empty expression");
        }
        if text.eq_ignore_ascii_case("true") {
            return EvalOutcome::ok(Value::Bool(true), None);
        }
        if text.eq_ignore_ascii_case("false") {
            return EvalOutcome::ok(Value::Bool(false), None);
        }
        if let Ok(parsed_string) = serde_json::from_str::<String>(text) {
            return EvalOutcome::ok(Value::String(parsed_string), None);
        }
        if let Some(identifier) = parse_standalone_identifier(text) {
            return self.resolve_identifier_value(node_id, identifier);
        }

        match self.evaluate_quantity_expression(node_id, text) {
            Ok(quantity) => EvalOutcome::from_quantity(quantity),
            Err(EvalStatus::DivByZero) => {
                EvalOutcome::error(EvalStatus::DivByZero, "division by zero")
            }
            Err(EvalStatus::Cycle) => {
                EvalOutcome::error(EvalStatus::Cycle, "cyclic reference detected")
            }
            Err(EvalStatus::TypeError) => EvalOutcome::error(
                EvalStatus::TypeError,
                "expression has type or unit mismatch for arithmetic",
            ),
            Err(EvalStatus::Unknown) => {
                EvalOutcome::error(EvalStatus::Unknown, "expression could not be resolved")
            }
            Err(EvalStatus::Incomplete) => EvalOutcome::error(
                EvalStatus::Incomplete,
                "expression depends on unevaluated value",
            ),
            Err(EvalStatus::Unsupported) | Err(EvalStatus::Ok) => {
                EvalOutcome::error(EvalStatus::Unsupported, "expression form is not supported")
            }
        }
    }

    pub(crate) fn evaluate_quantity_expression(
        &mut self,
        node_id: &NodeId,
        expression: &str,
    ) -> Result<Quantity, EvalStatus> {
        let units = self.units.clone();
        let mut parser = QuantityParser::new(expression, &units, |name, args| {
            if let Some(arg_list) = args {
                self.evaluate_invocation_quantity(node_id, name, arg_list)
            } else {
                self.resolve_identifier_quantity(node_id, name)
            }
        });
        let quantity = parser.parse_expression()?;
        parser.skip_ws();
        if !parser.is_eof() {
            return Err(EvalStatus::Unsupported);
        }
        Ok(quantity)
    }

    pub(crate) fn resolve_identifier_value(
        &mut self,
        node_id: &NodeId,
        identifier: &str,
    ) -> EvalOutcome {
        let referenced_id = match self.resolve_identifier_node(node_id, identifier) {
            Ok(found) => found,
            Err(outcome) => return outcome,
        };
        self.evaluate_node(&referenced_id)
    }

    pub(crate) fn resolve_identifier_quantity(
        &mut self,
        node_id: &NodeId,
        identifier: &str,
    ) -> Result<Quantity, EvalStatus> {
        if let Some(bound) = self.lookup_bound_value(identifier) {
            return match bound {
                BoundValue::Quantity(q) => Ok(q),
                BoundValue::Collection(_) => Err(EvalStatus::TypeError),
            };
        }
        let referenced_id = self
            .resolve_identifier_node(node_id, identifier)
            .map_err(|outcome| outcome.status)?;
        let outcome = self.evaluate_node(&referenced_id);
        if outcome.status != EvalStatus::Ok {
            return Err(outcome.status);
        }
        let Some(value) = outcome.value else {
            return Err(EvalStatus::Unknown);
        };
        let Some(number) = json_value_to_f64(&value) else {
            return Err(EvalStatus::TypeError);
        };
        Ok(Quantity {
            value: number,
            unit: outcome.unit,
        })
    }

    pub(crate) fn lookup_bound_value(&self, identifier: &str) -> Option<BoundValue> {
        self.parameter_bindings.iter().rev().find_map(|scope| {
            scope.get(identifier).cloned().or_else(|| {
                identifier
                    .rsplit("::")
                    .next()
                    .and_then(|tail| scope.get(tail).cloned())
            })
        })
    }

    pub(crate) fn resolve_identifier_node(
        &self,
        current_id: &NodeId,
        identifier: &str,
    ) -> Result<NodeId, EvalOutcome> {
        if let Some((head, rest)) = identifier.split_once('.') {
            let mut current = self.resolve_identifier_node(current_id, head)?;
            for seg in rest.split('.') {
                let qualified = format!("{}::{seg}", current.qualified_name);
                let candidates = self.lookup_qualified_candidates(&qualified);
                if !candidates.is_empty() {
                    current = choose_candidate(self.graph, candidates, seg)?;
                    continue;
                }
                let Some(owner) = self.graph.get_node(&current) else {
                    return Err(EvalOutcome::error(
                        EvalStatus::Unknown,
                        format!("unresolved reference '{identifier}'"),
                    ));
                };
                match resolve_member_via_type(self.graph, owner, seg) {
                    ResolveResult::Resolved(member_id) => current = member_id,
                    ResolveResult::Ambiguous => {
                        return Err(EvalOutcome::error(
                            EvalStatus::Unknown,
                            format!("ambiguous reference '{seg}'"),
                        ));
                    }
                    ResolveResult::Unresolved => {
                        return Err(EvalOutcome::error(
                            EvalStatus::Unknown,
                            format!("unresolved reference '{identifier}'"),
                        ));
                    }
                }
            }
            return Ok(current);
        }
        let Some(current) = self.graph.get_node(current_id) else {
            return Err(EvalOutcome::error(
                EvalStatus::Unknown,
                format!("unknown evaluation node '{}'", current_id.qualified_name),
            ));
        };
        let scoped_candidates = self.scoped_candidates(current, identifier);
        if !scoped_candidates.is_empty() {
            return choose_candidate(self.graph, scoped_candidates, identifier);
        }
        let fallback_candidates = self.fallback_candidates(current, identifier);
        if !fallback_candidates.is_empty() {
            return choose_candidate(self.graph, fallback_candidates, identifier);
        }
        Err(EvalOutcome::error(
            EvalStatus::Unknown,
            format!("unresolved reference '{identifier}'"),
        ))
    }

    pub(crate) fn scoped_candidates(
        &self,
        current: &SemanticNode,
        identifier: &str,
    ) -> Vec<NodeId> {
        let mut candidates = Vec::new();
        let mut prefixes = scope_prefixes(self.graph, current);
        prefixes.insert(0, current.id.qualified_name.clone());
        prefixes.extend(typed_case_definition_scope_prefixes(self.graph, current));
        prefixes.extend(typed_requirement_definition_scope_prefixes(
            self.graph, current,
        ));
        for scope_prefix in prefixes {
            let qualified = format!("{scope_prefix}::{identifier}");
            candidates.extend(self.lookup_qualified_candidates(&qualified));
        }
        dedupe_node_ids(candidates)
    }

    pub(crate) fn fallback_candidates(
        &self,
        current: &SemanticNode,
        identifier: &str,
    ) -> Vec<NodeId> {
        let mut candidates = Vec::new();
        if identifier.contains("::") {
            candidates.extend(self.lookup_qualified_candidates(identifier));
        } else {
            let same_uri_named = self
                .graph
                .nodes_for_uri(&current.id.uri)
                .into_iter()
                .filter(|node| node.name == identifier)
                .map(|node| node.id.clone())
                .collect::<Vec<_>>();
            candidates.extend(same_uri_named);
            candidates.extend(self.lookup_qualified_candidates(identifier));
        }
        dedupe_node_ids(candidates)
    }

    pub(crate) fn lookup_qualified_candidates(&self, qualified_name: &str) -> Vec<NodeId> {
        self.graph
            .node_ids_by_qualified_name
            .get(qualified_name)
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }

    pub(crate) fn evaluate_invocation_quantity(
        &mut self,
        context_id: &NodeId,
        callable_name: &str,
        args: &[&str],
    ) -> Result<Quantity, EvalStatus> {
        let normalized_args = normalize_invocation_args(args);
        if callable_name == "sum" {
            return self.evaluate_builtin_sum(context_id, &normalized_args);
        }
        if callable_name == "count" {
            return self.evaluate_builtin_count(&normalized_args);
        }
        if callable_name == "min" {
            return self.evaluate_builtin_min_max(context_id, &normalized_args, true);
        }
        if callable_name == "max" {
            return self.evaluate_builtin_min_max(context_id, &normalized_args, false);
        }
        if callable_name == "avg" {
            return self.evaluate_builtin_avg(context_id, &normalized_args);
        }
        let callable_id = self
            .resolve_callable_node(context_id, callable_name)
            .ok_or(EvalStatus::Unknown)?;
        let callable = self
            .graph
            .get_node(&callable_id)
            .ok_or(EvalStatus::Unknown)?;
        if callable.element_kind != ElementKind::CalcDef {
            return Err(EvalStatus::TypeError);
        }
        let expression = callable
            .attributes
            .get(ANALYSIS_EXPRESSION_KEY)
            .and_then(Value::as_str)
            .ok_or(EvalStatus::Unknown)?
            .to_string();
        let mut param_names = in_parameter_names(callable);
        if param_names.is_empty() && !normalized_args.is_empty() {
            let inferred = infer_parameter_names_from_expression(&expression);
            if inferred.len() == normalized_args.len() {
                param_names = inferred;
            }
        }
        let bindings =
            self.bind_invocation_parameters(context_id, callable, &param_names, &normalized_args)?;
        self.parameter_bindings.push(bindings);
        let result = self.evaluate_quantity_expression(&callable_id, &expression);
        self.parameter_bindings.pop();
        result
    }

    pub(crate) fn evaluate_builtin_count(
        &self,
        normalized_args: &[&str],
    ) -> Result<Quantity, EvalStatus> {
        if normalized_args.is_empty() {
            return Err(EvalStatus::Unsupported);
        }
        Ok(Quantity::scalar(normalized_args.len() as f64))
    }

    pub(crate) fn evaluate_builtin_avg(
        &mut self,
        context_id: &NodeId,
        normalized_args: &[&str],
    ) -> Result<Quantity, EvalStatus> {
        if normalized_args.is_empty() {
            return Err(EvalStatus::Unsupported);
        }
        let sum = self.evaluate_builtin_sum(context_id, normalized_args)?;
        let denom = normalized_args.len() as f64;
        Ok(Quantity {
            value: sum.value / denom,
            unit: sum.unit,
        })
    }

    pub(crate) fn evaluate_builtin_min_max(
        &mut self,
        context_id: &NodeId,
        normalized_args: &[&str],
        is_min: bool,
    ) -> Result<Quantity, EvalStatus> {
        if normalized_args.is_empty() {
            return Err(EvalStatus::Unsupported);
        }
        let mut it = normalized_args.iter();
        let first_expr = it.next().expect("non-empty args");
        let mut best = self.evaluate_quantity_expression(context_id, first_expr)?;
        for expr in it {
            let candidate = self.evaluate_quantity_expression(context_id, expr)?;
            let candidate_value = match (&best.unit, &candidate.unit) {
                (None, None) => candidate.value,
                (Some(best_unit), Some(candidate_unit)) => {
                    let converted =
                        self.units
                            .convert_value(candidate.value, candidate_unit, best_unit);
                    converted.map_err(map_unit_error)?
                }
                (Some(unit), None) | (None, Some(unit)) => {
                    if !self.units.has_symbol(unit) {
                        return Err(EvalStatus::Unknown);
                    }
                    return Err(EvalStatus::TypeError);
                }
            };
            let take = if is_min {
                candidate_value < best.value
            } else {
                candidate_value > best.value
            };
            if take {
                best.value = candidate_value;
            }
        }
        Ok(best)
    }

    pub(crate) fn collect_member_paths_for_sum_projection(
        &self,
        context_id: &NodeId,
        head: &str,
        rest: &str,
    ) -> Vec<String> {
        let Some(context) = self.graph.get_node(context_id) else {
            return Vec::new();
        };
        let part_child_names: Vec<String> = self
            .graph
            .children_of(context)
            .into_iter()
            .filter(|child| child.element_kind == ElementKind::Part)
            .map(|child| child.name.clone())
            .collect();
        let named_matches: Vec<_> = part_child_names
            .iter()
            .filter(|name| name.as_str() == head)
            .collect();
        if named_matches.len() == 1 {
            return vec![format!("{head}.{rest}")];
        }
        if named_matches.len() > 1 {
            return named_matches
                .iter()
                .map(|name| format!("{name}.{rest}"))
                .collect();
        }
        if part_child_names.len() > 1 {
            return part_child_names
                .iter()
                .map(|name| format!("{name}.{rest}"))
                .collect();
        }
        Vec::new()
    }

    pub(crate) fn evaluate_builtin_sum(
        &mut self,
        context_id: &NodeId,
        normalized_args: &[&str],
    ) -> Result<Quantity, EvalStatus> {
        if normalized_args.is_empty() {
            return Err(EvalStatus::Unsupported);
        }
        if normalized_args.len() == 1 {
            let needle = normalized_args[0].trim();
            if let Some((head, rest)) = needle.split_once('.') {
                if let Some(BoundValue::Collection(items)) = self.lookup_bound_value(head) {
                    let mut acc: Option<Quantity> = None;
                    for item in items {
                        let projected = format!("{item}.{rest}");
                        let q = self.resolve_identifier_quantity(context_id, &projected)?;
                        acc = Some(match acc {
                            None => q,
                            Some(prev) => add_quantities_with_units(&self.units, prev, q)?,
                        });
                    }
                    return acc.ok_or(EvalStatus::Unsupported);
                }
                let member_paths =
                    self.collect_member_paths_for_sum_projection(context_id, head, rest);
                if !member_paths.is_empty() {
                    let mut acc: Option<Quantity> = None;
                    for path in member_paths {
                        let q = self.resolve_identifier_quantity(context_id, &path)?;
                        acc = Some(match acc {
                            None => q,
                            Some(prev) => add_quantities_with_units(&self.units, prev, q)?,
                        });
                    }
                    return acc.ok_or(EvalStatus::Unsupported);
                }
            }
        }
        let mut it = normalized_args.iter();
        let first = it.next().expect("non-empty args").to_string();
        let mut acc = self.evaluate_quantity_expression(context_id, &first)?;
        for arg in it {
            let evaluated = self.evaluate_quantity_expression(context_id, arg)?;
            acc = add_quantities_with_units(&self.units, acc, evaluated)?;
        }
        Ok(acc)
    }

    pub(crate) fn evaluate_invocation_bool(
        &mut self,
        context_id: &NodeId,
        callable_name: &str,
        args: &[&str],
    ) -> Result<bool, AnalysisEvalError> {
        let normalized_args = normalize_invocation_args(args);
        let callable_id = self
            .resolve_callable_node(context_id, callable_name)
            .ok_or_else(|| {
                AnalysisEvalError::with_message(
                    EvalStatus::Unknown,
                    format!("unresolved callable '{callable_name}'"),
                )
            })?;
        let callable = self.graph.get_node(&callable_id).ok_or_else(|| {
            AnalysisEvalError::with_message(
                EvalStatus::Unknown,
                format!("unresolved callable '{callable_name}'"),
            )
        })?;
        let expression = callable
            .attributes
            .get(ANALYSIS_EXPRESSION_KEY)
            .and_then(Value::as_str)
            .ok_or_else(|| {
                AnalysisEvalError::with_message(
                    EvalStatus::Incomplete,
                    format!("callable '{callable_name}' has no analysis expression"),
                )
            })?
            .to_string();
        let mut param_names = in_parameter_names(callable);
        if param_names.is_empty() && !normalized_args.is_empty() {
            let inferred = infer_parameter_names_from_expression(&expression);
            if inferred.len() == normalized_args.len() {
                param_names = inferred;
            }
        }
        let bindings = self
            .bind_invocation_parameters(context_id, callable, &param_names, &normalized_args)
            .map_err(AnalysisEvalError::from_status)?;
        self.parameter_bindings.push(bindings);
        let result = evaluate_analysis_expression(self, &callable_id, &expression);
        self.parameter_bindings.pop();
        result
    }

    pub(crate) fn bind_invocation_parameters(
        &mut self,
        context_id: &NodeId,
        callable: &SemanticNode,
        param_names: &[String],
        normalized_args: &[&str],
    ) -> Result<HashMap<String, BoundValue>, EvalStatus> {
        let collection_params = callable_collection_params(callable);
        let parsed = parse_invocation_args(normalized_args)?;
        match parsed {
            InvocationArgs::Positional(args) => {
                if param_names.len() != args.len() {
                    return Err(EvalStatus::Unsupported);
                }
                let mut bindings = HashMap::new();
                for (name, arg_expr) in param_names.iter().zip(args.iter()) {
                    let bound = if collection_params.contains(name) {
                        BoundValue::Collection(parse_tuple_identifier_list(arg_expr)?)
                    } else {
                        BoundValue::Quantity(
                            self.evaluate_quantity_expression(context_id, arg_expr)?,
                        )
                    };
                    bindings.insert(name.clone(), bound);
                }
                Ok(bindings)
            }
            InvocationArgs::Named(named) => {
                if param_names.is_empty() {
                    return Err(EvalStatus::Unsupported);
                }
                // Reject unknown names to avoid silent typos.
                if named.len() != param_names.len() {
                    return Err(EvalStatus::Unsupported);
                }
                let mut bindings = HashMap::new();
                for param in param_names {
                    let Some(expr) = named.get(param) else {
                        return Err(EvalStatus::Unsupported);
                    };
                    let bound = if collection_params.contains(param) {
                        BoundValue::Collection(parse_tuple_identifier_list(expr)?)
                    } else {
                        BoundValue::Quantity(self.evaluate_quantity_expression(context_id, expr)?)
                    };
                    bindings.insert(param.clone(), bound);
                }
                Ok(bindings)
            }
        }
    }

    pub(crate) fn resolve_callable_node(
        &self,
        context_id: &NodeId,
        callable_name: &str,
    ) -> Option<NodeId> {
        let current = self.graph.get_node(context_id)?;
        let mut candidates = Vec::new();
        for scope_prefix in scope_prefixes(self.graph, current) {
            let qualified = format!("{scope_prefix}::{callable_name}");
            candidates.extend(self.lookup_callable_candidates(&qualified));
        }
        if callable_name.contains("::") {
            candidates.extend(self.lookup_callable_candidates(callable_name));
        } else {
            candidates.extend(
                self.graph
                    .nodes_for_uri(&current.id.uri)
                    .into_iter()
                    .filter(|node| {
                        node.name == callable_name
                            && matches!(
                                node.element_kind,
                                ElementKind::CalcDef | ElementKind::ConstraintDef
                            )
                    })
                    .map(|node| node.id.clone()),
            );
            candidates.extend(self.lookup_callable_candidates(callable_name));
        }
        dedupe_node_ids(candidates).into_iter().next()
    }

    pub(crate) fn lookup_callable_candidates(&self, qualified_name: &str) -> Vec<NodeId> {
        self.graph
            .node_ids_by_qualified_name
            .get(qualified_name)
            .into_iter()
            .flatten()
            .filter_map(|node_id| {
                let node = self.graph.get_node(node_id)?;
                matches!(
                    node.element_kind,
                    ElementKind::CalcDef | ElementKind::ConstraintDef
                )
                .then_some(node_id.clone())
            })
            .collect()
    }
}

pub(crate) fn scope_prefixes(graph: &SemanticGraph, current: &SemanticNode) -> Vec<String> {
    let mut prefixes = Vec::new();
    if let Some(parent) = graph.parent_of(current) {
        prefixes.push(parent.id.qualified_name.clone());
    }
    for ancestor in graph.ancestors_of(current) {
        prefixes.push(ancestor.id.qualified_name.clone());
    }
    prefixes
}

pub(crate) fn choose_candidate(
    graph: &SemanticGraph,
    candidates: Vec<NodeId>,
    identifier: &str,
) -> Result<NodeId, EvalOutcome> {
    if candidates.len() == 1 {
        return Ok(candidates[0].clone());
    }
    let mut sorted = candidates;
    sorted.sort_by(|left, right| {
        candidate_preference_score(graph, left)
            .cmp(&candidate_preference_score(graph, right))
            .reverse()
            .then_with(|| left.qualified_name.len().cmp(&right.qualified_name.len()))
    });
    let best = sorted[0].clone();
    let best_score = candidate_preference_score(graph, &best);
    let best_len = best.qualified_name.len();
    let ambiguous = sorted.iter().skip(1).any(|candidate| {
        let score = candidate_preference_score(graph, candidate);
        if score < best_score {
            return false;
        }
        score == best_score && candidate.qualified_name.len() == best_len
    });
    if !ambiguous {
        return Ok(best);
    }
    Err(EvalOutcome::error(
        EvalStatus::Unknown,
        format!("ambiguous reference '{identifier}'"),
    ))
}

pub(crate) fn candidate_preference_score(graph: &SemanticGraph, candidate: &NodeId) -> u8 {
    let Some(node) = graph.get_node(candidate) else {
        return 0;
    };
    let has_evaluable_source = EVALUATION_SOURCE_KEYS.iter().any(|key| {
        node.attributes
            .get(*key)
            .is_some_and(value_has_evaluable_content)
    });
    if has_evaluable_source {
        2
    } else {
        0
    }
}

pub(crate) fn value_has_evaluable_content(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(text) => !text.trim().is_empty(),
        _ => true,
    }
}

pub(crate) fn dedupe_node_ids(ids: Vec<NodeId>) -> Vec<NodeId> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for id in ids {
        if seen.insert(id.clone()) {
            out.push(id);
        }
    }
    out
}

pub(crate) fn in_parameter_names(node: &SemanticNode) -> Vec<String> {
    node.attributes
        .get("parameters")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let direction = entry.get("direction").and_then(Value::as_str)?;
            let name = entry.get("name").and_then(Value::as_str)?;
            matches!(direction, "in" | "inout").then_some(name.to_string())
        })
        .collect()
}

pub(crate) fn infer_parameter_names_from_expression(expression: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let ch = chars[i];
        if ch.is_ascii_alphabetic() || ch == '_' {
            let start = i;
            i += 1;
            while i < chars.len() {
                let c = chars[i];
                if c.is_ascii_alphanumeric() || c == '_' {
                    i += 1;
                    continue;
                }
                if i + 1 < chars.len() && c == ':' && chars[i + 1] == ':' {
                    i += 2;
                    continue;
                }
                break;
            }
            let token: String = chars[start..i].iter().collect();
            if token.eq_ignore_ascii_case("true") || token.eq_ignore_ascii_case("false") {
                continue;
            }
            if seen.insert(token.clone()) {
                names.push(token);
            }
            continue;
        }
        i += 1;
    }
    names
}

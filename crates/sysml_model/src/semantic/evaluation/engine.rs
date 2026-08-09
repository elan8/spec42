use super::*;
use crate::semantic::model::{DeclaredExpression, DeclaredExpressionKind, RelationshipKind};

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

    /// Evaluation accepts only normalized parser facts. Presentation projections are not semantic
    /// input and cannot affect an evaluation result.
    pub(crate) fn node_expression(&self, node_id: &NodeId) -> Option<DeclaredExpression> {
        let node = self.graph.get_node(node_id)?;
        node.declared_facts
            .feature_value
            .as_ref()
            .map(|value| value.expression.clone())
            .or_else(|| node.declared_facts.own_expression.clone())
            .or_else(|| {
                self.graph
                    .children_of(node)
                    .into_iter()
                    .find(|child| {
                        matches!(
                            child.element_kind,
                            ElementKind::AnalysisResult
                                | ElementKind::Verdict
                                | ElementKind::AssertConstraint
                                | ElementKind::RequireConstraint
                        )
                    })
                    .and_then(child_expression)
            })
            .or_else(|| self.typed_case_expression(node))
            .or_else(|| self.typed_requirement_constraint_expression(node))
    }

    fn typed_case_expression(&self, node: &SemanticNode) -> Option<DeclaredExpression> {
        let definition_id = typed_case_definition_id(self.graph, node)?;
        let definition = self.graph.get_node(&definition_id)?;
        self.graph
            .children_of(definition)
            .into_iter()
            .find(|child| {
                matches!(
                    child.element_kind,
                    ElementKind::AnalysisResult | ElementKind::Verdict
                )
            })
            .and_then(child_expression)
    }

    fn typed_requirement_constraint_expression(
        &self,
        node: &SemanticNode,
    ) -> Option<DeclaredExpression> {
        let definition_id = typed_requirement_definition_id(self.graph, node)?;
        let definition = self.graph.get_node(&definition_id)?;
        self.graph
            .children_of(definition)
            .into_iter()
            .find(|child| child.element_kind == ElementKind::RequireConstraint)
            .and_then(child_expression)
    }

    pub(crate) fn evaluate_node(&mut self, node_id: &NodeId) -> EvalOutcome {
        if let Some(outcome) = self.memoized.get(node_id) {
            return outcome.clone();
        }
        if !self.active_stack.insert(node_id.clone()) {
            return EvalOutcome::error(
                EvalStatus::Cycle,
                format!(
                    "cyclic dependency detected while evaluating '{}'",
                    node_id.qualified_name
                ),
            );
        }
        let outcome = self.node_expression(node_id).map_or_else(
            || EvalOutcome::error(EvalStatus::Incomplete, "no declared expression"),
            |expression| self.evaluate_declared_expression(node_id, &expression),
        );
        self.active_stack.remove(node_id);
        self.memoized.insert(node_id.clone(), outcome.clone());
        outcome
    }

    pub(crate) fn evaluate_declared_expression(
        &mut self,
        node_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> EvalOutcome {
        self.evaluate_value(node_id, expression)
            .unwrap_or_else(|status| EvalOutcome::error(status, status_message(status)))
    }

    fn evaluate_value(
        &mut self,
        node_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        match expression.kind {
            DeclaredExpressionKind::IntegerLiteral => expression
                .literal
                .clone()
                .filter(|value| value.is_number())
                .map(|value| EvalOutcome::ok(value, None))
                .ok_or(EvalStatus::Malformed),
            DeclaredExpressionKind::RealLiteral => expression
                .literal
                .as_ref()
                .and_then(real_literal)
                .map(Quantity::scalar)
                .map(EvalOutcome::from_quantity)
                .ok_or(EvalStatus::Malformed),
            DeclaredExpressionKind::StringLiteral | DeclaredExpressionKind::BooleanLiteral => {
                expression
                    .literal
                    .clone()
                    .filter(|value| value.is_string() || value.is_boolean())
                    .map(|value| EvalOutcome::ok(value, None))
                    .ok_or(EvalStatus::Malformed)
            }
            DeclaredExpressionKind::Null => Err(EvalStatus::Incomplete),
            DeclaredExpressionKind::FeatureReference | DeclaredExpressionKind::FeatureChain => {
                let reference = expression
                    .reference
                    .as_deref()
                    .ok_or(EvalStatus::Malformed)?;
                if let Some(value) = self.bound_value(reference) {
                    return Ok(EvalOutcome::from_quantity(value));
                }
                self.resolve_identifier(node_id, reference)
                    .map(|id| self.evaluate_node(&id))
                    .and_then(outcome_result)
            }
            DeclaredExpressionKind::Parenthesized
            | DeclaredExpressionKind::Bracket
            | DeclaredExpressionKind::MetadataAccess => {
                self.evaluate_single_child(node_id, expression)
            }
            DeclaredExpressionKind::LiteralWithUnit => {
                self.evaluate_literal_with_unit(node_id, expression)
            }
            DeclaredExpressionKind::Unary => self.evaluate_unary(node_id, expression),
            DeclaredExpressionKind::Binary => self.evaluate_binary(node_id, expression),
            DeclaredExpressionKind::Invocation => self.evaluate_invocation(node_id, expression),
            DeclaredExpressionKind::MemberAccess => {
                self.evaluate_member_access(node_id, expression)
            }
            _ => Err(EvalStatus::Unsupported),
        }
    }

    fn evaluate_single_child(
        &mut self,
        node_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        let [child] = expression.children.as_slice() else {
            return Err(EvalStatus::Malformed);
        };
        self.evaluate_value(node_id, child)
    }

    fn evaluate_literal_with_unit(
        &mut self,
        node_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        let [value, unit] = expression.children.as_slice() else {
            return Err(EvalStatus::Malformed);
        };
        let value = outcome_quantity(self.evaluate_value(node_id, value)?)?;
        let unit = unit_reference(unit).ok_or(EvalStatus::Malformed)?;
        if !self.units.has_symbol(&unit) {
            return Err(EvalStatus::Unresolved);
        }
        Ok(EvalOutcome::from_quantity(Quantity {
            value: value.value,
            unit: Some(unit),
        }))
    }

    fn evaluate_unary(
        &mut self,
        node_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        let value = self.evaluate_single_child(node_id, expression)?;
        match expression.operator.as_deref() {
            Some("+") => Ok(value),
            Some("-") => {
                let value = outcome_quantity(value)?;
                Ok(EvalOutcome::from_quantity(Quantity {
                    value: -value.value,
                    unit: value.unit,
                }))
            }
            Some("!") | Some("not") => value
                .value
                .and_then(|value| value.as_bool())
                .map(|value| EvalOutcome::ok(Value::Bool(!value), None))
                .ok_or(EvalStatus::TypeError),
            _ => Err(EvalStatus::Unsupported),
        }
    }

    fn evaluate_binary(
        &mut self,
        node_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        let [left, right] = expression.children.as_slice() else {
            return Err(EvalStatus::Malformed);
        };
        let operator = expression
            .operator
            .as_deref()
            .ok_or(EvalStatus::Malformed)?;
        if matches!(operator, "and" | "or" | "&&" | "||") {
            let left = bool_outcome(self.evaluate_value(node_id, left)?)?;
            if matches!(operator, "and" | "&&") && !left {
                return Ok(EvalOutcome::ok(Value::Bool(false), None));
            }
            if matches!(operator, "or" | "||") && left {
                return Ok(EvalOutcome::ok(Value::Bool(true), None));
            }
            return Ok(EvalOutcome::ok(
                Value::Bool(bool_outcome(self.evaluate_value(node_id, right)?)?),
                None,
            ));
        }
        let left = self.evaluate_value(node_id, left)?;
        let right = self.evaluate_value(node_id, right)?;
        match operator {
            "+" | "-" | "*" | "/" => self.arithmetic(operator, left, right),
            "<" | "<=" | ">" | ">=" | "==" | "!=" => self.comparison(operator, left, right),
            _ => Err(EvalStatus::Unsupported),
        }
    }

    fn arithmetic(
        &self,
        operator: &str,
        left: EvalOutcome,
        right: EvalOutcome,
    ) -> Result<EvalOutcome, EvalStatus> {
        let left = outcome_quantity(left)?;
        let right = outcome_quantity(right)?;
        let value = match operator {
            "+" => add_quantities(&self.units, left, right)?,
            "-" => add_quantities(
                &self.units,
                left,
                Quantity {
                    value: -right.value,
                    unit: right.unit,
                },
            )?,
            "*" | "/" => {
                if operator == "/" && right.value == 0.0 {
                    return Err(EvalStatus::DivByZero);
                }
                let (value, unit) = self
                    .units
                    .compose_product(
                        left.value,
                        left.unit.as_deref(),
                        right.value,
                        right.unit.as_deref(),
                        operator == "/",
                    )
                    .map_err(unit_error)?;
                Quantity { value, unit }
            }
            _ => return Err(EvalStatus::Unsupported),
        };
        Ok(EvalOutcome::from_quantity(value))
    }

    fn comparison(
        &self,
        operator: &str,
        left: EvalOutcome,
        right: EvalOutcome,
    ) -> Result<EvalOutcome, EvalStatus> {
        if matches!(operator, "==" | "!=") && left.unit.is_none() && right.unit.is_none() {
            let equal = left.value == right.value;
            return Ok(EvalOutcome::ok(
                Value::Bool(if operator == "==" { equal } else { !equal }),
                None,
            ));
        }
        let left = outcome_quantity(left)?;
        let right = outcome_quantity(right)?;
        let right = match (&left.unit, &right.unit) {
            (None, None) => right.value,
            (Some(left_unit), Some(right_unit)) => self
                .units
                .convert_value(right.value, right_unit, left_unit)
                .map_err(unit_error)?,
            _ => return Err(EvalStatus::TypeError),
        };
        let value = match operator {
            "<" => left.value < right,
            "<=" => left.value <= right,
            ">" => left.value > right,
            ">=" => left.value >= right,
            "==" => (left.value - right).abs() < 1e-9,
            "!=" => (left.value - right).abs() >= 1e-9,
            _ => return Err(EvalStatus::Unsupported),
        };
        Ok(EvalOutcome::ok(Value::Bool(value), None))
    }

    fn evaluate_invocation(
        &mut self,
        context_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        let callable = expression
            .children
            .first()
            .and_then(|child| child.reference.as_deref())
            .ok_or(EvalStatus::Malformed)?;
        let args = expression
            .arguments
            .iter()
            .map(|argument| self.evaluate_value(context_id, &argument.value))
            .collect::<Result<Vec<_>, _>>()?;
        match callable {
            "count" if !args.is_empty() => Ok(EvalOutcome::from_quantity(Quantity::scalar(
                args.len() as f64,
            ))),
            "sum" | "min" | "max" | "avg" => self.builtin(callable, args),
            _ => self.invoke_callable(context_id, callable, expression),
        }
    }

    fn builtin(&self, name: &str, args: Vec<EvalOutcome>) -> Result<EvalOutcome, EvalStatus> {
        let mut values = args.into_iter().map(outcome_quantity);
        let Some(mut result) = values.next().transpose()? else {
            return Err(EvalStatus::Malformed);
        };
        let mut count = 1usize;
        for value in values {
            let value = value?;
            count += 1;
            match name {
                "sum" | "avg" => result = add_quantities(&self.units, result, value)?,
                "min" | "max" => {
                    let converted = match (&result.unit, &value.unit) {
                        (None, None) => value.value,
                        (Some(result_unit), Some(value_unit)) => self
                            .units
                            .convert_value(value.value, value_unit, result_unit)
                            .map_err(unit_error)?,
                        _ => return Err(EvalStatus::TypeError),
                    };
                    if (name == "min" && converted < result.value)
                        || (name == "max" && converted > result.value)
                    {
                        result.value = converted;
                    }
                }
                _ => return Err(EvalStatus::Unsupported),
            }
        }
        if name == "avg" {
            result.value /= count as f64;
        }
        Ok(EvalOutcome::from_quantity(result))
    }

    fn invoke_callable(
        &mut self,
        context_id: &NodeId,
        callable: &str,
        invocation: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        let callable_id = self.resolve_identifier(context_id, callable)?;
        let (body, parameters) = {
            let node = self
                .graph
                .get_node(&callable_id)
                .ok_or(EvalStatus::Unresolved)?;
            if !matches!(
                node.element_kind,
                ElementKind::CalcDef | ElementKind::ConstraintDef
            ) {
                return Err(EvalStatus::TypeError);
            }
            let body = node
                .declared_facts
                .own_expression
                .clone()
                .ok_or(EvalStatus::Incomplete)?;
            let parameters = self
                .graph
                .children_of(node)
                .into_iter()
                .filter(|parameter| {
                    matches!(
                        parameter
                            .declared_facts
                            .feature_properties
                            .as_ref()
                            .and_then(|facts| facts.direction.as_deref()),
                        Some("in") | Some("inout")
                    )
                })
                .map(|parameter| parameter.name.clone())
                .collect::<Vec<_>>();
            (body, parameters)
        };
        if invocation.arguments.len() != parameters.len() {
            return Err(EvalStatus::Malformed);
        }
        let named = invocation
            .arguments
            .iter()
            .any(|argument| argument.name.is_some());
        if named
            && invocation
                .arguments
                .iter()
                .any(|argument| argument.name.is_none())
        {
            return Err(EvalStatus::Malformed);
        }
        let mut bindings = HashMap::new();
        for (index, parameter) in parameters.iter().enumerate() {
            let argument = if named {
                invocation
                    .arguments
                    .iter()
                    .find(|argument| argument.name.as_deref() == Some(parameter))
                    .ok_or(EvalStatus::Malformed)?
            } else {
                &invocation.arguments[index]
            };
            bindings.insert(
                parameter.clone(),
                BoundValue(outcome_quantity(
                    self.evaluate_value(context_id, &argument.value)?,
                )?),
            );
        }
        self.parameter_bindings.push(bindings);
        let result = self.evaluate_value(&callable_id, &body);
        self.parameter_bindings.pop();
        result
    }

    fn evaluate_member_access(
        &mut self,
        node_id: &NodeId,
        expression: &DeclaredExpression,
    ) -> Result<EvalOutcome, EvalStatus> {
        let [base] = expression.children.as_slice() else {
            return Err(EvalStatus::Malformed);
        };
        let member = expression
            .reference
            .as_deref()
            .ok_or(EvalStatus::Malformed)?;
        let path = reference_path(base).ok_or(EvalStatus::Unsupported)?;
        self.resolve_identifier(node_id, &format!("{path}.{member}"))
            .map(|id| self.evaluate_node(&id))
            .and_then(outcome_result)
    }

    fn bound_value(&self, reference: &str) -> Option<Quantity> {
        self.parameter_bindings.iter().rev().find_map(|bindings| {
            bindings
                .get(reference)
                .cloned()
                .or_else(|| {
                    reference
                        .rsplit("::")
                        .next()
                        .and_then(|tail| bindings.get(tail).cloned())
                })
                .map(|value| value.0)
        })
    }

    fn resolve_identifier(
        &self,
        current_id: &NodeId,
        reference: &str,
    ) -> Result<NodeId, EvalStatus> {
        if let Some((head, rest)) = reference.split_once('.') {
            let mut current = self.resolve_identifier(current_id, head)?;
            for segment in rest.split('.') {
                let qualified = format!("{}::{segment}", current.qualified_name);
                let candidates = self.lookup_candidates(&qualified);
                current = if candidates.is_empty() {
                    let owner = self
                        .graph
                        .get_node(&current)
                        .ok_or(EvalStatus::Unresolved)?;
                    match resolve_member_via_type(self.graph, owner, segment) {
                        ResolveResult::Resolved(id) => id,
                        ResolveResult::Ambiguous => return Err(EvalStatus::Ambiguous),
                        ResolveResult::Unresolved => return Err(EvalStatus::Unresolved),
                    }
                } else {
                    choose_candidate(self.graph, candidates)?
                };
            }
            return Ok(current);
        }
        let current = self
            .graph
            .get_node(current_id)
            .ok_or(EvalStatus::Unresolved)?;
        let mut candidates = Vec::new();
        let mut scopes = scope_prefixes(self.graph, current);
        scopes.insert(0, current.id.qualified_name.clone());
        scopes.extend(typed_case_definition_scope_prefixes(self.graph, current));
        scopes.extend(typed_requirement_definition_scope_prefixes(
            self.graph, current,
        ));
        for scope in scopes {
            candidates.extend(self.lookup_candidates(&format!("{scope}::{reference}")));
        }
        if candidates.is_empty() {
            if reference.contains("::") {
                candidates.extend(self.lookup_candidates(reference));
            } else {
                candidates.extend(
                    self.graph
                        .nodes_for_uri(&current.id.uri)
                        .into_iter()
                        .filter(|node| node.name == reference)
                        .map(|node| node.id.clone()),
                );
                candidates.extend(self.lookup_candidates(reference));
            }
        }
        choose_candidate(self.graph, dedupe(candidates))
    }

    fn lookup_candidates(&self, qualified: &str) -> Vec<NodeId> {
        self.graph
            .node_ids_by_qualified_name
            .get(qualified)
            .into_iter()
            .flatten()
            .cloned()
            .collect()
    }
}

fn child_expression(node: &SemanticNode) -> Option<DeclaredExpression> {
    node.declared_facts
        .feature_value
        .as_ref()
        .map(|value| value.expression.clone())
        .or_else(|| node.declared_facts.own_expression.clone())
}

fn real_literal(value: &Value) -> Option<f64> {
    value.as_f64().or_else(|| value.as_str()?.parse().ok())
}
fn bool_outcome(outcome: EvalOutcome) -> Result<bool, EvalStatus> {
    outcome_result(outcome)?
        .value
        .and_then(|value| value.as_bool())
        .ok_or(EvalStatus::TypeError)
}
fn outcome_result(outcome: EvalOutcome) -> Result<EvalOutcome, EvalStatus> {
    if outcome.status == EvalStatus::Ok {
        Ok(outcome)
    } else {
        Err(outcome.status)
    }
}
fn outcome_quantity(outcome: EvalOutcome) -> Result<Quantity, EvalStatus> {
    let outcome = outcome_result(outcome)?;
    Ok(Quantity {
        value: outcome
            .value
            .as_ref()
            .and_then(json_value_to_f64)
            .ok_or(EvalStatus::TypeError)?,
        unit: outcome.unit,
    })
}

fn unit_reference(expression: &DeclaredExpression) -> Option<String> {
    match expression.kind {
        DeclaredExpressionKind::FeatureReference | DeclaredExpressionKind::FeatureChain => {
            expression.reference.clone()
        }
        DeclaredExpressionKind::Bracket | DeclaredExpressionKind::Parenthesized => expression
            .children
            .as_slice()
            .first()
            .and_then(unit_reference),
        _ => None,
    }
}

fn reference_path(expression: &DeclaredExpression) -> Option<String> {
    match expression.kind {
        DeclaredExpressionKind::FeatureReference | DeclaredExpressionKind::FeatureChain => {
            expression.reference.clone()
        }
        DeclaredExpressionKind::MemberAccess => Some(format!(
            "{}.{}",
            reference_path(expression.children.first()?)?,
            expression.reference.as_deref()?
        )),
        DeclaredExpressionKind::Bracket | DeclaredExpressionKind::Parenthesized => {
            reference_path(expression.children.first()?)
        }
        _ => None,
    }
}

fn scope_prefixes(graph: &SemanticGraph, current: &SemanticNode) -> Vec<String> {
    graph
        .parent_of(current)
        .into_iter()
        .chain(graph.ancestors_of(current))
        .map(|node| node.id.qualified_name.clone())
        .collect()
}

fn typed_case_definition_id(graph: &SemanticGraph, usage: &SemanticNode) -> Option<NodeId> {
    let expected = match usage.element_kind {
        ElementKind::Analysis => ElementKind::AnalysisDef,
        ElementKind::Verification => ElementKind::VerificationDef,
        _ => return None,
    };
    graph
        .outgoing_targets_by_kind(usage, RelationshipKind::Typing)
        .into_iter()
        .find(|candidate| candidate.element_kind == expected)
        .map(|candidate| candidate.id.clone())
}

fn typed_requirement_definition_id(graph: &SemanticGraph, usage: &SemanticNode) -> Option<NodeId> {
    (usage.element_kind == ElementKind::Requirement)
        .then(|| {
            graph
                .outgoing_targets_by_kind(usage, RelationshipKind::Typing)
                .into_iter()
                .find(|candidate| candidate.element_kind == ElementKind::RequirementDef)
                .map(|candidate| candidate.id.clone())
        })
        .flatten()
}

fn choose_candidate(
    graph: &SemanticGraph,
    mut candidates: Vec<NodeId>,
) -> Result<NodeId, EvalStatus> {
    if candidates.is_empty() {
        return Err(EvalStatus::Unresolved);
    }
    candidates.sort_by(|left, right| {
        candidate_score(graph, right)
            .cmp(&candidate_score(graph, left))
            .then_with(|| left.qualified_name.len().cmp(&right.qualified_name.len()))
    });
    let best = candidates[0].clone();
    let score = candidate_score(graph, &best);
    let length = best.qualified_name.len();
    if candidates.iter().skip(1).any(|candidate| {
        candidate_score(graph, candidate) == score && candidate.qualified_name.len() == length
    }) {
        Err(EvalStatus::Ambiguous)
    } else {
        Ok(best)
    }
}

fn candidate_score(graph: &SemanticGraph, candidate: &NodeId) -> u8 {
    graph
        .get_node(candidate)
        .map(|node| {
            u8::from(
                node.declared_facts.feature_value.is_some()
                    || node.declared_facts.own_expression.is_some(),
            )
        })
        .unwrap_or_default()
}
fn dedupe(ids: Vec<NodeId>) -> Vec<NodeId> {
    let mut seen = HashSet::new();
    ids.into_iter()
        .filter(|id| seen.insert(id.clone()))
        .collect()
}

fn status_message(status: EvalStatus) -> &'static str {
    match status {
        EvalStatus::Ok => "expression evaluated",
        EvalStatus::Unresolved => "expression has an unresolved reference",
        EvalStatus::Ambiguous => "expression has an ambiguous reference",
        EvalStatus::Malformed => "expression is malformed or recovered",
        EvalStatus::Incomplete => "expression is incomplete",
        EvalStatus::TypeError => "expression has a type or unit mismatch",
        EvalStatus::DivByZero => "expression divides by zero",
        EvalStatus::Unsupported => "declared expression form is not supported",
        EvalStatus::Cycle => "expression has a cyclic dependency",
    }
}

//! Phase 2 lowering — constraints, calculations, expressions, filters, and unit tokens.

use crate::evaluate::classify::classify_filter_predicate;
use crate::evaluate::classify::is_arithmetic_operator;
use crate::evaluate::classify::is_comparison_operator;
use crate::evaluate::classify::is_logical_operator;
use crate::evaluate::classify::is_range_or_coalesce_operator;
use crate::evaluate::classify::is_unary_operator;
use crate::evaluate::fold::quantity_unit_text;
use crate::lower::facts::direction_fact;
use crate::lower::facts::multiplicity_facts;
use crate::lower::facts::AuthoredExpression;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DeclarationModifiers;
use crate::lower::facts::ExpressionGrammar;
use crate::lower::facts::FilterForm;
use crate::lower::facts::PendingReference;
use crate::lower::facts::RelationshipFlags;
use crate::lower::facts::UnsupportedFamily;
use crate::lower::SemanticModelBuilder;
use crate::model::ConstructionError;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::model::Visibility;
use std::sync::Arc;
use sysml_v2_parser::ast::{
    AssertConstraintMember, CalcDef, CalcDefBody, CalcDefBodyElement, CalcUsage as ParserCalcUsage,
    ConstraintDef, ConstraintDefBody, ConstraintDefBodyElement,
    ConstraintUsage as ParserConstraintUsage, Expression, MembershipKind as ParserMembershipKind,
    Node, RequireConstraint, SequenceExpressionList,
};

impl SemanticModelBuilder {
    /// Lowers a `constraint def`/`constraint` usage body's boolean expression (slice 1 of the
    /// constraint/calc expression fact family, widened by the arithmetic/logical-combinator slice
    /// to accept nested arithmetic and `and`/`or` combinators; see `ReferenceKind::
    /// ExpressionOperand`). Supports a literal, a feature/feature-chain reference (resolved as an
    /// `ExpressionOperand` reference sourced at `declaration`, exactly like `lower_succession_end`
    /// resolves `Expression::FeatureRef` through the shared `DeclarationDomain::Any` lexical lookup
    /// fixed point), a parenthesized wrapper (unwrapped and recursed into), a comparison `BinaryOp`
    /// (`Eq`/`Ne`/`Lt`/`Le`/`Gt`/`Ge` -- `StrictEq`/`StrictNe` KerML identity comparisons are
    /// deliberately excluded, left unsupported like every other operator), an arithmetic `BinaryOp`
    /// (`is_arithmetic_operator`, e.g. an operand like `chassisMass + engine.mass`), or a logical
    /// `BinaryOp` (`is_logical_operator`, `and`/`or`/`xor`/`implies`, combining multiple comparisons, e.g. `... and
    /// mass > 0[kg]`; `xor`/`implies` deliberately excluded) -- every one of these `BinaryOp` arms
    /// simply recurses into both operands identically, since reference resolution does not care
    /// which of the three operator families is used, only evaluation (`classify_constraint_node`)
    /// distinguishes them by building a different `EvalNode` shape. Evaluation itself is otherwise
    /// out of scope here. Also supports `Expression::Invocation`/`Expression::Constructor`
    /// (reference-resolution slice, see `ReferenceKind::InvocationCallee`/
    /// `lower_invocation_callee`): the callee/type name resolves as an `InvocationCallee` reference
    /// and each argument recurses back into this same function, but the invocation is never
    /// evaluated (`EvalNode::Invocation` always folds to `NonConstant`). Any other expression shape
    /// -- tuples, type-check/classification expressions, unary ops, etc. -- falls through to the
    /// existing unsupported-member diagnostic, unchanged from prior behavior.
    pub(crate) fn lower_constraint_expression(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::LiteralInteger(_)
            | Expression::LiteralReal(_)
            | Expression::LiteralBoolean(_)
            | Expression::LiteralString(_)
            | Expression::Null => Ok(()),
            Expression::Bracket { base, operands, .. } => {
                self.lower_unit_token(document, declaration, operands)?;
                self.lower_constraint_expression(document, declaration, family, base)
            }
            Expression::Index { base, operands, .. } => {
                self.lower_constraint_expression(document, declaration, family, base)?;
                for element in &operands.value.elements {
                    self.lower_constraint_expression(
                        document,
                        declaration,
                        family,
                        &element.expression,
                    )?;
                }
                Ok(())
            }
            Expression::Select { base, selector } => {
                self.lower_constraint_expression(document, declaration, family, base)?;
                self.push_expression_operand_reference(document, declaration, *selector)
            }
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ExpressionOperand,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::MemberAccess { .. } => {
                if self
                    .push_member_access_expression(declaration, document, node)?
                    .is_none()
                {
                    self.push_unsupported(document, family, node.span);
                }
                Ok(())
            }
            Expression::Sequence { operands, .. } => {
                // Grouping and comma-list spelling share one production; lowering recurses into
                // each operand either way.
                for element in &operands.value.elements {
                    self.lower_constraint_expression(
                        document,
                        declaration,
                        family,
                        &element.expression,
                    )?;
                }
                Ok(())
            }
            Expression::BodyExpr(body)
                if body.value.parameters.is_empty() && body.value.result.is_some() =>
            {
                self.lower_constraint_expression(
                    document,
                    declaration,
                    family,
                    body.value.result.as_deref().expect("guarded result"),
                )
            }
            Expression::BinaryOp { op, left, right }
                if is_comparison_operator(op)
                    || is_arithmetic_operator(op)
                    || is_logical_operator(op)
                    || is_range_or_coalesce_operator(op) =>
            {
                self.lower_constraint_expression(document, declaration, family, left)?;
                self.lower_constraint_expression(document, declaration, family, right)
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, declaration, callee, args.len(), node.span)?;
                for arg in args {
                    self.lower_constraint_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, declaration, *type_name)?;
                for arg in args {
                    self.lower_constraint_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::CollectionOp { base, args, .. } => {
                self.lower_constraint_expression(document, declaration, family, base)?;
                for arg in args {
                    self.lower_constraint_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
                self.lower_constraint_expression(document, declaration, family, operand)
            }
            Expression::Conditional {
                test,
                then_expr,
                else_expr,
            } => {
                self.lower_constraint_expression(document, declaration, family, test)?;
                self.lower_constraint_expression(document, declaration, family, then_expr)?;
                self.lower_constraint_expression(document, declaration, family, else_expr)
            }
            Expression::TypeCheck {
                operand, type_name, ..
            } => {
                if let Some(operand) = operand {
                    self.lower_constraint_expression(document, declaration, family, operand)?;
                }
                self.push_type_check_target_reference(document, declaration, *type_name)
            }
            Expression::MetaCast { base, metaclass } => {
                self.lower_constraint_expression(document, declaration, family, base)?;
                self.push_meta_cast_target_reference(document, declaration, *metaclass)
            }
            _ => {
                self.push_unsupported(document, family, node.span);
                Ok(())
            }
        }
    }

    /// Lowers a `calc def`/`calc` usage body's formula expression (slice 1 of the constraint/calc
    /// expression fact family, extended by slice 4 for arithmetic). Originally scoped to
    /// arithmetic-only `BinaryOp` support on the theory that calc bodies are typically
    /// arithmetic-result formulas rather than boolean comparisons -- but the exhaustive
    /// `unsupported_calc_definition_member` audit found this premise false for a large share of
    /// the real corpus (Kernel Function Library equality/comparison functions like
    /// `BaseFunctions.kerml`'s `return : Boolean[1] = not (x == y);`, KerML `inv { ... }`
    /// boolean-invariant bodies reusing this same `CalcDefBody`/`lower_calc_def_body` walker per
    /// `KermlInvariantMember`, etc.), so comparison/logical `BinaryOp` support now matches
    /// `lower_constraint_expression`'s `BinaryOp` arm exactly (`is_comparison_operator`/
    /// `is_logical_operator`, alongside `is_arithmetic_operator`'s `Add`/`Sub`/`Mul`/`Div`/`Mod`/
    /// `Exp`/`Pow`). This slice supports the minimal leaf shapes -- a literal, a feature/
    /// feature-chain reference (resolved as an `ExpressionOperand` reference exactly like
    /// `lower_constraint_expression`), a parenthesized wrapper -- plus every comparison/
    /// arithmetic/logical `BinaryOp` whose operands are recursed into just like
    /// `lower_constraint_expression`'s own `BinaryOp` arm. Also supports `Expression::Invocation`/
    /// `Expression::Constructor` (e.g. `sum(partMasses)`, `new PusherOutput(pusherForce)`),
    /// exactly like `lower_constraint_expression`'s own Invocation/Constructor arm: reference
    /// resolution only, never evaluation. Also supports a unary `-`/`not` `UnaryOp` (see
    /// `is_unary_operator`), recursing into its single operand exactly like `Parenthesized`, and a
    /// `Conditional` (`if <test> ? <then> else <else>`), recursing into all three sub-expressions
    /// reference-resolution-only exactly like `Tuple`'s multi-operand shape (no evaluation of
    /// which branch is taken). Every other expression shape stays unsupported, falling through to
    /// the existing `unsupported_calc_definition_member` diagnostic, unchanged from prior
    /// behavior.
    pub(crate) fn lower_calc_expression(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::LiteralInteger(_)
            | Expression::LiteralReal(_)
            | Expression::LiteralBoolean(_)
            | Expression::LiteralString(_)
            | Expression::Null => Ok(()),
            Expression::Bracket { base, operands, .. } => {
                self.lower_unit_token(document, declaration, operands)?;
                self.lower_calc_expression(document, declaration, family, base)
            }
            Expression::Index { base, operands, .. } => {
                self.lower_calc_expression(document, declaration, family, base)?;
                for element in &operands.value.elements {
                    self.lower_calc_expression(document, declaration, family, &element.expression)?;
                }
                Ok(())
            }
            Expression::Select { base, selector } => {
                self.lower_calc_expression(document, declaration, family, base)?;
                self.push_expression_operand_reference(document, declaration, *selector)
            }
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ExpressionOperand,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::MemberAccess { .. } => {
                if self
                    .push_member_access_expression(declaration, document, node)?
                    .is_none()
                {
                    self.push_unsupported(document, family, node.span);
                }
                Ok(())
            }
            Expression::Sequence { operands, .. } => {
                // Grouping and comma-list spelling share one production; lowering recurses into
                // each operand either way.
                for element in &operands.value.elements {
                    self.lower_calc_expression(document, declaration, family, &element.expression)?;
                }
                Ok(())
            }
            Expression::BodyExpr(body)
                if body.value.parameters.is_empty() && body.value.result.is_some() =>
            {
                self.lower_calc_expression(
                    document,
                    declaration,
                    family,
                    body.value.result.as_deref().expect("guarded result"),
                )
            }
            Expression::BinaryOp { op, left, right }
                if is_arithmetic_operator(op)
                    || is_comparison_operator(op)
                    || is_logical_operator(op)
                    || is_range_or_coalesce_operator(op) =>
            {
                self.lower_calc_expression(document, declaration, family, left)?;
                self.lower_calc_expression(document, declaration, family, right)
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, declaration, callee, args.len(), node.span)?;
                for arg in args {
                    self.lower_calc_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, declaration, *type_name)?;
                for arg in args {
                    self.lower_calc_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::CollectionOp { base, args, .. } => {
                self.lower_calc_expression(document, declaration, family, base)?;
                for arg in args {
                    self.lower_calc_expression(document, declaration, family, &arg.value)?;
                }
                Ok(())
            }
            Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
                self.lower_calc_expression(document, declaration, family, operand)
            }
            Expression::Conditional {
                test,
                then_expr,
                else_expr,
            } => {
                self.lower_calc_expression(document, declaration, family, test)?;
                self.lower_calc_expression(document, declaration, family, then_expr)?;
                self.lower_calc_expression(document, declaration, family, else_expr)
            }
            Expression::TypeCheck {
                operand, type_name, ..
            } => {
                if let Some(operand) = operand {
                    self.lower_calc_expression(document, declaration, family, operand)?;
                }
                self.push_type_check_target_reference(document, declaration, *type_name)
            }
            Expression::MetaCast { base, metaclass } => {
                self.lower_calc_expression(document, declaration, family, base)?;
                self.push_meta_cast_target_reference(document, declaration, *metaclass)
            }
            _ => {
                self.push_unsupported(document, family, node.span);
                Ok(())
            }
        }
    }

    /// Lowers a package-level `filter <expr>;` statement's condition (BNF `ElementFilterMember`,
    /// `ast::FilterMember`, see `PackageBodyElement::Filter`), narrowing a recursive import to only
    /// members satisfying the expression. Reuses `lower_constraint_expression`'s operand-resolution
    /// shape as closely as the filter grammar allows: a literal (recognized, no reference), a
    /// feature/feature-chain reference (`Expression::FeatureRef`/`FeatureChainRef`, e.g.
    /// `Safety::isMandatory`, resolved as `ReferenceKind::ExpressionOperand` through the same
    /// `DeclarationDomain::Any` lexical lookup fixed point), a parenthesized wrapper (unwrapped and
    /// recursed into), and a comparison `BinaryOp` (`is_comparison_operator`) whose operands are
    /// recursed into, exactly like `lower_constraint_expression`.
    ///
    /// Two shapes are specific to filter conditions and have no analog in
    /// `lower_constraint_expression`: an `@Name` metadata-classification test
    /// (`Expression::Classification`, e.g. `@Safety`) is resolved as a new
    /// `ReferenceKind::FilterMetadataTest` reference through the same `DeclarationDomain::Type`
    /// lexical lookup fixed point `MetadataAnnotation` uses (`Safety` must name a metadata def);
    /// and a logical `BinaryOp` (`and`/`or`/`xor`/`implies`, `is_logical_operator`) whose operands are recursed
    /// into, alongside comparison operators.
    ///
    /// `declaration` is the enclosing package's own declaration (the filter statement's owner,
    /// sourced directly, no anonymous nested-declaration scope shift -- mirroring
    /// `ExpressionOperand`'s shape). Evaluation of the filter (computing which imported members
    /// actually pass it) is explicitly out of scope for this slice; only the condition's own
    /// references are resolved. Any other expression shape falls through to
    /// `UnsupportedFamily::PackageMember`'s `unsupported_package_member` diagnostic, matching the
    /// unconditional `unsupported_package_member` this statement produced before this slice.
    /// Records the authored unit token of a `value [unit]` quantity literal.
    ///
    /// `unit` is the parser's bracketed unit node, whose span is exactly the token between the
    /// brackets, so a diagnostic about the unit points at the unit and not at the whole literal.
    /// A shape `quantity_unit_text` does not recognise records nothing rather than a guess: the
    /// literal still publishes its value, and no unit fact claims a token that was not written.
    pub(crate) fn lower_unit_token(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        operands: &Node<SequenceExpressionList>,
    ) -> Result<(), ConstructionError> {
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let Some(text) = quantity_unit_text(&parsed, &operands.value) else {
            return Ok(());
        };
        self.push_unit_token(declaration, document, &text, operands.span)
    }

    /// Lowers one authored `filter` condition: its references, and the classified expression that
    /// lets the barrier settle what it evaluates to.
    pub(crate) fn lower_filter_condition(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        form: FilterForm,
        condition: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        let expression = AuthoredExpression {
            document,
            grammar: ExpressionGrammar::Constraint,
            operand_start: self.expression_operand_offset(owner),
            node: condition.value.clone(),
        };
        let mut metadata_ordinal = self
            .next_reference_ordinals
            .get(&(owner, ReferenceKind::FilterMetadataTest))
            .copied()
            .unwrap_or(0);
        let predicate = classify_filter_predicate(&condition.value, &mut metadata_ordinal);
        self.push_filter_condition(owner, document, form, condition.span, expression, predicate)?;
        self.lower_filter_expression(document, owner, condition)
    }

    pub(crate) fn lower_filter_expression(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        node: &Node<Expression>,
    ) -> Result<(), ConstructionError> {
        match &node.value {
            Expression::LiteralInteger(_)
            | Expression::LiteralReal(_)
            | Expression::LiteralBoolean(_)
            | Expression::LiteralString(_) => Ok(()),
            Expression::Bracket { base, operands, .. } => {
                self.lower_unit_token(document, declaration, operands)?;
                self.lower_filter_expression(document, declaration, base)
            }
            Expression::Index { base, operands, .. } => {
                self.lower_filter_expression(document, declaration, base)?;
                for element in &operands.value.elements {
                    self.lower_filter_expression(document, declaration, &element.expression)?;
                }
                Ok(())
            }
            Expression::Select { base, selector } => {
                self.lower_filter_expression(document, declaration, base)?;
                self.push_expression_operand_reference(document, declaration, *selector)
            }
            Expression::FeatureRef(target) | Expression::FeatureChainRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ExpressionOperand,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::Classification { metaclass } => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*metaclass)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::FilterMetadataTest,
                    document,
                    local: *metaclass,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
                Ok(())
            }
            Expression::MemberAccess { .. } => {
                if self
                    .push_member_access_expression(declaration, document, node)?
                    .is_none()
                {
                    self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span);
                }
                Ok(())
            }
            Expression::Sequence { operands, .. } => {
                // Grouping and comma-list spelling share one production; lowering recurses into
                // each operand either way.
                for element in &operands.value.elements {
                    self.lower_filter_expression(document, declaration, &element.expression)?;
                }
                Ok(())
            }
            Expression::BinaryOp { op, left, right }
                if is_comparison_operator(op) || is_logical_operator(op) =>
            {
                self.lower_filter_expression(document, declaration, left)?;
                self.lower_filter_expression(document, declaration, right)
            }
            Expression::Invocation { callee, args } => {
                self.lower_invocation_callee(document, declaration, callee, args.len(), node.span)?;
                for arg in args {
                    self.lower_filter_expression(document, declaration, &arg.value)?;
                }
                Ok(())
            }
            Expression::Constructor { type_name, args } => {
                self.push_invocation_callee_reference(document, declaration, *type_name)?;
                for arg in args {
                    self.lower_filter_expression(document, declaration, &arg.value)?;
                }
                Ok(())
            }
            Expression::CollectionOp { base, args, .. } => {
                self.lower_filter_expression(document, declaration, base)?;
                for arg in args {
                    self.lower_filter_expression(document, declaration, &arg.value)?;
                }
                Ok(())
            }
            Expression::TypeCheck {
                operand, type_name, ..
            } => {
                if let Some(operand) = operand {
                    self.lower_filter_expression(document, declaration, operand)?;
                }
                self.push_type_check_target_reference(document, declaration, *type_name)
            }
            Expression::MetaCast { base, metaclass } => {
                self.lower_filter_expression(document, declaration, base)?;
                self.push_meta_cast_target_reference(document, declaration, *metaclass)
            }
            Expression::UnaryOp { op, operand } if is_unary_operator(op) => {
                self.lower_filter_expression(document, declaration, operand)
            }
            _ => {
                self.push_unsupported(document, UnsupportedFamily::PackageMember, node.span);
                Ok(())
            }
        }
    }

    /// Lowers a `rendering def` (BNF RenderingDefinition), mirroring `lower_view_def`: ownership,
    /// membership, an optional `:>` specialization relationship (participates in the shared
    /// `DeclarationDomain::Type` fixed point). Render-specific body members (`filter`/`render`)
    /// are out of scope -- see `DeclarationKind::RenderingDefinition`'s doc comment.
    /// Lowers a `constraint def` (BNF ConstraintDefinition), mirroring `lower_view_def`:
    /// ownership, membership, an optional `:>` specialization relationship participating in the
    /// shared `DeclarationDomain::Type` fixed point. Constraint-body expression content is out of
    /// scope for this slice and falls through to `UnsupportedFamily::ConstraintDefinitionMember`.
    pub(crate) fn lower_constraint_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ConstraintDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConstraintDefinition,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Shared body walker for `constraint def`/`constraint` usage bodies (both use
    /// `ConstraintDefBody`/`ConstraintDefBodyElement` in the typed AST -- there is no separate
    /// `ConstraintUsageBody`), mirroring `lower_view_def_body`. Expression/nested-constraint
    /// content falls through to `unsupported_constraint_definition_member`.
    pub(crate) fn lower_constraint_def_body(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        body: &ConstraintDefBody,
    ) -> Result<(), ConstructionError> {
        if let ConstraintDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    ConstraintDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span);
                    }
                    ConstraintDefBodyElement::AliasDef(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::ConstraintDefinitionMember,
                            node.span,
                        );
                    }
                    ConstraintDefBodyElement::ReturnDecl(node) => {
                        self.lower_return_decl(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ConstraintDefinitionMember,
                            node,
                        )?;
                    }
                    ConstraintDefBodyElement::Constraint(constraint) => {
                        self.lower_constraint_usage(document, Some(declaration), constraint)?;
                    }
                    ConstraintDefBodyElement::PartUsage(part_usage) => {
                        self.lower_part_usage(document, Some(declaration), part_usage)?;
                    }
                    ConstraintDefBodyElement::InOutDecl(param) => {
                        self.lower_parameter_declaration(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ConstraintDefinitionMember,
                            param,
                        )?;
                    }
                    ConstraintDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ConstraintDefinitionMember,
                        element.span,
                    ),
                    ConstraintDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ConstraintDefinitionMember,
                            member,
                        )?;
                    }
                    ConstraintDefBodyElement::Expression(expression) => {
                        self.push_evaluation_fact(
                            declaration,
                            self.constraint_expression_site(document, &expression.value),
                        );
                        self.lower_constraint_expression(
                            document,
                            declaration,
                            UnsupportedFamily::ConstraintDefinitionMember,
                            expression,
                        )?
                    }
                    ConstraintDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    ConstraintDefBodyElement::FeatureDecl(node) => self
                        .lower_default_reference_usage(
                            document,
                            Some(declaration),
                            UnsupportedFamily::ConstraintDefinitionMember,
                            node,
                        )?,
                    ConstraintDefBodyElement::RequireConstraint(node) => self
                        .lower_require_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::ConstraintDefinitionMember,
                            node,
                        )?,
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `constraint` feature member (BNF
    /// ConstraintUsage), mirroring `lower_analysis_case_usage`: ownership, membership, a `:`
    /// typing target, and `subsets`/`redefines` subsetting relationships. Resolved upstream in
    /// `0757de13` (planning/UPSTREAM_PARSER_GAPS.md #4): `ConstraintUsage` previously had no
    /// `subsets`/`redefines` fields at all.
    pub(crate) fn lower_constraint_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserConstraintUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.name)?;
        let short_name = self.intern_short_name(document, node.value.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConstraintUsage,
            name,
            node.span,
            // `ast::ConstraintUsage` carries no modifier or direction field.
            DeclarationFacts {
                short_name,
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Lowers `assert constraint { <boolExpr> }` / `assert constraint <name> : <ConstraintDef>
    /// { ... }` (BNF `AssertConstraintMember`, `AssertConstraintUsage`): semantically an inline,
    /// anonymous (or named) constraint usage introduced via `assert` rather than the bare
    /// `constraint` keyword. Mirrors `lower_first_stmt`'s "anonymous nested declaration" pattern
    /// (`Succession`) and reuses `lower_constraint_usage`'s typing + `lower_constraint_def_body`
    /// wiring verbatim -- `AssertConstraintMember.body` is the exact same `ConstraintDefBody`
    /// shape as `ConstraintDef`/`ConstraintUsage`, so the existing
    /// `lower_constraint_expression`/`classify_expression` evaluation machinery (Slice
    /// 1, `4ca42166`) applies unchanged.
    ///
    /// Deferred (falls through to `family`'s unsupported diagnostic):
    /// - `assert <path> { ... }` shorthand (`target` set, no `constraint` keyword): references an
    ///   existing constraint by path rather than declaring one inline; out of scope for this
    ///   slice, which targets the `constraint`-keyword forms only.
    pub(crate) fn lower_assert_constraint_member(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<AssertConstraintMember>,
    ) -> Result<(), ConstructionError> {
        if node.value.target.is_some() {
            self.push_unsupported(document, family, node.span);
            return Ok(());
        }
        let name = self.intern_declaration_name(document, node.value.declaration_name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::AssertConstraintUsage,
            name,
            node.span,
            DeclarationFacts {
                negated: Some(node.value.is_negated),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a requirement/objective/case-family-def-body `require`/`assume` constraint member
    /// (BNF `RequireConstraint`, `ast::RequireConstraint`): the `require constraint { ... }` /
    /// `assume constraint <name> { ... }` shape (`has_constraint_keyword == true`) declares an
    /// anonymous or named nested `ConstraintUsage` feature, structurally identical to
    /// `AssertConstraintMember`'s constraint-keyword form (`lower_assert_constraint_member`) minus
    /// the `is_negated`/shorthand-`target`/`type_name` operands `AssertConstraintMember` has and
    /// `RequireConstraint` does not. Its body *is* `ConstraintDefBody` upstream (the duplicate
    /// `RequireConstraintBody` name collapsed into the type it was always equal to), so it is
    /// dispatched through the existing `lower_constraint_def_body` walker unchanged.
    ///
    /// Deferred (falls through to `family`'s unsupported diagnostic): the `require <name>;` /
    /// `require <name> { ... }` shorthand (`has_constraint_keyword == false`), which references an
    /// *existing* constraint by name rather than declaring one. Upstream now carries that role on
    /// its own arena-backed `RequireConstraint::target`, so it can participate in the shared
    /// lexical-lookup reference machinery; wiring it is pending
    /// (planning/UPSTREAM_PARSER_GAPS.md, "Typed upstream, not yet lowered here"). Likewise
    /// `require constraint <name> : <Type>;` / `require
    /// constraint <name> :>> <target>;` (a `:`/`:>>` clause after the name) fails to parse as
    /// `RequireConstraint` at all upstream (no field for either), so those never reach this
    /// function in the first place.
    pub(crate) fn lower_require_constraint_member(
        &mut self,
        document: DocumentIdx,
        owner: DeclarationId,
        family: UnsupportedFamily,
        node: &Node<RequireConstraint>,
    ) -> Result<(), ConstructionError> {
        let name = if node.value.has_constraint_keyword {
            self.intern_declaration_name(document, node.value.name)?
        } else {
            None
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            if node.value.is_assume {
                DeclarationKind::AssumeConstraintUsage
            } else {
                DeclarationKind::RequireConstraintUsage
            },
            name,
            node.span,
            // `has_constraint_keyword` selects the authored form (checked above) rather than
            // modifying the declaration; `is_assume` rides the declaration kind, because it is
            // what makes `RequirementConstraintMembership.kind` derivable.
            DeclarationFacts::none(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span,
        )?;
        // `require constraint c : C;` -- `ConstraintUsageDeclaration` is an ordinary
        // `UsageDeclaration` (SysML BNF 2066-2071), so the declared usage may be typed.
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if !node.value.has_constraint_keyword {
            let Some(target) = node.value.target else {
                self.push_unsupported(document, family, node.span);
                return Ok(());
            };
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::Subsetting,
                document,
                local: target,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_constraint_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a `calc def` (BNF CalculationDefinition), mirroring `lower_action_def`: ownership,
    /// membership, an optional `:>` specialization relationship participating in the shared
    /// `DeclarationDomain::Type` fixed point. Resolved upstream in `0757de13`
    /// (planning/UPSTREAM_PARSER_GAPS.md #3): `CalcDef` previously dropped its parsed `:>` clause.
    /// Calculation-expression body content is out of scope and falls through to
    /// `UnsupportedFamily::CalcDefinitionMember`.
    pub(crate) fn lower_calc_def(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<CalcDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CalcDefinition,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span,
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }

    /// Shared body walker for `calc def`/`calc` usage bodies (both use `CalcDefBody`/
    /// `CalcDefBodyElement` in the typed AST -- there is no separate `CalcUsageBody`), mirroring
    /// `lower_constraint_def_body`. Calculation-expression content, in/out/return parameters, and
    /// nested calc structure fall through to `unsupported_calc_definition_member`.
    pub(crate) fn lower_calc_def_body(
        &mut self,
        document: DocumentIdx,
        declaration: DeclarationId,
        body: &CalcDefBody,
    ) -> Result<(), ConstructionError> {
        if let CalcDefBody::Brace { elements, .. } = body {
            for element in elements {
                match &element.value {
                    CalcDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span);
                    }
                    // KerML `flow of <payload> from <a> to <b>;` in a calc-shaped body
                    // (`classifier`/`struct`/`class`/`behavior`, KerML 8.2's `Flow`). Upstream
                    // types the whole declaration, so it lowers through the same
                    // `lower_flow_usage` an action body uses rather than being reported as an
                    // unsupported member.
                    CalcDefBodyElement::FlowUsage(node) => {
                        self.lower_flow_usage(document, declaration, node)?;
                    }
                    CalcDefBodyElement::AliasDef(node) => {
                        // New upstream member kind: kept visible as unsupported rather than dropped.
                        self.push_unsupported(
                            document,
                            UnsupportedFamily::CalcDefinitionMember,
                            node.span,
                        );
                    }
                    CalcDefBodyElement::CalcUsage(calc_usage) => {
                        self.lower_calc_usage(document, Some(declaration), calc_usage)?;
                    }
                    CalcDefBodyElement::CalcDef(calc_def) => {
                        self.lower_calc_def(document, Some(declaration), calc_def)?;
                    }
                    CalcDefBodyElement::PartUsage(part_usage) => {
                        self.lower_part_usage(document, Some(declaration), part_usage)?;
                    }
                    // `MemberPrefix Package`/`LibraryPackage` in a KerML type body, lowered
                    // through the same owners a top-level package declaration uses.
                    CalcDefBodyElement::Package(member) => {
                        self.lower_package(document, Some(declaration), &member.package)?;
                    }
                    CalcDefBodyElement::LibraryPackage(member) => {
                        self.lower_library_package(document, Some(declaration), &member.package)?;
                    }
                    CalcDefBodyElement::MetadataKeywordUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::CalcDefinitionMember,
                        element.span,
                    ),
                    // `CalculationBodyItem = ActionBodyItem | ReturnParameterMember`, so a
                    // calculation body owns every action-body member as well as its own `return`.
                    // They arrive through the action dispatcher rather than as restated variants,
                    // and lower through the owner that already lowers them in an action body.
                    CalcDefBodyElement::ActionMember(node) => {
                        self.lower_action_def_body_element(document, declaration, node)?;
                    }
                    CalcDefBodyElement::KermlRelationship(node) => {
                        self.lower_kerml_relationship_decl(document, declaration, node)?;
                    }
                    CalcDefBodyElement::Annotating(member) => {
                        self.lower_annotating_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::CalcDefinitionMember,
                            member,
                        )?;
                    }
                    CalcDefBodyElement::Expression(expression) => {
                        self.push_evaluation_fact(
                            declaration,
                            self.calc_expression_site(document, &expression.value),
                        );
                        self.lower_calc_expression(
                            document,
                            declaration,
                            UnsupportedFamily::CalcDefinitionMember,
                            expression,
                        )?
                    }
                    CalcDefBodyElement::ReturnDecl(return_decl) => {
                        self.lower_return_decl(
                            document,
                            Some(declaration),
                            UnsupportedFamily::CalcDefinitionMember,
                            return_decl,
                        )?;
                    }
                    CalcDefBodyElement::AttributeUsage(nested) => {
                        self.lower_attribute_usage(document, Some(declaration), nested)?;
                    }
                    CalcDefBodyElement::KermlClassifier(nested) => {
                        self.lower_kerml_classifier_decl(document, Some(declaration), nested)?;
                    }
                    CalcDefBodyElement::KermlFeature(nested) => {
                        self.lower_kerml_feature_member(
                            document,
                            Some(declaration),
                            UnsupportedFamily::CalcDefinitionMember,
                            nested,
                        )?;
                    }
                    CalcDefBodyElement::DefaultReferenceUsage(node) => {
                        self.lower_default_reference_usage(
                            document,
                            Some(declaration),
                            UnsupportedFamily::CalcDefinitionMember,
                            node,
                        )?;
                    }
                    CalcDefBodyElement::Invariant(node) => {
                        self.lower_kerml_invariant_member(document, Some(declaration), node)?;
                    }
                    CalcDefBodyElement::Connector(node) => {
                        self.lower_kerml_connector_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::Binding(node) => {
                        self.lower_kerml_binding_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::Succession(node) => {
                        self.lower_kerml_succession_member(document, declaration, node)?;
                    }
                    CalcDefBodyElement::Import(node) => {
                        self.lower_import(document, Some(declaration), node)?;
                    }
                    CalcDefBodyElement::AssertConstraint(node) => {
                        self.lower_assert_constraint_member(
                            document,
                            declaration,
                            UnsupportedFamily::CalcDefinitionMember,
                            node,
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `calc` feature member (BNF CalculationUsage),
    /// mirroring `lower_analysis_case_usage`: ownership, membership, a `:` typing target, and
    /// `redefines` targets. Unlike other usage kinds, `CalcUsage::redefines` is a bare
    /// `Vec<QualifiedReferenceId>` rather than a `Node<SubsettingRelationship>` (and there is no
    /// `subsets` field at all), so each target is pushed as its own `Redefinition` reference
    /// using that target's own resolved span (via `qualified_reference`) rather than through
    /// `lower_subsetting_relationship`. `in`/`out`/`inout` direction, value binding, and
    /// calculation-expression body content are out of scope, sharing
    /// `UnsupportedFamily::CalcDefinitionMember` with the `def` form.
    pub(crate) fn lower_calc_usage(
        &mut self,
        document: DocumentIdx,
        owner: Option<DeclarationId>,
        node: &Node<ParserCalcUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declaration_name(document, node.value.identification.name)?;
        let short_name = self.intern_short_name(document, node.identification.short_name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::CalcUsage,
            name,
            node.span,
            DeclarationFacts {
                short_name,
                modifiers: DeclarationModifiers {
                    is_abstract: node.value.is_abstract,
                    ..DeclarationModifiers::default()
                },
                direction: direction_fact(node.value.direction.as_ref()),
                multiplicity: multiplicity_facts(node.value.multiplicity.as_ref()),
                ..DeclarationFacts::none()
            },
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span,
        )?;
        // Constructs the canonical value Expression/result and preserves its authored spelling.
        // This usage family does not yet classify the expression operands.
        if let Some(feature_value) = &node.value.value {
            self.record_feature_value(document, declaration, feature_value)?;
        }
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span;
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(targets) = &node.value.redefines {
            for target in targets.iter().copied() {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span;
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::Redefinition,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_calc_def_body(document, declaration, &node.value.body)
    }
}

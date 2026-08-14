//! Opaque facade for the parser-owned resolved semantic slice.

use std::fmt;

pub use sysml_resolution::{
    AnnotationForm, AuthoredValue, BuildMeasurements, Conformance, ConformanceObstacle,
    Documentation, EffectiveType, EffectiveTypeOrigin, ElementInspection, ElementInspectionAt,
    ElementKind, ElementModifier, ElementRelationship, EvaluatedScalar, EvaluationFailure,
    EvaluationState, FeatureDirection, MembershipFacts, MembershipKind, MembershipRole,
    MultiplicityBound, MultiplicityFacts, NavigationTarget, OccurrenceRole, PortionKind,
    PublicationCompleteness, QueryOutcome, ReferenceAt, RelationshipProvenance, RelationshipTarget,
    RenameOutcome, RequirementConstraintKind, SourceLocation, SpecializationScope,
    StateSubactionKind, SubsettingConformance, SymbolEntry, SymbolIdentity, TextPosition,
    TextRange, TypeReference, ValueKind, Visibility, VisibilityProvenance, VisibleMember,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    inner: sysml_resolution::SourceInput,
}

impl SourceDocument {
    pub fn from_uri(
        uri: &str,
        content: String,
        source_kind: SourceKind,
    ) -> Result<Self, SourceError> {
        if uri.is_empty() {
            return Err(SourceError("source identity must not be empty"));
        }
        Ok(Self {
            inner: sysml_resolution::SourceInput::new(uri, content, source_kind.into()),
        })
    }

    pub fn from_memory_path(
        namespace: &str,
        path: &str,
        content: String,
        source_kind: SourceKind,
    ) -> Result<Self, SourceError> {
        let normalized = path.trim_start_matches('/').replace('\\', "/");
        if namespace.is_empty() || normalized.is_empty() {
            return Err(SourceError("source identity must not be empty"));
        }
        Ok(Self {
            inner: sysml_resolution::SourceInput::new(
                format!("memory://{namespace}/{normalized}"),
                content,
                source_kind.into(),
            ),
        })
    }
}

impl From<SourceKind> for sysml_resolution::SourceKind {
    fn from(kind: SourceKind) -> Self {
        match kind {
            SourceKind::Workspace => Self::Workspace,
            SourceKind::StandardLibrary => Self::StandardLibrary,
            SourceKind::Library => Self::Library,
            SourceKind::External => Self::External,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(&'static str);

impl fmt::Display for SourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for SourceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructionStrategy {
    Sequential,
    Parallel,
}

#[derive(Debug)]
pub struct BuildRequest {
    inner: sysml_resolution::BuildRequest,
}

impl BuildRequest {
    pub fn resolved(
        sources: Vec<SourceDocument>,
        construction: ConstructionStrategy,
    ) -> Result<Self, BuildError> {
        let schedule = match construction {
            ConstructionStrategy::Sequential => sysml_resolution::ConstructionSchedule::Sequential,
            ConstructionStrategy::Parallel => sysml_resolution::ConstructionSchedule::Parallel,
        };
        sysml_resolution::BuildRequest::new(
            sources.into_iter().map(|source| source.inner).collect(),
            schedule,
            "parser-owned-resolution-v1",
        )
        .map(|inner| Self { inner })
        .map_err(BuildError)
    }
}

/// Opaque published semantic state. Share it behind `Arc`; do not duplicate its owner.
///
/// ```compile_fail
/// fn requires_clone<T: Clone>() {}
/// requires_clone::<sysml_query::resolved_slice::PublishedModel>();
/// ```
#[derive(Debug)]
pub struct PublishedModel {
    inner: sysml_resolution::PublishedResolution,
}

pub fn build(request: BuildRequest) -> Result<PublishedModel, BuildError> {
    sysml_resolution::build(request.inner)
        .map(|inner| PublishedModel { inner })
        .map_err(BuildError)
}

/// Builds one publication and returns timings measured at its semantic phase barriers.
pub fn build_measured(
    request: BuildRequest,
) -> Result<(PublishedModel, BuildMeasurements), BuildError> {
    sysml_resolution::build_measured(request.inner)
        .map(|(inner, measurements)| (PublishedModel { inner }, measurements))
        .map_err(BuildError)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildError(sysml_resolution::BuildFailure);

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for BuildError {}

impl PublishedModel {
    pub fn debug(&self) -> DebugQueries<'_> {
        DebugQueries { model: &self.inner }
    }

    pub fn publication(&self) -> PublicationQueries<'_> {
        PublicationQueries { model: &self.inner }
    }

    pub fn navigation(&self) -> NavigationQueries<'_> {
        NavigationQueries { model: &self.inner }
    }

    pub fn edits(&self) -> EditQueries<'_> {
        EditQueries { model: &self.inner }
    }

    pub fn completion(&self) -> CompletionQueries<'_> {
        CompletionQueries { model: &self.inner }
    }

    pub fn inspection(&self) -> InspectionQueries<'_> {
        InspectionQueries { model: &self.inner }
    }

    pub fn types(&self) -> TypeQueries<'_> {
        TypeQueries { model: &self.inner }
    }
}

/// Direct types, supertypes, subtypes, effective types, featuring types and conformance.
///
/// Every answer is read from facts the publication settled before it became visible; none of these
/// calls traverses the model, and repeating one cannot change what it returns.
pub struct TypeQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl TypeQueries<'_> {
    /// The types a feature declares.
    pub fn direct_types(&self, symbol: &SymbolIdentity) -> QueryOutcome<Box<[TypeReference]>> {
        self.model.direct_types(symbol)
    }

    /// The types a feature has, directly or inherited along its subsetting/redefinition chain.
    pub fn effective_types(&self, symbol: &SymbolIdentity) -> QueryOutcome<Box<[EffectiveType]>> {
        self.model.effective_types(symbol)
    }

    /// The supertypes one specialization edge away.
    pub fn direct_supertypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.direct_supertypes(symbol, scope)
    }

    /// Every supertype, reflexively including `symbol` itself, as the Pilot's `allSupertypes` does.
    pub fn all_supertypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.all_supertypes(symbol, scope)
    }

    /// The declarations one specialization edge below `symbol`.
    pub fn direct_subtypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        self.model.direct_subtypes(symbol, scope)
    }

    /// The type that features `symbol`, if any.
    pub fn featuring_type(&self, symbol: &SymbolIdentity) -> QueryOutcome<Option<SymbolIdentity>> {
        self.model.featuring_type(symbol)
    }

    /// Whether `specific` conforms to `general` (KerML §8.4.3.2).
    pub fn conforms_to(
        &self,
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Conformance> {
        self.model.conforms_to(specific, general, scope)
    }

    /// Whether the specific feature's types conform to the general feature's (KerML §7.4.12).
    pub fn feature_typing_conforms(
        &self,
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
    ) -> QueryOutcome<Conformance> {
        self.model.feature_typing_conforms(specific, general)
    }

    /// Both halves of the subsetting rule (KerML §8.4.3.4), reported separately.
    pub fn subsetting_conforms(
        &self,
        subsetting: &SymbolIdentity,
        subsetted: &SymbolIdentity,
    ) -> QueryOutcome<SubsettingConformance> {
        self.model.subsetting_conforms(subsetting, subsetted)
    }
}

pub struct PublicationQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl PublicationQueries<'_> {
    pub fn completeness(&self) -> PublicationCompleteness {
        self.model.completeness()
    }
}

pub struct NavigationQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl NavigationQueries<'_> {
    pub fn target_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<NavigationTarget> {
        self.model.target_at(document, position)
    }

    pub fn references(
        &self,
        symbol: &SymbolIdentity,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        self.model.references(symbol, include_declaration)
    }
}

pub struct EditQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl EditQueries<'_> {
    pub fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        self.model.prepare_rename(document, position, new_name)
    }
}

pub struct CompletionQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl CompletionQueries<'_> {
    pub fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<Box<[VisibleMember]>> {
        self.model.visible_members(document, position, qualifier)
    }
}

/// Element inspection and document symbols.
///
/// The `PRODUCTION_CUTOVER.md` row this serves names `sysml_query` as the owner of the typed
/// service, so the contract is reachable here rather than only from the owning crate that
/// consumers are not permitted to depend on.
pub struct InspectionQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

impl InspectionQueries<'_> {
    /// Everything the publication knows about one element.
    pub fn inspect(&self, symbol: &SymbolIdentity) -> QueryOutcome<ElementInspection> {
        self.model.inspect(symbol)
    }

    /// The element whose declaration encloses `position`, and what a reference there points at.
    pub fn inspect_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ElementInspectionAt> {
        self.model.inspect_at(document, position)
    }

    /// Every element declared in one document, in source order.
    pub fn document_symbols(&self, document: &str) -> QueryOutcome<Box<[SymbolEntry]>> {
        self.model.document_symbols(document)
    }
}

pub struct DebugQueries<'a> {
    model: &'a sysml_resolution::PublishedResolution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorProbe {
    pub document: String,
    pub position: TextPosition,
    pub qualifier: Option<String>,
    pub rename_to: Option<String>,
}

impl DebugQueries<'_> {
    pub fn write_semantic_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_semantic_sexpr(output)
    }

    pub fn write_diagnostics_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_diagnostics_sexpr(output)
    }

    pub fn write_navigation_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_navigation_sexpr(output)
    }

    pub fn write_types_sexpr(&self, output: &mut dyn fmt::Write) -> fmt::Result {
        self.model.debug().write_types_sexpr(output)
    }

    pub fn write_editor_queries_sexpr(
        &self,
        probes: &[EditorProbe],
        output: &mut dyn fmt::Write,
    ) -> fmt::Result {
        writeln!(output, "(editor-queries")?;
        for probe in probes {
            writeln!(
                output,
                "  (probe (document {:?}) (position {} {})",
                probe.document, probe.position.line, probe.position.character
            )?;
            let target = self.model.target_at(&probe.document, probe.position);
            write_target_outcome(output, "target", &target)?;
            if let QueryOutcome::Resolved(target)
            | QueryOutcome::Recovered(target)
            | QueryOutcome::UnsupportedWith(target) = &target
            {
                write_locations_outcome(
                    output,
                    "references",
                    &self.model.references(&target.symbol, true),
                )?;
            }
            write_rename_outcome(
                output,
                &self.model.prepare_rename(
                    &probe.document,
                    probe.position,
                    probe.rename_to.as_deref(),
                ),
            )?;
            write_members_outcome(
                output,
                &self.model.visible_members(
                    &probe.document,
                    probe.position,
                    probe.qualifier.as_deref(),
                ),
            )?;
            write_inspection_at_outcome(
                output,
                &self.model.inspect_at(&probe.document, probe.position),
            )?;
            writeln!(output, "  )")?;
        }
        // Once per probed document rather than once per probe: the outline is a property of the
        // document, and repeating it would make a fixture's probe count decide how much of the
        // snapshot is outline.
        let mut written = Vec::new();
        for probe in probes {
            if written.contains(&probe.document) {
                continue;
            }
            written.push(probe.document.clone());
            write_document_symbols(
                output,
                &probe.document,
                &self.model.document_symbols(&probe.document),
            )?;
        }
        write!(output, ")")
    }
}

fn write_document_symbols(
    output: &mut dyn fmt::Write,
    document: &str,
    outcome: &QueryOutcome<Box<[SymbolEntry]>>,
) -> fmt::Result {
    writeln!(output, "  (document-symbols (document {document:?})")?;
    match outcome {
        QueryOutcome::Resolved(entries)
        | QueryOutcome::Recovered(entries)
        | QueryOutcome::UnsupportedWith(entries) => {
            writeln!(output, "    (status {})", outcome_status(outcome))?;
            for entry in entries.iter() {
                write!(output, "    (symbol (kind {:?})", entry.kind.as_str())?;
                if let Some(name) = &entry.name {
                    write!(output, " (name {name:?})")?;
                }
                write!(output, " (qualified-name {:?}) ", entry.qualified_name)?;
                write_location(output, &entry.location)?;
                write!(output, " (declaration ")?;
                write_range(output, entry.declaration_range)?;
                writeln!(output, "))")?;
            }
        }
        _ => writeln!(output, "    (status {})", outcome_status(outcome))?,
    }
    writeln!(output, "  )")
}

fn write_range(output: &mut dyn fmt::Write, range: TextRange) -> fmt::Result {
    write!(
        output,
        "(range (start {} {}) (end {} {}))",
        range.start.line, range.start.character, range.end.line, range.end.character
    )
}

fn write_location(output: &mut dyn fmt::Write, location: &SourceLocation) -> fmt::Result {
    write!(output, "(location (document {:?}) ", location.document)?;
    write_range(output, location.range)?;
    write!(output, " (role {:?}))", location.role)
}

fn write_target(output: &mut dyn fmt::Write, target: &NavigationTarget) -> fmt::Result {
    write!(output, "(candidate (name {:?}) ", target.name)?;
    write_location(output, &target.location)?;
    write!(output, ")")
}

fn write_target_outcome(
    output: &mut dyn fmt::Write,
    label: &str,
    outcome: &QueryOutcome<NavigationTarget>,
) -> fmt::Result {
    write!(output, "    ({label} ")?;
    match outcome {
        QueryOutcome::Resolved(target) => {
            write!(output, "(status resolved) ")?;
            write_target(output, target)?;
        }
        QueryOutcome::Recovered(target) => {
            write!(output, "(status recovery) ")?;
            write_target(output, target)?;
        }
        QueryOutcome::UnsupportedWith(target) => {
            write!(output, "(status unsupported) ")?;
            write_target(output, target)?;
        }
        QueryOutcome::Ambiguous(targets) => {
            write!(output, "(status ambiguous) (candidates")?;
            for target in targets {
                write!(output, " ")?;
                write_target(output, target)?;
            }
            write!(output, ")")?;
        }
        QueryOutcome::Unresolved => write!(output, "(status unresolved)")?,
        QueryOutcome::Unsupported => write!(output, "(status unsupported)")?,
        QueryOutcome::Recovery => write!(output, "(status recovery)")?,
        QueryOutcome::Incomplete => write!(output, "(status incomplete)")?,
    }
    writeln!(output, ")")
}

fn write_locations_outcome(
    output: &mut dyn fmt::Write,
    label: &str,
    outcome: &QueryOutcome<Box<[SourceLocation]>>,
) -> fmt::Result {
    write!(output, "    ({label} ")?;
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => {
            write!(output, "(locations")?;
            for value in values.iter() {
                write!(output, " ")?;
                write_location(output, value)?;
            }
            write!(output, ")")?;
        }
        _ => write!(output, "(status unavailable)")?,
    }
    writeln!(output, ")")
}

fn write_rename_outcome(output: &mut dyn fmt::Write, outcome: &RenameOutcome) -> fmt::Result {
    write!(output, "    (rename ")?;
    match outcome {
        RenameOutcome::Ready {
            name,
            range,
            occurrences,
            ..
        } => {
            write!(output, "(status ready) (name {name:?}) ")?;
            write_range(output, *range)?;
            write!(output, " (occurrences {})", occurrences.len())?;
        }
        RenameOutcome::Collision(targets) => {
            write!(output, "(status collision) (candidates")?;
            for target in targets.iter() {
                write!(output, " ")?;
                write_target(output, target)?;
            }
            write!(output, ")")?;
        }
        // No trailing `)` here: the shared `writeln!` below closes `(rename` for every arm.
        RenameOutcome::Ambiguous(targets) => {
            write!(output, "(status ambiguous) (candidates {})", targets.len())?
        }
        RenameOutcome::InvalidName => write!(output, "(status invalid-name)")?,
        RenameOutcome::Unresolved => write!(output, "(status unresolved)")?,
        RenameOutcome::Unsupported => write!(output, "(status unsupported)")?,
        RenameOutcome::Recovery => write!(output, "(status recovery)")?,
        RenameOutcome::Incomplete => write!(output, "(status incomplete)")?,
    }
    writeln!(output, ")")
}

fn write_members_outcome(
    output: &mut dyn fmt::Write,
    outcome: &QueryOutcome<Box<[VisibleMember]>>,
) -> fmt::Result {
    write!(output, "    (visible-members ")?;
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => {
            write!(output, "(candidates")?;
            for value in values.iter() {
                write!(
                    output,
                    " (member (name {:?}) (qualified-name {:?}) (kind {:?})",
                    value.name,
                    value.qualified_name,
                    value.kind.as_str()
                )?;
                if let Some(role) = value.role {
                    write!(output, " (role {:?})", role.as_str())?;
                }
                write!(output, ")")?;
            }
            write!(output, ")")?;
        }
        _ => write!(output, "(status unavailable)")?,
    }
    writeln!(output, ")")
}

fn write_inspection_at_outcome(
    output: &mut dyn fmt::Write,
    outcome: &QueryOutcome<ElementInspectionAt>,
) -> fmt::Result {
    writeln!(output, "    (inspection")?;
    match outcome {
        QueryOutcome::Resolved(at)
        | QueryOutcome::Recovered(at)
        | QueryOutcome::UnsupportedWith(at) => {
            writeln!(output, "      (status {})", outcome_status(outcome))?;
            match &at.containing {
                Some(containing) => {
                    writeln!(output, "      (containing")?;
                    write_element(output, "        ", containing)?;
                    writeln!(output, "      )")?;
                }
                None => writeln!(output, "      (containing (status none))")?,
            }
            write_reference_at(output, &at.referenced)?;
        }
        _ => writeln!(output, "      (status {})", outcome_status(outcome))?,
    }
    writeln!(output, "    )")
}

fn outcome_status<T>(outcome: &QueryOutcome<T>) -> &'static str {
    match outcome {
        QueryOutcome::Resolved(_) => "resolved",
        QueryOutcome::Recovered(_) | QueryOutcome::Recovery => "recovery",
        QueryOutcome::UnsupportedWith(_) | QueryOutcome::Unsupported => "unsupported",
        QueryOutcome::Ambiguous(_) => "ambiguous",
        QueryOutcome::Unresolved => "unresolved",
        QueryOutcome::Incomplete => "incomplete",
    }
}

fn write_reference_at(output: &mut dyn fmt::Write, referenced: &ReferenceAt) -> fmt::Result {
    match referenced {
        ReferenceAt::None => writeln!(output, "      (referenced (status none))"),
        ReferenceAt::Unresolved => writeln!(output, "      (referenced (status unresolved))"),
        ReferenceAt::Unsupported => writeln!(output, "      (referenced (status unsupported))"),
        ReferenceAt::Incomplete => writeln!(output, "      (referenced (status incomplete))"),
        ReferenceAt::Resolved(inspection) => {
            writeln!(output, "      (referenced (status resolved)")?;
            write_element(output, "        ", inspection)?;
            writeln!(output, "      )")
        }
        ReferenceAt::Ambiguous(candidates) => {
            writeln!(output, "      (referenced (status ambiguous)")?;
            for candidate in candidates.iter() {
                write_element(output, "        ", candidate)?;
            }
            writeln!(output, "      )")
        }
    }
}

/// One element's published facts.
///
/// Absent facts are omitted rather than rendered as an empty form, so a snapshot diff that gains a
/// line is a fact that started being published, not a formatting change.
fn write_element(
    output: &mut dyn fmt::Write,
    indent: &str,
    inspection: &ElementInspection,
) -> fmt::Result {
    writeln!(
        output,
        "{indent}(element (kind {:?})",
        inspection.kind.as_str()
    )?;
    if let Some(role) = inspection.role {
        writeln!(output, "{indent}  (role {:?})", role.as_str())?;
    }
    if let Some(name) = &inspection.name {
        writeln!(output, "{indent}  (name {name:?})")?;
    }
    if let Some(short_name) = &inspection.short_name {
        writeln!(output, "{indent}  (short-name {short_name:?})")?;
    }
    writeln!(
        output,
        "{indent}  (qualified-name {:?})",
        inspection.qualified_name
    )?;
    write!(output, "{indent}  ")?;
    write_location(output, &inspection.location)?;
    writeln!(output)?;
    write!(output, "{indent}  (declaration ")?;
    write_range(output, inspection.declaration_range)?;
    writeln!(output, ")")?;
    let membership = inspection.membership;
    writeln!(
        output,
        "{indent}  (membership (kind {}) (visibility {}) (provenance {}))",
        membership_kind_name(membership.kind),
        visibility_name(membership.visibility),
        visibility_provenance_name(membership.provenance)
    )?;
    write_multiplicity(output, indent, inspection.multiplicity)?;
    if !inspection.modifiers.is_empty() {
        write!(output, "{indent}  (modifiers")?;
        for modifier in inspection.modifiers.iter() {
            write!(output, " {:?}", modifier.as_str())?;
        }
        writeln!(output, ")")?;
    }
    if let Some(portion) = inspection.portion_kind {
        writeln!(output, "{indent}  (portion {})", portion_name(portion))?;
    }
    if let Some(direction) = inspection.direction {
        writeln!(
            output,
            "{indent}  (direction {})",
            direction_name(direction)
        )?;
    }
    if let Some(value) = inspection.value {
        writeln!(
            output,
            "{indent}  (value (kind {}) (default {}) (operator {}))",
            value_kind_name(value.kind),
            value.is_default,
            value.has_operator
        )?;
    }
    if inspection.evaluation != EvaluationState::NotApplicable {
        write!(output, "{indent}  (evaluation {}", inspection.evaluation)?;
        if let Some(scalar) = inspection.evaluation.value() {
            write!(output, " ")?;
            write_scalar(output, scalar)?;
        }
        writeln!(output, ")")?;
    }
    for documentation in inspection.documentation.iter() {
        write!(
            output,
            "{indent}  (documentation (form {})",
            annotation_form_name(documentation.form)
        )?;
        if let Some(locale) = &documentation.locale {
            write!(output, " (locale {locale:?})")?;
        }
        if let Some(language) = &documentation.language {
            write!(output, " (language {language:?})")?;
        }
        writeln!(output, " (text {:?}))", documentation.text)?;
    }
    for relationship in inspection.relationships.iter() {
        write_relationship(output, indent, relationship)?;
    }
    writeln!(output, "{indent})")
}

fn write_relationship(
    output: &mut dyn fmt::Write,
    indent: &str,
    relationship: &ElementRelationship,
) -> fmt::Result {
    write!(
        output,
        "{indent}  (relationship (kind {:?}) (provenance {})",
        relationship.kind,
        match relationship.provenance {
            RelationshipProvenance::Authored => "authored",
            RelationshipProvenance::Implied => "implied",
        }
    )?;
    if let Some(authored) = &relationship.authored {
        write!(output, " (authored {authored:?})")?;
    }
    match &relationship.target {
        RelationshipTarget::Resolved(_) => write!(output, " (target resolved)")?,
        RelationshipTarget::Ambiguous(candidates) => {
            write!(output, " (target ambiguous {})", candidates.len())?
        }
        RelationshipTarget::Unresolved => write!(output, " (target unresolved)")?,
        RelationshipTarget::Unsupported => write!(output, " (target unsupported)")?,
    }
    writeln!(output, ")")
}

fn write_multiplicity(
    output: &mut dyn fmt::Write,
    indent: &str,
    multiplicity: MultiplicityFacts,
) -> fmt::Result {
    match multiplicity {
        // Absence is the common case, and printing it on every element would bury the declared
        // ones. `[*]` still prints, because writing it is not the same as omitting it.
        MultiplicityFacts::Absent => Ok(()),
        MultiplicityFacts::Declared {
            lower,
            upper,
            ordered,
            nonunique,
        } => {
            write!(output, "{indent}  (multiplicity (lower ")?;
            write_bound(output, lower)?;
            write!(output, ") (upper ")?;
            write_bound(output, upper)?;
            writeln!(output, ") (ordered {ordered}) (nonunique {nonunique}))")
        }
    }
}

fn write_bound(output: &mut dyn fmt::Write, bound: MultiplicityBound) -> fmt::Result {
    match bound {
        MultiplicityBound::Unbounded => write!(output, "unbounded"),
        MultiplicityBound::Literal(value) => write!(output, "{value}"),
        MultiplicityBound::Expression => write!(output, "expression"),
    }
}

fn write_scalar(output: &mut dyn fmt::Write, scalar: &EvaluatedScalar) -> fmt::Result {
    match scalar {
        EvaluatedScalar::Boolean(value) => write!(output, "{value}"),
        EvaluatedScalar::Integer(value) => write!(output, "{value}"),
        EvaluatedScalar::Real(value) => write!(output, "{value}"),
        EvaluatedScalar::String(value) => write!(output, "{value:?}"),
        EvaluatedScalar::Quantity { magnitude, unit } => {
            write!(output, "(quantity ")?;
            write_scalar(output, magnitude)?;
            write!(output, " {unit:?})")
        }
    }
}

fn membership_kind_name(kind: MembershipKind) -> &'static str {
    match kind {
        MembershipKind::Owning => "owning",
        MembershipKind::Feature => "feature",
        MembershipKind::Import => "import",
        MembershipKind::Alias => "alias",
    }
}

fn visibility_name(visibility: Visibility) -> &'static str {
    match visibility {
        Visibility::Public => "public",
        Visibility::Private => "private",
        Visibility::Protected => "protected",
    }
}

fn visibility_provenance_name(provenance: VisibilityProvenance) -> &'static str {
    match provenance {
        VisibilityProvenance::Authored => "authored",
        VisibilityProvenance::Default => "default",
    }
}

fn portion_name(portion: PortionKind) -> &'static str {
    match portion {
        PortionKind::Snapshot => "snapshot",
        PortionKind::Timeslice => "timeslice",
    }
}

fn direction_name(direction: FeatureDirection) -> &'static str {
    match direction {
        FeatureDirection::In => "in",
        FeatureDirection::Out => "out",
        FeatureDirection::InOut => "inout",
    }
}

fn value_kind_name(kind: ValueKind) -> &'static str {
    match kind {
        ValueKind::Bind => "bind",
        ValueKind::Assign => "assign",
    }
}

fn annotation_form_name(form: AnnotationForm) -> &'static str {
    match form {
        AnnotationForm::Documentation => "doc",
        AnnotationForm::Comment => "comment",
        AnnotationForm::TextualRepresentation => "rep",
    }
}

/// Storage and implementation models are not part of this facade.
///
/// ```compile_fail
/// use sysml_query::resolved_slice::{ResolutionResults, SemanticModelStorage};
/// ```
pub struct RawStorageIsNotPublic;

#[cfg(test)]
mod tests {
    use super::{
        build, build_measured, BuildRequest, ConstructionStrategy, PublishedModel, SourceDocument,
        SourceKind,
    };

    #[test]
    fn immutable_publication_can_be_shared_by_async_hosts() {
        fn requires_send_sync<T: Send + Sync>() {}
        requires_send_sync::<PublishedModel>();
    }

    #[test]
    fn measured_and_unmeasured_builds_publish_the_same_semantics() {
        fn request() -> BuildRequest {
            BuildRequest::resolved(
                vec![SourceDocument::from_memory_path(
                    "measurement-parity",
                    "model.sysml",
                    "package P { part def Vehicle; part vehicle : Vehicle; }".into(),
                    SourceKind::Workspace,
                )
                .unwrap()],
                ConstructionStrategy::Sequential,
            )
            .unwrap()
        }

        let ordinary = build(request()).unwrap();
        let (measured, _) = build_measured(request()).unwrap();
        let mut ordinary_output = String::new();
        let mut measured_output = String::new();
        ordinary
            .debug()
            .write_semantic_sexpr(&mut ordinary_output)
            .unwrap();
        measured
            .debug()
            .write_semantic_sexpr(&mut measured_output)
            .unwrap();
        assert_eq!(ordinary_output, measured_output);
    }
}

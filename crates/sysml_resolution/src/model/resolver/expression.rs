//! Typed unit and measurement facts over the expressions this publication admitted.
//!
//! # Why a unit is a declaration and a dimension is a type
//!
//! SysML declares units as ordinary model elements: `attribute <kg> kilogram : MassUnit;` in the
//! `SI` library is an attribute usage typed by `ISQ::MassUnit`, which specialises
//! `MeasurementReferences::MeasurementUnit`. Nothing about that is spelling. A declaration is a
//! unit exactly when its type conforms to `MeasurementUnit`, its symbols are the short name and
//! name it was declared with, and its dimension is the measurement-reference type it is an
//! instance of -- a `DeclarationId`, compared by the same specialization closure every other type
//! query uses.
//!
//! The legacy registry instead decided unit-ness with `name.ends_with("Unit")`, recovered a
//! quantity's expected dimension by rewriting the type name's `Value` suffix to `Unit`, and
//! compared dimensions as strings with a hand-maintained alias table. Every one of those is an
//! inference from a name, and each fails on a library that names things differently and succeeds
//! on a user type that happens to end in `Unit`.
//!
//! # The one thing that is looked up by name
//!
//! Which declaration *is* `MeasurementUnit` cannot be derived; it is a normative identity the
//! standard library owns, exactly as the OMG Pilot resolves its implicit generalizations by
//! qualified library name. [`LibraryAnchors`] resolves that closed set of paths once, against
//! admitted library documents only, and an anchor that is absent or ambiguous stays absent --
//! every rule reading one publishes an explicit "not admitted" outcome rather than guessing.
//!
//! # What the parser cannot give us
//!
//! A unit token is opaque text, not a reference: units may contain operators (`m/s^2`), so the
//! parser hands the bracketed text over verbatim. A token that is a name is resolved through the
//! catalog; one that is not is published as
//! [`UnitResolution::UnsupportedExpression`](crate::UnitResolution::UnsupportedExpression). This
//! layer does not reimplement a unit-expression grammar over text the parser declined to model.
//!
//! The same gap decides the catalog's scope. An ordinary reference resolves through the scopes
//! visible where it was written; a unit token is not a reference, carries no scope, and cannot be
//! resolved through the import machinery without the parser first modelling it as one. The catalog
//! is therefore publication-wide: a token names the units the whole publication declares, and a
//! symbol two of them answer to is reported as ambiguous rather than decided by proximity. That is
//! visible behaviour, not an accident -- see `test/snapshots/resolution/ambiguous_unit_symbol.md`.

use super::*;
use crate::evaluation::EvaluatedScalar;

/// A normative standard-library declaration this fact family is rooted in.
///
/// `None` means "no admitted library document declares this path, or several do". It is never a
/// silent pass: every reader turns it into an explicit published outcome -- an unresolvable unit
/// catalog, or a measurement requirement that does not apply -- so a workspace built without the
/// measurement libraries reports that it has none, not that its units are wrong.
type Anchor = Option<DeclarationId>;

/// `MeasurementReferences::MeasurementUnit`: the root every measurement unit's type conforms to.
const MEASUREMENT_UNIT_PATH: &[&str] = &["MeasurementReferences", "MeasurementUnit"];
/// `Quantities::TensorQuantityValue`: the root every quantity value's type conforms to.
const QUANTITY_VALUE_PATH: &[&str] = &["Quantities", "TensorQuantityValue"];
/// `Quantities::TensorQuantityValue::mRef`: the feature every quantity value redefines to state
/// which measurement reference its values are expressed in.
const MEASUREMENT_REFERENCE_PATH: &[&str] = &["Quantities", "TensorQuantityValue", "mRef"];
/// The KerML scalar datatypes a literal value has.
const BOOLEAN_PATH: &[&str] = &["ScalarValues", "Boolean"];
const STRING_PATH: &[&str] = &["ScalarValues", "String"];
const INTEGER_PATH: &[&str] = &["ScalarValues", "Integer"];
const REAL_PATH: &[&str] = &["ScalarValues", "Real"];

/// The standard-library declarations the unit and value rules are rooted in.
#[derive(Debug, Default)]
pub(super) struct LibraryAnchors {
    measurement_unit: Anchor,
    quantity_value: Anchor,
    measurement_reference: Anchor,
    boolean: Anchor,
    string: Anchor,
    integer: Anchor,
    real: Anchor,
}

impl LibraryAnchors {
    fn build(storage: &SemanticModelStorage) -> Self {
        Self {
            measurement_unit: find_anchor(storage, MEASUREMENT_UNIT_PATH),
            quantity_value: find_anchor(storage, QUANTITY_VALUE_PATH),
            measurement_reference: find_anchor(storage, MEASUREMENT_REFERENCE_PATH),
            boolean: find_anchor(storage, BOOLEAN_PATH),
            string: find_anchor(storage, STRING_PATH),
            integer: find_anchor(storage, INTEGER_PATH),
            real: find_anchor(storage, REAL_PATH),
        }
    }

    /// The scalar datatype a settled literal value has, or `None` when the library declaring it is
    /// not admitted.
    ///
    /// A quantity deliberately has none: its type is the quantity value definition its unit
    /// belongs to, not the datatype of its magnitude, and answering `Integer` for `10 [kg]` would
    /// make every mass look like a mistyped integer.
    fn scalar_type(&self, value: &EvaluatedScalar) -> Option<DeclarationId> {
        match value {
            EvaluatedScalar::Boolean(_) => self.boolean,
            EvaluatedScalar::String(_) => self.string,
            EvaluatedScalar::Integer(_) => self.integer,
            EvaluatedScalar::Real(_) => self.real,
            EvaluatedScalar::Quantity { .. } => None,
        }
    }
}

/// Resolves one qualified library path to the single declaration that owns it.
///
/// Only library-role documents are searched: the anchors are normative library identities, and a
/// workspace package that happens to be called `Quantities` must not become the root of the unit
/// system. Several matches leave the anchor absent for the same reason an ambiguous reference
/// resolves to nothing -- choosing one would be a guess.
fn find_anchor(storage: &SemanticModelStorage, path: &[&str]) -> Anchor {
    let (last, owners) = path.split_last()?;
    let mut found = None;
    for index in 0..storage.declarations.len() {
        let declaration = &storage.declarations[index];
        if storage
            .document(declaration.document)
            .is_none_or(|document| document.role == SourceRole::Workspace)
        {
            continue;
        }
        if !declaration
            .name
            .is_some_and(|name| storage.symbol(name) == Some(*last))
        {
            continue;
        }
        if !owner_path_matches(storage, declaration.owner, owners) {
            continue;
        }
        let id = match DeclarationId::from_index(index) {
            Ok(id) => id,
            Err(_) => return None,
        };
        if found.is_some() {
            return None;
        }
        found = Some(id);
    }
    found
}

/// Whether `owner`'s chain of named ancestors spells `expected`, outermost first, and stops there.
fn owner_path_matches(
    storage: &SemanticModelStorage,
    owner: Option<DeclarationId>,
    expected: &[&str],
) -> bool {
    let mut cursor = owner;
    for name in expected.iter().rev() {
        let Some(current) = cursor else {
            return false;
        };
        let Some(declaration) = storage.declaration(current) else {
            return false;
        };
        if !declaration
            .name
            .is_some_and(|id| storage.symbol(id) == Some(*name))
        {
            return false;
        }
        cursor = declaration.owner;
    }
    cursor.is_none()
}

/// What the publication settled for one authored unit token.
///
/// The internal mirror of [`crate::UnitResolution`], carrying declarations where the published
/// contract carries identities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum UnitOutcome {
    Resolved {
        unit: DeclarationId,
        /// The measurement-reference types the unit is an instance of, in canonical order.
        dimensions: Box<[DeclarationId]>,
    },
    UnknownSymbol,
    Ambiguous(Box<[DeclarationId]>),
    UnsupportedExpression,
    CatalogUnavailable,
}

/// One authored unit token with the outcome the barrier settled for it.
#[derive(Debug, Clone)]
pub(super) struct SettledUnit {
    pub(super) declaration: DeclarationId,
    pub(super) document: DocumentId,
    pub(super) ordinal: u32,
    pub(super) text: SymbolId,
    pub(super) span: Span,
    pub(super) outcome: UnitOutcome,
}

/// The measurement reference a declaration's type requires of its values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum RequiredMeasurement {
    /// The declaration is not typed by a quantity value.
    NotApplicable,
    /// It is typed by a quantity value whose measurement-reference feature has no settled type.
    Indeterminate,
    /// Its values must be measured in one of these measurement-reference types.
    Required(Box<[DeclarationId]>),
}

/// One authored `filter` condition and what its expression settled to.
#[derive(Debug, Clone)]
pub(super) struct SettledFilter {
    pub(super) owner: DeclarationId,
    pub(super) document: DocumentId,
    pub(super) form: FilterForm,
    pub(super) span: Span,
    pub(super) state: EvaluationState,
}

/// One authored invocation whose callee settled, with both argument counts.
#[derive(Debug, Clone)]
pub(super) struct SettledInvocation {
    pub(super) declaration: DeclarationId,
    pub(super) document: DocumentId,
    pub(super) span: Span,
    pub(super) callee: DeclarationId,
    /// How many arguments the author wrote at the call site.
    pub(super) supplied: u32,
    /// How many bindable parameters the callee declares: `in`/`inout` parameters with no default
    /// value of their own.
    pub(super) required: u32,
}

/// Every settled expression fact of one publication.
#[derive(Debug, Default)]
pub(crate) struct ExpressionIndex {
    anchors: LibraryAnchors,
    /// Settled unit tokens, sorted by `(declaration, ordinal)`.
    units: Box<[SettledUnit]>,
    /// Contiguous range into `units` per declaration, indexed by declaration ordinal.
    unit_ranges: Box<[(u32, u32)]>,
    /// Each declaration's required measurement reference, indexed by declaration ordinal.
    required: Box<[RequiredMeasurement]>,
    filters: Box<[SettledFilter]>,
    invocations: Box<[SettledInvocation]>,
}

impl ExpressionIndex {
    pub(super) fn build(
        model: &ResolvedSemanticModel,
        filter_states: &[SettledFilterCondition],
    ) -> Result<Self, ResolutionError> {
        let storage = &model.storage;
        let count = storage.declarations.len();
        let anchors = LibraryAnchors::build(storage);
        let catalog = UnitCatalog::build(model, &anchors)?;

        let mut units = storage
            .unit_tokens
            .iter()
            .map(|token| SettledUnit {
                declaration: token.declaration,
                document: token.document,
                ordinal: token.ordinal,
                text: token.text,
                span: token.span.clone(),
                outcome: catalog.resolve(storage, token.text),
            })
            .collect::<Vec<_>>();
        units.sort_by_key(|unit| (unit.declaration, unit.ordinal));
        let mut unit_ranges = Vec::with_capacity(count);
        let mut cursor = 0usize;
        for index in 0..count {
            let start = u32::try_from(cursor).map_err(|_| ResolutionError::Capacity)?;
            while units
                .get(cursor)
                .is_some_and(|unit| unit.declaration.index() == index)
            {
                cursor += 1;
            }
            let end = u32::try_from(cursor).map_err(|_| ResolutionError::Capacity)?;
            unit_ranges.push((start, end));
        }
        // A token owned by a declaration outside the domain would otherwise be silently dropped,
        // leaving an authored unit that exists and can never be read.
        if cursor != units.len() {
            return Err(ResolutionError::InvalidStorage);
        }

        let mut required = Vec::with_capacity(count);
        let measurements = MeasurementReferences::build(model, &anchors)?;
        for index in 0..count {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            required.push(measurements.required_for(model, id));
        }

        // The parallel arrays are produced by one pass over one table, so a length mismatch means
        // the evaluation pass and the lowering disagree about how many conditions exist.
        if filter_states.len() != storage.filter_conditions.len() {
            return Err(ResolutionError::InvalidStorage);
        }
        let filters = storage
            .filter_conditions
            .iter()
            .zip(filter_states)
            .map(|(condition, settled)| SettledFilter {
                owner: condition.owner,
                document: condition.document,
                form: condition.form,
                span: condition.span.clone(),
                state: settled.state.clone(),
            })
            .collect();

        let mut invocations = Vec::new();
        let parameters = bindable_parameter_counts(storage)?;
        for invocation in storage.invocations.iter() {
            let Some(ResolutionStatus::Resolved(callee)) =
                model.resolution.outcome(invocation.callee)
            else {
                continue;
            };
            invocations.push(SettledInvocation {
                declaration: invocation.declaration,
                document: invocation.document,
                span: invocation.span.clone(),
                callee,
                supplied: invocation.argument_count,
                required: parameters.get(callee.index()).copied().unwrap_or_default(),
            });
        }

        Ok(Self {
            anchors,
            units: units.into_boxed_slice(),
            unit_ranges: unit_ranges.into_boxed_slice(),
            required: required.into_boxed_slice(),
            filters,
            invocations: invocations.into_boxed_slice(),
        })
    }

    /// The unit tokens authored in one declaration's expression, in authored order.
    ///
    /// One range lookup and the row it names: the cost is the tokens returned, not the number of
    /// unit tokens the publication holds.
    pub(super) fn units(&self, declaration: DeclarationId) -> &[SettledUnit] {
        let Some(&(start, end)) = self.unit_ranges.get(declaration.index()) else {
            return &[];
        };
        let units = self
            .units
            .get(start as usize..end as usize)
            .unwrap_or_default();
        record_visited_index_entries(units.len().saturating_add(1));
        units
    }

    pub(super) fn required_measurement(&self, declaration: DeclarationId) -> &RequiredMeasurement {
        record_visited_index_entries(1);
        self.required
            .get(declaration.index())
            .unwrap_or(&RequiredMeasurement::NotApplicable)
    }

    /// Every settled unit token in the publication, sorted by `(declaration, ordinal)`.
    pub(super) fn all_units(&self) -> &[SettledUnit] {
        &self.units
    }

    /// The scalar datatype a settled literal value has, when the library declaring it is admitted.
    pub(super) fn scalar_type(&self, value: &EvaluatedScalar) -> Option<DeclarationId> {
        self.anchors.scalar_type(value)
    }

    pub(super) fn filters(&self) -> &[SettledFilter] {
        &self.filters
    }

    pub(super) fn invocations(&self) -> &[SettledInvocation] {
        &self.invocations
    }
}

/// How many of each declaration's parameters an invocation is expected to bind positionally.
///
/// `in` and `inout` parameters that declare no value of their own. An `out` parameter is the
/// callee's result rather than an input, and a parameter with a declared default is bound whether
/// or not the call site supplies it, so counting either would report a call that is complete.
///
/// Counted once for every declaration rather than per invocation: a model with many call sites
/// would otherwise pay a scan of the whole declaration table for each one.
fn bindable_parameter_counts(
    storage: &SemanticModelStorage,
) -> Result<Box<[u32]>, ResolutionError> {
    let mut counts = vec![0u32; storage.declarations.len()];
    let mut has_value = vec![false; storage.declarations.len()];
    for value in storage.feature_values.iter() {
        if let Some(slot) = has_value.get_mut(value.declaration.index()) {
            *slot = true;
        }
    }
    for (index, declaration) in storage.declarations.iter().enumerate() {
        if declaration.kind != DeclarationKind::ParameterUsage {
            continue;
        }
        let Some(owner) = declaration.owner else {
            continue;
        };
        let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        let Some(facts) = storage.declaration_facts(id) else {
            continue;
        };
        if !matches!(
            facts.direction,
            Some(ParameterDirection::In) | Some(ParameterDirection::InOut)
        ) {
            continue;
        }
        if has_value.get(index).copied().unwrap_or(false) {
            continue;
        }
        let slot = counts
            .get_mut(owner.index())
            .ok_or(ResolutionError::InvalidStorage)?;
        *slot = slot.checked_add(1).ok_or(ResolutionError::Capacity)?;
    }
    Ok(counts.into_boxed_slice())
}

/// Every unit declaration this publication admits, keyed by the symbols it was declared with.
#[derive(Debug, Default)]
struct UnitCatalog {
    /// `(symbol, unit)` sorted by symbol, so a token is a binary search rather than a scan.
    by_symbol: Box<[(SymbolId, DeclarationId)]>,
    /// The measurement-reference types of each admitted unit.
    dimensions: std::collections::BTreeMap<DeclarationId, Box<[DeclarationId]>>,
    /// Whether a catalog exists at all: false when the library declaring `MeasurementUnit` is not
    /// admitted, which is a different answer from "the catalog is empty".
    admitted: bool,
}

impl UnitCatalog {
    fn build(
        model: &ResolvedSemanticModel,
        anchors: &LibraryAnchors,
    ) -> Result<Self, ResolutionError> {
        let Some(measurement_unit) = anchors.measurement_unit else {
            return Ok(Self::default());
        };
        let storage = &model.storage;
        let mut by_symbol = Vec::new();
        let mut dimensions = std::collections::BTreeMap::new();
        for index in 0..storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            // The types a unit *declares*. A unit is an instance of a measurement-reference type,
            // so this is featureTyping; a declaration that specialises `MassUnit` is another unit
            // type, not a unit, and must not become one by inheriting its supertype's role.
            let unit_dimensions = model
                .types
                .direct_types(id)
                .iter()
                .map(|(target, _)| *target)
                .filter(|target| conforms(model, *target, measurement_unit))
                .collect::<Vec<_>>();
            if unit_dimensions.is_empty() {
                continue;
            }
            let declaration = storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            // Both spellings a unit answers to. KerML makes a short name an alternative name of the
            // element, and `10 [kg]` and `10 [kilogram]` name the same declaration.
            for symbol in [
                declaration.name,
                storage
                    .declaration_facts(id)
                    .and_then(|facts| facts.short_name),
            ]
            .into_iter()
            .flatten()
            {
                by_symbol.push((symbol, id));
            }
            dimensions.insert(id, unit_dimensions.into_boxed_slice());
        }
        by_symbol.sort_unstable();
        by_symbol.dedup();
        Ok(Self {
            by_symbol: by_symbol.into_boxed_slice(),
            dimensions,
            admitted: true,
        })
    }

    /// What one authored token names.
    fn resolve(&self, storage: &SemanticModelStorage, text: SymbolId) -> UnitOutcome {
        if !self.admitted {
            return UnitOutcome::CatalogUnavailable;
        }
        let Some(authored) = storage.symbol(text) else {
            return UnitOutcome::UnsupportedExpression;
        };
        let Some(segments) = unit_symbol_path(authored) else {
            return UnitOutcome::UnsupportedExpression;
        };
        let Some((last, qualifiers)) = segments.split_last() else {
            return UnitOutcome::UnsupportedExpression;
        };
        let mut candidates = self
            .lookup(storage, last)
            .filter(|unit| {
                storage.declaration(*unit).is_some_and(|declaration| {
                    owner_path_matches_suffix(storage, declaration.owner, qualifiers)
                })
            })
            .collect::<Vec<_>>();
        candidates.sort_unstable();
        candidates.dedup();
        match candidates.len() {
            0 => UnitOutcome::UnknownSymbol,
            1 => {
                let unit = candidates[0];
                UnitOutcome::Resolved {
                    unit,
                    dimensions: self.dimensions.get(&unit).cloned().unwrap_or_default(),
                }
            }
            _ => UnitOutcome::Ambiguous(candidates.into_boxed_slice()),
        }
    }

    fn lookup<'a>(
        &'a self,
        storage: &'a SemanticModelStorage,
        symbol: &'a str,
    ) -> impl Iterator<Item = DeclarationId> + 'a {
        self.by_symbol
            .iter()
            .filter(move |(candidate, _)| storage.symbol(*candidate) == Some(symbol))
            .map(|(_, unit)| *unit)
    }
}

/// Whether `owner`'s named ancestors end with `expected`, outermost segment first.
///
/// A qualified unit token names a path, not necessarily a rooted one: `SI::s` is written from
/// wherever the author is, so its segments qualify the unit rather than address it absolutely.
fn owner_path_matches_suffix(
    storage: &SemanticModelStorage,
    owner: Option<DeclarationId>,
    expected: &[&str],
) -> bool {
    let mut cursor = owner;
    for name in expected.iter().rev() {
        let Some(current) = cursor else {
            return false;
        };
        let Some(declaration) = storage.declaration(current) else {
            return false;
        };
        if !declaration
            .name
            .is_some_and(|id| storage.symbol(id) == Some(*name))
        {
            return false;
        }
        cursor = declaration.owner;
    }
    true
}

/// Decodes an authored unit token into the name path it spells, or `None` when it spells none.
///
/// The parser hands units over as text because they may contain operators, so this is the one
/// place that decides whether a token is a name at all. It accepts what SysML's own name syntax
/// accepts -- `kg`, `SI::s`, and the quoted form `'in'` -- and rejects everything else, which is
/// then published as an unsupported unit expression rather than silently resolved to whichever
/// unit some prefix of it happens to name.
fn unit_symbol_path(text: &str) -> Option<Vec<&str>> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let mut segments = Vec::new();
    for segment in text.split("::") {
        let segment = segment.trim();
        let segment = match segment.strip_prefix('\'') {
            Some(quoted) => quoted.strip_suffix('\'')?,
            None => segment,
        };
        if segment.is_empty() {
            return None;
        }
        // A quoted name may contain anything but a quote; an unquoted one is an identifier.
        if !text.contains('\'')
            && !segment
                .chars()
                .all(|character| character.is_alphanumeric() || character == '_')
        {
            return None;
        }
        segments.push(segment);
    }
    Some(segments)
}

/// Whether `specific` is `general` or specializes it, in any specialization scope.
pub(super) fn conforms(
    model: &ResolvedSemanticModel,
    specific: DeclarationId,
    general: DeclarationId,
) -> bool {
    specific == general
        || model.types.specialization().reaches(
            specific,
            general,
            types::SpecializationScope::AnySpecialization,
        )
}

/// Which measurement reference each quantity value definition requires.
///
/// Built from the feature every quantity value redefines to state it: `Quantities::
/// TensorQuantityValue::mRef`. A definition states its own by redefining that feature with a
/// narrower type -- `attribute def MassValue :> ScalarQuantityValue { attribute :>> mRef: MassUnit; }`
/// -- so the requirement of a type is the type of the most specific such redefinition it or its
/// supertypes declare.
#[derive(Debug, Default)]
struct MeasurementReferences {
    /// The root every quantity value's type conforms to, absent when its library is not admitted.
    quantity_value: Anchor,
    /// The measurement-reference feature each type declares, if it declares one.
    declared: std::collections::BTreeMap<DeclarationId, DeclarationId>,
}

impl MeasurementReferences {
    fn build(
        model: &ResolvedSemanticModel,
        anchors: &LibraryAnchors,
    ) -> Result<Self, ResolutionError> {
        let Some(root) = anchors.measurement_reference else {
            return Ok(Self::default());
        };
        let quantity_value = anchors.quantity_value;
        let storage = &model.storage;
        let mut declared = std::collections::BTreeMap::new();
        for index in 0..storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            // Redefinition and subsetting are how one feature specialises another; a feature that
            // reaches the root along that chain is the same feature, restated.
            if id != root
                && !model.types.specialization().reaches(
                    id,
                    root,
                    types::SpecializationScope::FeatureSpecialization,
                )
            {
                continue;
            }
            let Some(owner) = model.types.featuring_type(id) else {
                continue;
            };
            declared.insert(owner, id);
        }
        Ok(Self {
            quantity_value,
            declared,
        })
    }

    /// What one declaration's own type requires of its values.
    fn required_for(
        &self,
        model: &ResolvedSemanticModel,
        declaration: DeclarationId,
    ) -> RequiredMeasurement {
        let Some(quantity_value) = self.quantity_value else {
            return RequiredMeasurement::NotApplicable;
        };
        let mut applicable = false;
        let mut candidates = Vec::new();
        for (type_id, _) in model.types.effective_types(declaration) {
            if !conforms(model, *type_id, quantity_value) {
                continue;
            }
            applicable = true;
            candidates.extend(self.statements_for(model, *type_id));
        }
        if !applicable {
            return RequiredMeasurement::NotApplicable;
        }
        // Keep only what the most specific *type* states. `MassValue` states `mRef: MassUnit`,
        // and every quantity-value definition above it states a wider one; a mass is measured in
        // what `MassValue` says, not in anything `TensorQuantityValue` would accept.
        //
        // Specificity is asked of the owning types rather than of the `mRef` features themselves,
        // because a redefinition names whichever inherited `mRef` resolution settles on -- often
        // the outermost one -- so two redefinitions of the same feature need not specialise each
        // other even when their owners plainly do.
        let specific = candidates
            .iter()
            .copied()
            .filter(|(owner, _)| {
                !candidates.iter().any(|(other, _)| {
                    other != owner
                        && model.types.specialization().reaches(
                            *other,
                            *owner,
                            types::SpecializationScope::AnySpecialization,
                        )
                })
            })
            .collect::<Vec<_>>();
        // The types the most specific statement *declares*, not the ones it inherits. A
        // redefinition inherits its predecessors' typings too -- `MassValue::mRef` effectively has
        // `MassUnit` and every measurement reference above it -- and admitting those would make
        // any measurement reference at all satisfy a mass.
        let mut dimensions = specific
            .iter()
            .flat_map(|(_, feature)| model.types.direct_types(*feature))
            .map(|(target, _)| *target)
            .collect::<Vec<_>>();
        dimensions.sort_unstable();
        dimensions.dedup();
        if dimensions.is_empty() {
            return RequiredMeasurement::Indeterminate;
        }
        RequiredMeasurement::Required(dimensions.into_boxed_slice())
    }

    /// The `(type, measurement-reference feature)` statements `type_id` and its supertypes make.
    fn statements_for(
        &self,
        model: &ResolvedSemanticModel,
        type_id: DeclarationId,
    ) -> Vec<(DeclarationId, DeclarationId)> {
        let mut found = Vec::new();
        if let Some(feature) = self.declared.get(&type_id) {
            found.push((type_id, *feature));
        }
        for (ancestor, _) in model.types.specialization().scoped_ancestors(type_id) {
            if let Some(feature) = self.declared.get(&ancestor) {
                found.push((ancestor, *feature));
            }
        }
        found
    }
}

impl ResolvedSemanticModel {
    /// The settled evaluation of one element: its state, its authored units, and what its type
    /// requires of them.
    ///
    /// Three indexed lookups and the rows they name -- no traversal, no re-resolution, and no
    /// evaluation. Every answer was decided before this publication became visible, so repeating
    /// the call, or asking inspection first, cannot change it.
    pub(crate) fn evaluate(&self, symbol: &SymbolIdentity) -> QueryOutcome<ElementEvaluation> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(element) = self.symbol_identity(declaration) else {
            return QueryOutcome::Unresolved;
        };
        let units = self
            .expressions
            .units(declaration)
            .iter()
            .map(|unit| self.published_unit(unit))
            .collect::<Vec<_>>();
        self.resolved_outcome(ElementEvaluation {
            element,
            state: self.evaluation_for(declaration),
            units: units.into_boxed_slice(),
            expected_measurement: self.published_measurement(declaration),
        })
    }

    fn published_unit(&self, unit: &SettledUnit) -> AuthoredUnit {
        AuthoredUnit {
            authored: self.storage.symbol(unit.text).unwrap_or_default().into(),
            location: SourceLocation {
                document: writer::document_identity(self, unit.document).into(),
                range: document_range(&self.storage, unit.document, &unit.span).unwrap_or(
                    TextRange {
                        start: TextPosition {
                            line: 0,
                            character: 0,
                        },
                        end: TextPosition {
                            line: 0,
                            character: 0,
                        },
                    },
                ),
                // A unit token names a declaration, so it occupies the same role as any other
                // reference occurrence.
                role: OccurrenceRole::Reference,
            },
            resolution: match &unit.outcome {
                UnitOutcome::Resolved { unit, dimensions } => match self.symbol_identity(*unit) {
                    Some(identity) => UnitResolution::Resolved(ResolvedUnit {
                        unit: identity,
                        dimensions: self.symbols(dimensions.iter().copied()),
                    }),
                    None => UnitResolution::UnknownSymbol,
                },
                UnitOutcome::UnknownSymbol => UnitResolution::UnknownSymbol,
                UnitOutcome::Ambiguous(candidates) => {
                    UnitResolution::Ambiguous(self.symbols(candidates.iter().copied()))
                }
                UnitOutcome::UnsupportedExpression => UnitResolution::UnsupportedExpression,
                UnitOutcome::CatalogUnavailable => UnitResolution::CatalogUnavailable,
            },
        }
    }

    fn published_measurement(&self, declaration: DeclarationId) -> ExpectedMeasurement {
        match self.expressions.required_measurement(declaration) {
            RequiredMeasurement::NotApplicable => ExpectedMeasurement::NotApplicable,
            RequiredMeasurement::Indeterminate => ExpectedMeasurement::Indeterminate,
            RequiredMeasurement::Required(dimensions) => {
                ExpectedMeasurement::Required(self.symbols(dimensions.iter().copied()))
            }
        }
    }
}

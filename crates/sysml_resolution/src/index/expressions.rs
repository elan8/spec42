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
//! visible behaviour, not an accident -- see `tests/snapshots/resolution/ambiguous_unit_symbol.md`.

use crate::diagnose::document_range;
use crate::index::documents::record_visited_index_entries;
use crate::index::types;
use crate::index::types::TypeIndex;
use crate::lower::facts::FilterForm;
use crate::lower::facts::ParameterDirection;
use crate::lower::storage::SemanticModelStorage;
use crate::model::render as writer;
use crate::model::resolver::SemanticModel;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentId;
use crate::model::SymbolId;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::AuthoredUnit;
use crate::ElementEvaluation;
use crate::EvaluationState;
use crate::ExpectedMeasurement;
use crate::OccurrenceRole;
use crate::QueryOutcome;
use crate::ResolvedUnit;
use crate::SourceLocation;
use crate::SymbolIdentity;
use crate::TextPosition;
use crate::TextRange;
use crate::UnitResolution;
use source_identity::SourceRole;
use sysml_v2_parser::ast::Span;

use crate::evaluation::EvaluatedScalar;

/// A normative standard-library declaration this fact family is rooted in.
///
/// `None` means "no admitted library document declares this path, or several do". It is never a
/// silent pass: every reader turns it into an explicit published outcome -- an unresolvable unit
/// catalog, or a measurement requirement that does not apply -- so a workspace built without the
/// measurement libraries reports that it has none, not that its units are wrong.
pub(crate) type Anchor = Option<DeclarationId>;

/// `MeasurementReferences::MeasurementUnit`: the root every measurement unit's type conforms to.
pub(crate) const MEASUREMENT_UNIT_PATH: &[&str] = &["MeasurementReferences", "MeasurementUnit"];
/// `Quantities::TensorQuantityValue`: the root every quantity value's type conforms to.
pub(crate) const QUANTITY_VALUE_PATH: &[&str] = &["Quantities", "TensorQuantityValue"];
/// `Quantities::TensorQuantityValue::mRef`: the feature every quantity value redefines to state
/// which measurement reference its values are expressed in.
pub(crate) const MEASUREMENT_REFERENCE_PATH: &[&str] =
    &["Quantities", "TensorQuantityValue", "mRef"];
/// The KerML scalar datatypes a literal value has.
pub(crate) const BOOLEAN_PATH: &[&str] = &["ScalarValues", "Boolean"];
pub(crate) const STRING_PATH: &[&str] = &["ScalarValues", "String"];
pub(crate) const INTEGER_PATH: &[&str] = &["ScalarValues", "Integer"];
pub(crate) const REAL_PATH: &[&str] = &["ScalarValues", "Real"];

/// The standard-library declarations the unit and value rules are rooted in.
#[derive(Debug, Default)]
pub(crate) struct LibraryAnchors {
    pub(crate) measurement_unit: Anchor,
    pub(crate) quantity_value: Anchor,
    pub(crate) measurement_reference: Anchor,
    pub(crate) boolean: Anchor,
    pub(crate) string: Anchor,
    pub(crate) integer: Anchor,
    pub(crate) real: Anchor,
}

impl LibraryAnchors {
    pub(crate) fn build(storage: &SemanticModelStorage) -> Self {
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
    pub(crate) fn scalar_type(&self, value: &EvaluatedScalar) -> Option<DeclarationId> {
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
pub(crate) fn find_anchor(storage: &SemanticModelStorage, path: &[&str]) -> Anchor {
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
pub(crate) fn owner_path_matches(
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
pub(crate) enum UnitOutcome {
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
pub(crate) struct SettledUnit {
    pub(crate) declaration: DeclarationId,
    pub(crate) document: DocumentId,
    pub(crate) ordinal: u32,
    pub(crate) text: SymbolId,
    pub(crate) span: Span,
    pub(crate) outcome: UnitOutcome,
}

/// The measurement reference a declaration's type requires of its values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RequiredMeasurement {
    /// The declaration is not typed by a quantity value.
    NotApplicable,
    /// It is typed by a quantity value whose measurement-reference feature has no settled type.
    Indeterminate,
    /// Its values must be measured in one of these measurement-reference types.
    Required(Box<[DeclarationId]>),
}

/// One authored `filter` condition and what its expression settled to.
#[derive(Debug, Clone)]
pub(crate) struct SettledFilter {
    pub(crate) owner: DeclarationId,
    pub(crate) document: DocumentId,
    pub(crate) form: FilterForm,
    pub(crate) span: Span,
    pub(crate) state: EvaluationState,
    pub(crate) predicate: crate::lower::facts::FilterPredicate,
}

/// One authored invocation whose callee settled, with both argument counts.
#[derive(Debug, Clone)]
pub(crate) struct SettledInvocation {
    pub(crate) declaration: DeclarationId,
    pub(crate) document: DocumentId,
    pub(crate) span: Span,
    pub(crate) callee: DeclarationId,
    /// How many arguments the author wrote at the call site.
    pub(crate) supplied: u32,
    /// How many bindable parameters the callee declares: `in`/`inout` parameters with no default
    /// value of their own.
    pub(crate) required: u32,
}

/// Every settled expression fact of one publication.
#[derive(Debug, Default)]
pub(crate) struct ExpressionIndex {
    pub(crate) anchors: LibraryAnchors,
    /// Settled unit tokens, sorted by `(declaration, ordinal)`.
    pub(crate) units: Box<[SettledUnit]>,
    /// Contiguous range into `units` per declaration, indexed by declaration ordinal.
    pub(crate) unit_ranges: Box<[(u32, u32)]>,
    /// Each declaration's required measurement reference, indexed by declaration ordinal.
    pub(crate) required: Box<[RequiredMeasurement]>,
    pub(crate) filters: Box<[SettledFilter]>,
    pub(crate) filter_ranges: Box<[(u32, u32)]>,
    pub(crate) invocations: Box<[SettledInvocation]>,
}

/// The settled facts expression indexing reads, borrowed from the phase products that own them.
///
/// Phase 6 builds this index before any model value exists, so it names its inputs instead of
/// taking a model whose remaining fields would not yet be settled.
pub(crate) struct ExpressionInputs<'a> {
    pub(crate) storage: &'a SemanticModelStorage,
    pub(crate) resolution: &'a ResolutionResults,
    pub(crate) types: &'a TypeIndex,
}

impl ExpressionIndex {
    /// Assembles the settled expression facts of one publication.
    ///
    /// `filters` is `None` when resolution did not converge: nothing was evaluated, so no filter
    /// condition has an outcome to publish. The authored conditions are still in storage; what is
    /// absent is any claim about what they evaluate to.
    pub(crate) fn build(
        model: &ExpressionInputs<'_>,
        filters: Option<Box<[SettledFilter]>>,
    ) -> Result<Self, ResolutionError> {
        let storage = model.storage;
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

        let mut filters = filters.unwrap_or_default().into_vec();
        filters.sort_by_key(|filter| (filter.owner, filter.span.offset));
        let filter_ranges =
            ranges_for_sorted_owners(count, filters.iter().map(|filter| filter.owner))?;
        Ok(Self {
            anchors,
            units: units.into_boxed_slice(),
            unit_ranges: unit_ranges.into_boxed_slice(),
            required: required.into_boxed_slice(),
            filters: filters.into_boxed_slice(),
            filter_ranges,
            invocations: invocations.into_boxed_slice(),
        })
    }

    /// The unit tokens authored in one declaration's expression, in authored order.
    ///
    /// One range lookup and the row it names: the cost is the tokens returned, not the number of
    /// unit tokens the publication holds.
    pub(crate) fn units(&self, declaration: DeclarationId) -> &[SettledUnit] {
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

    /// Whether this publication admits the library that declares what a quantity value is.
    ///
    /// Without it, [`ExpressionIndex::required_measurement`] answers `NotApplicable` for every
    /// declaration because it has nothing to compare a type against -- which is the absence of an
    /// input, not a decision about the model, and the published contract says so separately.
    pub(crate) fn admits_quantity_values(&self) -> bool {
        self.anchors.quantity_value.is_some()
    }

    pub(crate) fn required_measurement(&self, declaration: DeclarationId) -> &RequiredMeasurement {
        record_visited_index_entries(1);
        self.required
            .get(declaration.index())
            .unwrap_or(&RequiredMeasurement::NotApplicable)
    }

    /// Every settled unit token in the publication, sorted by `(declaration, ordinal)`.
    pub(crate) fn all_units(&self) -> &[SettledUnit] {
        &self.units
    }

    /// The scalar datatype a settled literal value has, when the library declaring it is admitted.
    pub(crate) fn scalar_type(&self, value: &EvaluatedScalar) -> Option<DeclarationId> {
        self.anchors.scalar_type(value)
    }

    pub(crate) fn filters(&self) -> &[SettledFilter] {
        &self.filters
    }

    pub(crate) fn filters_for(&self, owner: DeclarationId) -> &[SettledFilter] {
        let Some(&(start, end)) = self.filter_ranges.get(owner.index()) else {
            return &[];
        };
        self.filters
            .get(start as usize..end as usize)
            .unwrap_or_default()
    }

    pub(crate) fn invocations(&self) -> &[SettledInvocation] {
        &self.invocations
    }
}

pub(crate) fn ranges_for_sorted_owners(
    count: usize,
    owners: impl Iterator<Item = DeclarationId>,
) -> Result<Box<[(u32, u32)]>, ResolutionError> {
    let mut counts = vec![0u32; count];
    for owner in owners {
        let Some(slot) = counts.get_mut(owner.index()) else {
            return Err(ResolutionError::InvalidStorage);
        };
        *slot = slot.checked_add(1).ok_or(ResolutionError::Capacity)?;
    }
    let mut start = 0u32;
    let mut ranges = Vec::with_capacity(count);
    for count in counts {
        let end = start.checked_add(count).ok_or(ResolutionError::Capacity)?;
        ranges.push((start, end));
        start = end;
    }
    Ok(ranges.into_boxed_slice())
}

/// How many of each declaration's parameters an invocation is expected to bind positionally.
///
/// `in` and `inout` parameters that declare no value of their own. An `out` parameter is the
/// callee's result rather than an input, and a parameter with a declared default is bound whether
/// or not the call site supplies it, so counting either would report a call that is complete.
///
/// Counted once for every declaration rather than per invocation: a model with many call sites
/// would otherwise pay a scan of the whole declaration table for each one.
pub(crate) fn bindable_parameter_counts(
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
pub(crate) struct UnitCatalog {
    /// `(symbol, unit)` sorted by symbol, so a token is a binary search rather than a scan.
    pub(crate) by_symbol: Box<[(SymbolId, DeclarationId)]>,
    /// The measurement-reference types of each admitted unit.
    pub(crate) dimensions: std::collections::BTreeMap<DeclarationId, Box<[DeclarationId]>>,
    /// Whether a catalog exists at all: false when the library declaring `MeasurementUnit` is not
    /// admitted, which is a different answer from "the catalog is empty".
    pub(crate) admitted: bool,
}

impl UnitCatalog {
    pub(crate) fn build(
        model: &ExpressionInputs<'_>,
        anchors: &LibraryAnchors,
    ) -> Result<Self, ResolutionError> {
        let Some(measurement_unit) = anchors.measurement_unit else {
            return Ok(Self::default());
        };
        let storage = model.storage;
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
                .filter(|target| conforms(model.types, *target, measurement_unit))
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
    pub(crate) fn resolve(&self, storage: &SemanticModelStorage, text: SymbolId) -> UnitOutcome {
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

    pub(crate) fn lookup<'a>(
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
pub(crate) fn owner_path_matches_suffix(
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
///
/// Quoting is decided per segment, not for the token. `'SI'::m/s` quotes only its first segment,
/// and treating the whole token as quoted would accept `m/s` as an ordinary name and then report
/// it as an unknown *symbol* -- a claim about the catalog, when the truth is that this layer
/// cannot decode the token.
pub(crate) fn unit_symbol_path(text: &str) -> Option<Vec<&str>> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let mut segments = Vec::new();
    for segment in text.split("::") {
        let segment = segment.trim();
        match segment.strip_prefix('\'') {
            // A quoted name is delimited by the quotes and may not contain one, so an unterminated
            // or re-quoted segment spells no name. A token whose quoted segment itself contains
            // `::` is split by it here and fails on the resulting halves, which is the honest
            // answer: recovering it would need a scanner the parser has not given us.
            Some(quoted) => match quoted.strip_suffix('\'') {
                Some(inner) if !inner.is_empty() && !inner.contains('\'') => segments.push(inner),
                _ => return None,
            },
            None => {
                if segment.is_empty()
                    || !segment
                        .chars()
                        .all(|character| character.is_alphanumeric() || character == '_')
                {
                    return None;
                }
                segments.push(segment);
            }
        }
    }
    Some(segments)
}

/// Whether `specific` is `general` or specializes it, in any specialization scope.
pub(crate) fn conforms(types: &TypeIndex, specific: DeclarationId, general: DeclarationId) -> bool {
    specific == general
        || types.specialization().reaches(
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
pub(crate) struct MeasurementReferences {
    /// The root every quantity value's type conforms to, absent when its library is not admitted.
    pub(crate) quantity_value: Anchor,
    /// The measurement-reference feature each type declares, if it declares one.
    pub(crate) declared: std::collections::BTreeMap<DeclarationId, DeclarationId>,
}

impl MeasurementReferences {
    pub(crate) fn build(
        model: &ExpressionInputs<'_>,
        anchors: &LibraryAnchors,
    ) -> Result<Self, ResolutionError> {
        let Some(root) = anchors.measurement_reference else {
            return Ok(Self::default());
        };
        let quantity_value = anchors.quantity_value;
        let storage = model.storage;
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
    pub(crate) fn required_for(
        &self,
        model: &ExpressionInputs<'_>,
        declaration: DeclarationId,
    ) -> RequiredMeasurement {
        let Some(quantity_value) = self.quantity_value else {
            return RequiredMeasurement::NotApplicable;
        };
        let mut applicable = false;
        let mut candidates = Vec::new();
        for (type_id, _) in model.types.effective_types(declaration) {
            if !conforms(model.types, *type_id, quantity_value) {
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
    pub(crate) fn statements_for(
        &self,
        model: &ExpressionInputs<'_>,
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

impl<D> SemanticModel<D> {
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
        match self.element_evaluation(declaration) {
            Some(evaluation) => self.resolved_outcome(evaluation),
            None => QueryOutcome::Unresolved,
        }
    }

    /// The same settled evaluation, addressed by declaration rather than by published identity.
    ///
    /// The cohesive element-details answer already holds the declaration, and routing back through
    /// the identity would make it unanswerable for the one case where two declarations share
    /// one -- which is exactly the case details reports as ambiguous rather than as absent.
    pub(crate) fn element_evaluation(
        &self,
        declaration: DeclarationId,
    ) -> Option<ElementEvaluation> {
        let units = self
            .expressions
            .units(declaration)
            .iter()
            .map(|unit| self.published_unit(unit))
            .collect::<Vec<_>>();
        Some(ElementEvaluation {
            element: self.symbol_identity(declaration)?,
            state: self.evaluation_for(declaration),
            units: units.into_boxed_slice(),
            expected_measurement: self.published_measurement(declaration),
        })
    }

    pub(crate) fn published_unit(&self, unit: &SettledUnit) -> AuthoredUnit {
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

    /// What one declaration's type requires of its values, as the published contract states it.
    ///
    /// Whether an element is quantity-typed can only be answered against the library that declares
    /// what a quantity value is. That is a property of the publication, not of the element, so it
    /// is applied here rather than stored once per declaration: with the anchor missing every
    /// element's answer is the same, and it is "unknown", not "no".
    pub(crate) fn published_measurement(&self, declaration: DeclarationId) -> ExpectedMeasurement {
        if !self.expressions.admits_quantity_values() {
            return ExpectedMeasurement::Unavailable;
        }
        match self.expressions.required_measurement(declaration) {
            RequiredMeasurement::NotApplicable => ExpectedMeasurement::NotApplicable,
            RequiredMeasurement::Indeterminate => ExpectedMeasurement::Indeterminate,
            RequiredMeasurement::Required(dimensions) => {
                ExpectedMeasurement::Required(self.symbols(dimensions.iter().copied()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::unit_symbol_path;

    /// A plain identifier and a qualified path are names; both reach the catalog.
    #[test]
    fn a_name_decodes_to_its_segments() {
        assert_eq!(unit_symbol_path("kg"), Some(vec!["kg"]));
        assert_eq!(unit_symbol_path(" SI :: s "), Some(vec!["SI", "s"]));
        assert_eq!(unit_symbol_path("'in'"), Some(vec!["in"]));
        assert_eq!(unit_symbol_path("'SI'::'in'"), Some(vec!["SI", "in"]));
    }

    /// Quoting one segment must not excuse the others.
    ///
    /// `'SI'::m/s` used to be accepted whole -- the quote guard was asked of the token rather than
    /// of each segment -- so `m/s` became a name, and the token was reported as an unknown unit
    /// symbol. That is a claim about the catalog; the truth is that this layer cannot decode it.
    #[test]
    fn a_quoted_segment_does_not_excuse_an_operator_in_another() {
        assert_eq!(unit_symbol_path("'SI'::m/s"), None);
        assert_eq!(unit_symbol_path("SI::m/s"), None);
        assert_eq!(unit_symbol_path("m/s^2"), None);
    }

    /// A quoted name is delimited by its quotes and may not contain one.
    #[test]
    fn a_malformed_quoted_segment_spells_no_name() {
        assert_eq!(unit_symbol_path("'in"), None);
        assert_eq!(unit_symbol_path("'i'n'"), None);
        assert_eq!(unit_symbol_path("''"), None);
        assert_eq!(unit_symbol_path("SI::"), None);
        assert_eq!(unit_symbol_path("   "), None);
    }
}

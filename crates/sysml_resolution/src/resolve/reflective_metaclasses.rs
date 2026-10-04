//! The reflective library metaclass of each published element kind.
//!
//! The KerML and SysML standard libraries carry a reflective model of the abstract syntax
//! (`KerML.kerml`, `SysML.sysml`): one metaclass declaration per metaclass, nested in
//! `KerML::Root`, `KerML::Core`, `KerML::Kernel` or `SysML::Systems`. The Pilot's
//! `ElementUtil.getMetaclassOf` resolves an element's metaclass by looking its abstract-syntax
//! class name up in exactly those four packages. Rules that compare an element's metaclass with a
//! library type (`validateMetadataFeatureAnnotatedElement`) consume this table, settled once per
//! publication beside the other library anchors.
//!
//! The mapping is keyed by the typed [`ElementKind`] -- whose `as_str` is the OMG metaclass name
//! by contract -- and the library declaration's structural owner path, never a display label.
//! A kind with no reflective declaration in the admitted library is `Missing`, so a consumer
//! leaves its question unanswered rather than guessing.

use std::collections::BTreeMap;

use source_identity::SourceRole;
use sysml_contract::ElementKind;

use crate::model::element_kind::element_kind;
use crate::model::DeclarationId;
use crate::resolve::implied::anchor_owner_path_matches;
use crate::resolve::implied::LibrarySpecializationAnchor;

/// The reflective packages, in the Pilot's lookup order.
const REFLECTIVE_PACKAGES: [&[&str]; 4] = [
    &["KerML", "Root"],
    &["KerML", "Core"],
    &["KerML", "Kernel"],
    &["SysML", "Systems"],
];

/// `Metaobjects::Metaobject::annotatedElement`, the feature a metaclass redefines to restrict
/// what its metadata may annotate.
const ANNOTATED_ELEMENT: [&str; 3] = ["Metaobjects", "Metaobject", "annotatedElement"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReflectiveMetaclassAnchors {
    by_kind: BTreeMap<ElementKind, LibrarySpecializationAnchor>,
    /// The settled `Metaobjects::Metaobject::annotatedElement`.
    pub(crate) annotated_element: LibrarySpecializationAnchor,
}

impl Default for ReflectiveMetaclassAnchors {
    fn default() -> Self {
        Self {
            by_kind: BTreeMap::new(),
            annotated_element: LibrarySpecializationAnchor::Missing,
        }
    }
}

impl ReflectiveMetaclassAnchors {
    /// One scan over the admitted standard-library declarations.
    pub(crate) fn resolve(names: &crate::resolve::implied::LibraryAnchorNames<'_>) -> Self {
        let storage = names.storage();
        let mut found = BTreeMap::<ElementKind, Vec<DeclarationId>>::new();
        for (index, declaration) in storage.declarations.iter().enumerate() {
            if !element_kind(declaration.kind).conforms_to(ElementKind::Metaclass) {
                continue;
            }
            if !storage
                .document(declaration.document)
                .is_some_and(|document| document.role == SourceRole::StandardLibrary)
            {
                continue;
            }
            let Some(kind) = declaration
                .name
                .and_then(|name| storage.symbol(name))
                .and_then(ElementKind::parse)
            else {
                continue;
            };
            if !REFLECTIVE_PACKAGES
                .iter()
                .any(|package| anchor_owner_path_matches(storage, declaration.owner, package))
            {
                continue;
            }
            if let Ok(id) = DeclarationId::from_index(index) {
                found.entry(kind).or_default().push(id);
            }
        }
        let by_kind = found
            .into_iter()
            .map(|(kind, mut candidates)| {
                candidates.sort_unstable();
                candidates.dedup();
                let anchor = match candidates.as_slice() {
                    [single] => LibrarySpecializationAnchor::Resolved(*single),
                    _ => LibrarySpecializationAnchor::Ambiguous(candidates.into_boxed_slice()),
                };
                (kind, anchor)
            })
            .collect();
        Self {
            by_kind,
            annotated_element: names.resolve_path(&ANNOTATED_ELEMENT),
        }
    }

    /// The reflective library metaclass of `kind`.
    pub(crate) fn metaclass(&self, kind: ElementKind) -> &LibrarySpecializationAnchor {
        self.by_kind
            .get(&kind)
            .unwrap_or(&LibrarySpecializationAnchor::Missing)
    }
}

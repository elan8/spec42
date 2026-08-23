//! Phase 2 barrier: the immutable product lowering freezes into.

use crate::lower::facts::AuthoredFilterCondition;
use crate::lower::facts::AuthoredInvocation;
use crate::lower::facts::AuthoredReference;
use crate::lower::facts::AuthoredUnitToken;
use crate::lower::facts::CanonicalDocument;
use crate::lower::facts::Declaration;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::DocumentationRecord;
use crate::lower::facts::FeatureValueRecord;
use crate::lower::facts::MembershipRecord;
use crate::lower::facts::PendingEvaluationFact;
use crate::lower::facts::RecoveryRecord;
use crate::lower::facts::UnsupportedRecord;
use crate::lower::intern::SymbolPathArena;
use crate::lower::intern::SymbolTable;
use crate::model::DeclarationId;
use crate::model::DocumentId;
use crate::model::SymbolId;

#[derive(Debug)]
pub(crate) struct SemanticModelStorage {
    pub(crate) documents: Box<[CanonicalDocument]>,
    pub(crate) declarations: Box<[Declaration]>,
    /// Parallel to `declarations`, one entry per `DeclarationId`.
    pub(crate) declaration_facts: Box<[DeclarationFacts]>,
    pub(crate) memberships: Box<[MembershipRecord]>,
    pub(crate) references: Box<[AuthoredReference]>,
    pub(crate) documentation: Box<[DocumentationRecord]>,
    pub(crate) feature_values: Box<[FeatureValueRecord]>,
    pub(crate) unsupported: Box<[UnsupportedRecord]>,
    pub(crate) recovery: Box<[RecoveryRecord]>,
    pub(crate) symbols: SymbolTable,
    pub(crate) paths: SymbolPathArena,
    pub(crate) evaluation_facts: Box<[PendingEvaluationFact]>,
    pub(crate) unit_tokens: Box<[AuthoredUnitToken]>,
    pub(crate) filter_conditions: Box<[AuthoredFilterCondition]>,
    pub(crate) invocations: Box<[AuthoredInvocation]>,
}

impl SemanticModelStorage {
    pub(crate) fn document(&self, id: DocumentId) -> Option<&CanonicalDocument> {
        self.documents.get(id.index())
    }

    pub(crate) fn declaration(&self, id: DeclarationId) -> Option<&Declaration> {
        self.declarations.get(id.index())
    }

    pub(crate) fn declaration_facts(&self, id: DeclarationId) -> Option<&DeclarationFacts> {
        self.declaration_facts.get(id.index())
    }

    pub(crate) fn symbol(&self, id: SymbolId) -> Option<&str> {
        self.symbols.get(id)
    }
}

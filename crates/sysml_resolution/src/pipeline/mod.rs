//! The phase order, made explicit: parse -> lower -> freeze -> resolve.

use std::sync::Arc;
use std::time::Instant;

use source_identity::{ContentDigest, SourceRole};
use sysml_v2_parser::{ParseError, ParsedDocument};

use crate::evaluation::EvaluationPolicy;
use crate::lower::document::LoweredDocument;
use crate::lower::memo::LoweringMemo;
use crate::lower::SemanticModelBuilder;
use crate::model::resolver;
use crate::model::DocumentId;
use crate::pipeline::schedule::{source_admission_rank, BuildPhaseDurations, BuildSchedule};
use crate::resolve::library_seed::SettledLibrary;

pub(crate) mod phase;
pub(crate) mod schedule;

#[derive(Debug, Clone)]
pub(crate) struct OwnedSourceRecord {
    pub(crate) identity: Box<str>,
    pub(crate) role: SourceRole,
    pub(crate) digest: ContentDigest,
    pub(crate) payload: crate::SourcePayload,
    pub(crate) syntax: Option<Arc<crate::syntax::SyntaxAuthority>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoordinatorError {
    DuplicateSourceIdentity,
    ConstructionFailed,
}

/// A library that has already been parsed and solved, ready to be reused by later publications.
///
/// Holds the parsed documents rather than their text, so a workspace build pays neither the
/// library's parse nor its solve. Lowering still runs: it is per-document and cheap, and rerunning
/// it keeps every dense identity assigned by exactly the same code path as an unseeded build.
#[derive(Debug)]
pub(crate) struct PreparedLibrary {
    pub(crate) documents: Vec<PreparedDocument>,
    pub(crate) settled: SettledLibrary,
}

#[derive(Debug)]
pub(crate) struct PreparedDocument {
    pub(crate) identity: Box<str>,
    pub(crate) role: SourceRole,
    pub(crate) digest: ContentDigest,
    pub(crate) parsed: Arc<ParsedDocument>,
    pub(crate) parse_errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SemanticModelBuildCoordinator;

impl SemanticModelBuildCoordinator {
    pub(crate) fn build_measured_with_library(
        mut sources: Vec<OwnedSourceRecord>,
        schedule: BuildSchedule,
        policy: EvaluationPolicy,
        library: Option<&PreparedLibrary>,
        reported: &[Box<str>],
        memo: Option<&LoweringMemo>,
    ) -> Result<
        (
            resolver::ResolvedSemanticModel,
            crate::lower::storage::ParsedSources,
            BuildPhaseDurations,
        ),
        CoordinatorError,
    > {
        // Library sources are ordered ahead of workspace sources so that the dense declaration
        // domain assigns them a contiguous prefix. Rendered output is sorted independently by
        // document identity, so this affects storage order only. Duplicate detection therefore
        // compares identities across the whole set, not within one role.
        sources.sort_unstable_by(|left, right| {
            source_admission_rank(left.role)
                .cmp(&source_admission_rank(right.role))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        let mut identities = sources
            .iter()
            .map(|source| source.identity.as_ref())
            .chain(
                library
                    .into_iter()
                    .flat_map(|library| library.documents.iter())
                    .map(|document| document.identity.as_ref()),
            )
            .collect::<Vec<_>>();
        identities.sort_unstable();
        if identities.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(CoordinatorError::DuplicateSourceIdentity);
        }

        let parse_started = Instant::now();
        let parsed: Vec<(AdmittedSource, bool)> = match schedule {
            BuildSchedule::Sequential => sources
                .into_iter()
                .map(Self::parse_source)
                .collect::<Result<Vec<_>, _>>()?,
            BuildSchedule::Parallel => {
                use rayon::prelude::*;
                sources
                    .into_par_iter()
                    .map(Self::parse_source)
                    .collect::<Result<Vec<_>, _>>()?
            }
        };
        let parse = parse_started.elapsed();
        // A typed fact, not a timer: how many sources this build had to parse itself.
        let sources_parsed = parsed
            .iter()
            .filter(|(_, parsed_here)| *parsed_here)
            .count();
        let parsed: Vec<AdmittedSource> = parsed.into_iter().map(|(source, _)| source).collect();

        let lowering_started = Instant::now();
        let mut builder = SemanticModelBuilder::default();
        let mut documents = Vec::with_capacity(parsed.len());
        // The prepared library is admitted first and in its own recorded order, so its
        // declarations and references land on exactly the dense prefix its settled outcomes were
        // recorded against.
        for document in library.into_iter().flat_map(|library| &library.documents) {
            let admitted = builder
                .admit_document(
                    document.identity.clone(),
                    document.role,
                    document.digest,
                    Arc::clone(&document.parsed),
                    document.parse_errors.clone(),
                )
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push((admitted, document.digest, Arc::clone(&document.parsed)));
        }
        for (identity, role, digest, tree, errors) in parsed {
            let admitted = builder
                .admit_document(identity, role, digest, Arc::clone(&tree), errors)
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push((admitted, digest, tree));
        }
        // Each document is lowered on its own, in a document-local identity space, and only then
        // relocated into this build's arenas in admission order. The isolation is what makes the
        // product a value the memo can key by content digest; the ordered splice is what keeps
        // every dense identity exactly where an undivided lowering would have put it.
        let generation = memo.map(LoweringMemo::begin);
        let products = lower_documents(&documents, memo, generation, schedule)?;
        // Counted facts, not timers: which documents this build lowered and which it took from
        // the memo. A wall-clock threshold cannot state "this edit lowered exactly one document".
        let documents_reused = products.iter().filter(|(_, reused)| *reused).count();
        let documents_lowered = products.len() - documents_reused;
        for ((document, _, _), (lowered, _)) in documents.iter().zip(products.iter()) {
            builder
                .splice(*document, lowered)
                .map_err(|_| CoordinatorError::ConstructionFailed)?;
        }
        drop(products);
        if let (Some(memo), Some(generation)) = (memo, generation) {
            memo.retain(generation);
        }
        let (storage, sources) = builder.freeze();
        let lowering = lowering_started.elapsed();
        let resolution_started = Instant::now();
        let (model, sources) = phase::build_model(
            storage,
            sources,
            policy,
            library.map(|library| &library.settled),
            reported,
        )
        .map_err(|_| CoordinatorError::ConstructionFailed)?;
        let resolution = resolution_started.elapsed();
        Ok((
            model,
            sources,
            BuildPhaseDurations {
                parse,
                lowering,
                resolution,
                sources_parsed,
                documents_lowered,
                documents_reused,
            },
        ))
    }

    /// A parsed handle is admitted as it is (two reference-count bumps); text is parsed here,
    /// the cold path for stateless callers.
    ///
    /// The returned flag reports whether this source was parsed by the build itself, so a
    /// caller can observe that pre-parsed handles are admitted without a second parse.
    fn parse_source(source: OwnedSourceRecord) -> Result<(AdmittedSource, bool), CoordinatorError> {
        let mut parsed_here = true;
        let (tree, errors) = match source.payload {
            crate::SourcePayload::Parsed(parsed) => {
                parsed_here = false;
                parsed.admission_parts()
            }
            crate::SourcePayload::Pending(document) => match source.syntax {
                Some(syntax) => syntax.parse(&document).admission_parts(),
                None => crate::syntax::ParsedSource::parse_text(
                    document.content().to_owned(),
                    document.digest(),
                )
                .admission_parts(),
            },
            crate::SourcePayload::Text(content) => {
                let result = sysml_v2_parser::parse_for_editor_owned(content);
                (Arc::new(result.document), result.errors)
            }
        };
        Ok((
            (source.identity, source.role, source.digest, tree, errors),
            parsed_here,
        ))
    }
}

/// Obtains each admitted document's lowering product, from the memo where one is held.
///
/// A parallel schedule lowers the misses concurrently: each document's walk touches only its own
/// arenas, so the products are independent, and they are spliced afterwards in admission order.
fn lower_documents(
    documents: &[(DocumentId, ContentDigest, Arc<ParsedDocument>)],
    memo: Option<&LoweringMemo>,
    generation: Option<crate::lower::memo::MemoGeneration>,
    schedule: BuildSchedule,
) -> Result<Vec<(Arc<LoweredDocument>, bool)>, CoordinatorError> {
    let lower_one = |(_, digest, parsed): &(DocumentId, ContentDigest, Arc<ParsedDocument>)| {
        match (memo, generation) {
            (Some(memo), Some(generation)) => memo.lower(*digest, generation, parsed),
            _ => crate::lower::document::lower_document(Arc::clone(parsed))
                .map(|lowered| (Arc::new(lowered), false)),
        }
        .map_err(|_| CoordinatorError::ConstructionFailed)
    };
    match schedule {
        BuildSchedule::Sequential => documents.iter().map(lower_one).collect(),
        BuildSchedule::Parallel => {
            use rayon::prelude::*;
            documents.par_iter().map(lower_one).collect()
        }
    }
}

type AdmittedSource = (
    Box<str>,
    SourceRole,
    ContentDigest,
    Arc<ParsedDocument>,
    Vec<ParseError>,
);

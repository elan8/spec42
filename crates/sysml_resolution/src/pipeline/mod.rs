//! The phase order, made explicit: parse -> lower -> freeze -> resolve.

use std::sync::Arc;
use std::time::Instant;

use source_identity::SourceRole;
use sysml_v2_parser::{ParseError, ParsedDocument};

use crate::evaluation::EvaluationPolicy;
use crate::lower::SemanticModelBuilder;
use crate::model::resolver;
use crate::pipeline::schedule::{source_admission_rank, BuildPhaseDurations, BuildSchedule};
use crate::resolve::library_seed::SettledLibrary;

pub(crate) mod phase;
pub(crate) mod schedule;

#[derive(Debug, Clone)]
pub(crate) struct OwnedSourceRecord {
    pub(crate) identity: Box<str>,
    pub(crate) role: SourceRole,
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
    ) -> Result<(resolver::ResolvedSemanticModel, BuildPhaseDurations), CoordinatorError> {
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
                    Arc::clone(&document.parsed),
                    document.parse_errors.clone(),
                )
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push(admitted);
        }
        for (identity, role, tree, errors) in parsed {
            let document = builder
                .admit_document(identity, role, tree, errors)
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push(document);
        }
        for document in documents {
            builder
                .canonicalize_document(document)
                .map_err(|_| CoordinatorError::ConstructionFailed)?;
        }
        let storage = builder.freeze();
        let lowering = lowering_started.elapsed();
        let resolution_started = Instant::now();
        let model = phase::build_model(
            storage,
            policy,
            library.map(|library| &library.settled),
            reported,
        )
        .map_err(|_| CoordinatorError::ConstructionFailed)?;
        let resolution = resolution_started.elapsed();
        Ok((
            model,
            BuildPhaseDurations {
                parse,
                lowering,
                resolution,
                sources_parsed,
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
        Ok(((source.identity, source.role, tree, errors), parsed_here))
    }
}

type AdmittedSource = (Box<str>, SourceRole, Arc<ParsedDocument>, Vec<ParseError>);

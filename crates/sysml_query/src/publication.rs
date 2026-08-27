//! The publication service: canonical construction of an immutable publication from admitted
//! documents, library-stratum reuse, and the session a long-lived host drives it through.
//!
//! A batch host calls [`PublicationService::publish`]. An editor host keeps a
//! [`PublicationSession`] inside its own state, starts a build with
//! [`PublicationSession::begin_build`], constructs it off its executor with
//! [`PublicationService::construct`], and admits the result at the session's barrier.

use std::sync::Arc;

use sysml_resolution::publication::PublicationAuthority;

use crate::resolved_slice::{BuildMeasurements, PublicationIdentity, PublishedModel};
use crate::source::SourceDocument;
use crate::syntax::SyntaxService;

pub use sysml_resolution::publication::{
    BuildToken, PublicationBuildFailure, PublicationFailureStage, PublicationOutcome,
    PublicationToken, Published, SessionLifecycle,
};

/// The lifecycle and publication barrier of one host session, publishing [`PublishedModel`]s.
pub type PublicationSession = sysml_resolution::publication::Session<PublishedModel>;

impl Published for PublishedModel {
    fn identity(&self) -> &PublicationIdentity {
        self.publication().identity()
    }
}

/// An immutable, dependency-complete request whose identity is known before it is built.
#[derive(Debug)]
pub struct PreparedPublication {
    inner: sysml_resolution::publication::PreparedPublication,
}

impl PreparedPublication {
    pub fn identity(&self) -> &PublicationIdentity {
        self.inner.identity()
    }
}

/// A build a session has started: the token that will admit it, and the request to construct.
#[derive(Debug)]
pub struct SemanticBuild {
    token: BuildToken,
    prepared: PreparedPublication,
}

impl SemanticBuild {
    pub fn new(token: BuildToken, prepared: PreparedPublication) -> Self {
        Self { token, prepared }
    }

    pub fn token(&self) -> &BuildToken {
        &self.token
    }
}

/// A constructed build waiting at the session's barrier.
#[derive(Debug)]
pub struct SemanticBuildCompletion {
    token: BuildToken,
    result: Result<Arc<PublishedModel>, PublicationBuildFailure>,
}

impl SemanticBuildCompletion {
    pub fn token(&self) -> &BuildToken {
        &self.token
    }

    pub fn failure(&self) -> Option<&PublicationBuildFailure> {
        self.result.as_ref().err()
    }

    /// Admit this completion at `session`'s barrier.
    pub fn admit(self, session: &mut PublicationSession) -> PublicationOutcome {
        session.admit(&self.token, self.result)
    }
}

/// Handle on the publication authority. Cheap to clone; all clones share one authority and its
/// library-stratum reuse.
#[derive(Debug, Clone)]
pub struct PublicationService {
    inner: Arc<PublicationAuthority>,
}

impl PublicationService {
    pub fn new(syntax: &SyntaxService) -> Self {
        Self {
            inner: Arc::new(PublicationAuthority::new(Arc::clone(syntax.authority()))),
        }
    }

    /// Publishes the exact admitted source set, reusing a solved library only when its complete
    /// canonical identity is unchanged.
    pub fn publish(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<Arc<PublishedModel>, PublicationBuildFailure> {
        self.inner
            .publish(documents, reported_documents)
            .map(|inner| Arc::new(PublishedModel::from_resolution(inner)))
    }

    /// Publishes through this process's authority and returns measurements captured by the phase
    /// owners. Intended for benchmark/reporting hosts; it has identical semantics to `publish`.
    pub fn publish_measured(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(Arc<PublishedModel>, BuildMeasurements), PublicationBuildFailure> {
        self.inner
            .publish_measured(documents, reported_documents)
            .map(|(inner, measurements)| {
                (
                    Arc::new(PublishedModel::from_resolution(inner)),
                    measurements,
                )
            })
    }

    /// Sequential counterpart used only by determinism tests and controlled benchmark tooling.
    #[doc(hidden)]
    pub fn publish_measured_sequential_for_testing(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<(Arc<PublishedModel>, BuildMeasurements), PublicationBuildFailure> {
        self.inner
            .publish_measured_sequential_for_testing(documents, reported_documents)
            .map(|(inner, measurements)| {
                (
                    Arc::new(PublishedModel::from_resolution(inner)),
                    measurements,
                )
            })
    }

    /// Prepares the canonical request without parsing or building it.
    pub fn prepare(
        &self,
        documents: &[SourceDocument],
        reported_documents: impl IntoIterator<Item = Box<str>>,
    ) -> Result<PreparedPublication, PublicationBuildFailure> {
        self.inner
            .prepare(documents, reported_documents)
            .map(|inner| PreparedPublication { inner })
    }

    /// Performs the expensive construction. Call it off the host's executor; it parses any
    /// document the memo does not hold and resolves the model.
    pub fn construct(&self, build: SemanticBuild) -> SemanticBuildCompletion {
        SemanticBuildCompletion {
            token: build.token,
            result: build
                .prepared
                .inner
                .build()
                .map(|inner| Arc::new(PublishedModel::from_resolution(inner))),
        }
    }

    /// A cold session seeded with an empty publication.
    pub fn session_empty(&self) -> PublicationSession {
        PublicationSession::new(
            self.publish(&[], [])
                .expect("an empty initial semantic publication must be constructible"),
        )
    }

    /// A cold session seeded with a publication the host built beforehand.
    pub fn session_seeded(&self, initial: Arc<PublishedModel>) -> PublicationSession {
        PublicationSession::new(initial)
    }
}

//! Phase 0/1 scheduling policy: how admitted sources are ordered and how the build is run.

use std::time::Duration;

use source_identity::SourceRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuildSchedule {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BuildPhaseDurations {
    pub(crate) parse: Duration,
    pub(crate) lowering: Duration,
    pub(crate) resolution: Duration,
    /// How many admitted sources the build parsed itself. A source admitted as a parsed handle
    /// is not counted: it enters the build as the tree the syntax authority already produced.
    pub(crate) sources_parsed: usize,
    /// How many admitted documents this build lowered itself.
    pub(crate) documents_lowered: usize,
    /// How many admitted documents this build took from the lowering memo unchanged.
    pub(crate) documents_reused: usize,
}

/// Storage order for admitted sources: every library role precedes workspace sources.
///
/// This is a construction-order policy, not a semantic one. It exists so a library-only build and
/// a workspace-plus-library build assign the same declaration ids to the same library
/// declarations, which is the precondition for reusing a solved library stratum.
pub(crate) fn source_admission_rank(role: SourceRole) -> u8 {
    match role {
        SourceRole::StandardLibrary => 0,
        SourceRole::Library => 1,
        SourceRole::External => 2,
        SourceRole::Workspace => 3,
    }
}

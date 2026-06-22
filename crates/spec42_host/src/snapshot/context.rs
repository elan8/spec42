//! Execution context for workspace loads.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::error::{HostResult, Spec42HostError};

/// Cooperative cancellation handle shared across threads.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

/// Optional resource limits enforced during workspace loading.
#[derive(Debug, Clone, Default)]
pub struct HostResourceLimits {
    pub max_documents: Option<usize>,
    pub max_total_bytes: Option<u64>,
    pub max_graph_nodes: Option<usize>,
    pub max_graph_relationships: Option<usize>,
}

/// Pipeline phases reported through optional progress callbacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostPipelinePhase {
    LoadingDocuments,
    BuildingGraph,
    BuildingLanguageWorkspace,
    BuildingViewCatalog,
    CollectingValidation,
    ProjectingModel,
}

/// Host execution context for workspace loading.
#[derive(Clone)]
pub struct HostContext {
    pub cancellation: CancellationToken,
    pub deadline: Option<Instant>,
    pub limits: HostResourceLimits,
    progress: Option<Arc<dyn Fn(HostPipelinePhase) + Send + Sync>>,
}

impl std::fmt::Debug for HostContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HostContext")
            .field("cancellation", &self.cancellation)
            .field("deadline", &self.deadline)
            .field("limits", &self.limits)
            .field("progress", &self.progress.as_ref().map(|_| "Fn(_)"))
            .finish()
    }
}

impl Default for HostContext {
    fn default() -> Self {
        Self {
            cancellation: CancellationToken::new(),
            deadline: None,
            limits: HostResourceLimits::default(),
            progress: None,
        }
    }
}

impl HostContext {
    pub fn with_deadline(mut self, timeout: Duration) -> Self {
        self.deadline = Some(Instant::now() + timeout);
        self
    }

    pub fn with_limits(mut self, limits: HostResourceLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn with_progress(mut self, progress: Arc<dyn Fn(HostPipelinePhase) + Send + Sync>) -> Self {
        self.progress = Some(progress);
        self
    }

    pub(crate) fn check_continue(&self, phase: HostPipelinePhase) -> HostResult<()> {
        if self.cancellation.is_cancelled() {
            return Err(Spec42HostError::cancelled());
        }
        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(Spec42HostError::cancelled());
            }
        }
        if let Some(progress) = &self.progress {
            progress(phase);
        }
        Ok(())
    }

    pub(crate) fn enforce_document_limits(
        &self,
        document_count: usize,
        total_bytes: u64,
    ) -> HostResult<()> {
        if let Some(max_documents) = self.limits.max_documents {
            if document_count > max_documents {
                return Err(Spec42HostError::resource_limit_exceeded(
                    "max_documents",
                    format!(
                        "workspace contains {document_count} documents, limit is {max_documents}"
                    ),
                ));
            }
        }
        if let Some(max_total_bytes) = self.limits.max_total_bytes {
            if total_bytes > max_total_bytes {
                return Err(Spec42HostError::resource_limit_exceeded(
                    "max_total_bytes",
                    format!(
                        "workspace content is {total_bytes} bytes, limit is {max_total_bytes}"
                    ),
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn enforce_graph_limits(
        &self,
        node_count: usize,
        relationship_count: usize,
    ) -> HostResult<()> {
        if let Some(max_graph_nodes) = self.limits.max_graph_nodes {
            if node_count > max_graph_nodes {
                return Err(Spec42HostError::resource_limit_exceeded(
                    "max_graph_nodes",
                    format!("semantic graph has {node_count} nodes, limit is {max_graph_nodes}"),
                ));
            }
        }
        if let Some(max_graph_relationships) = self.limits.max_graph_relationships {
            if relationship_count > max_graph_relationships {
                return Err(Spec42HostError::resource_limit_exceeded(
                    "max_graph_relationships",
                    format!(
                        "semantic graph has {relationship_count} relationships, limit is {max_graph_relationships}"
                    ),
                ));
            }
        }
        Ok(())
    }
}

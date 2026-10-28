//! Host source-discovery and notification epochs, separate from semantic construction.

use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ReadinessToken(Arc<()>);

#[derive(Clone)]
pub(crate) struct DiagnosticReadiness {
    token: ReadinessToken,
    announced: bool,
    initial: bool,
    source_inputs_pending: bool,
}

impl Default for DiagnosticReadiness {
    fn default() -> Self {
        Self {
            token: ReadinessToken(Arc::new(())),
            announced: false,
            initial: true,
            source_inputs_pending: false,
        }
    }
}

impl DiagnosticReadiness {
    pub(crate) fn rekey_for_owner(&mut self) {
        self.token = ReadinessToken(Arc::new(()));
    }

    pub(crate) fn reset(&mut self, initial: bool) {
        *self = Self {
            initial,
            source_inputs_pending: !initial,
            ..Self::default()
        };
    }

    pub(crate) fn source_inputs_pending(&self) -> bool {
        self.source_inputs_pending
    }

    /// Configured library source discovery completes before semantic construction can run.
    pub(crate) fn admit_source_inputs(&mut self, token: &ReadinessToken) -> bool {
        if !self.is_current(token) || !self.source_inputs_pending {
            return false;
        }
        self.source_inputs_pending = false;
        true
    }

    pub(crate) fn token(&self) -> ReadinessToken {
        self.token.clone()
    }

    pub(crate) fn announced(&self) -> bool {
        self.announced
    }

    pub(crate) fn is_current(&self, token: &ReadinessToken) -> bool {
        Arc::ptr_eq(&self.token.0, &token.0)
    }

    pub(crate) fn needs_initial_notification(&self) -> bool {
        self.initial && !self.announced
    }

    pub(crate) fn needs_reindex_notification(&self) -> bool {
        !self.initial && !self.announced && !self.source_inputs_pending
    }

    /// A send completed for this exact project/notification epoch. An older startup or
    /// reindex notification must never unlock diagnostics for its replacement.
    pub(crate) fn announce(&mut self, token: &ReadinessToken) -> bool {
        if !self.is_current(token) || self.source_inputs_pending {
            return false;
        }
        self.announced = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_completion_cannot_unlock_another_epoch_or_project() {
        let mut readiness = DiagnosticReadiness::default();
        let old = readiness.token();
        assert!(!readiness.announced());
        assert!(readiness.announce(&old));
        readiness.reset(false);
        assert!(!readiness.announce(&old));
        assert!(!readiness.announced());
        assert!(!readiness.needs_initial_notification());
        assert!(!readiness.needs_reindex_notification());
        let foreign = DiagnosticReadiness::default().token();
        assert!(!readiness.announce(&foreign));
        assert!(!readiness.announce(&readiness.token()));
        assert!(!readiness.admit_source_inputs(&old));
        assert!(readiness.admit_source_inputs(&readiness.token()));
        assert!(readiness.needs_reindex_notification());
        assert!(readiness.announce(&readiness.token()));
        assert!(readiness.announced());
        assert!(!readiness.needs_reindex_notification());
    }
}

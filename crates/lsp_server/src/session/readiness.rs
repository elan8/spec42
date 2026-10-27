//! Host notification ordering, separate from the semantic publication lifecycle.

use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ReadinessToken(Arc<()>);

#[derive(Clone)]
pub(crate) struct DiagnosticReadiness {
    token: ReadinessToken,
    announced: bool,
    initial: bool,
}

impl Default for DiagnosticReadiness {
    fn default() -> Self {
        Self {
            token: ReadinessToken(Arc::new(())),
            announced: false,
            initial: true,
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
            ..Self::default()
        };
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

    /// A send completed for this exact project/notification epoch. An older startup or
    /// reindex notification must never unlock diagnostics for its replacement.
    pub(crate) fn announce(&mut self, token: &ReadinessToken) -> bool {
        if !self.is_current(token) {
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
        let foreign = DiagnosticReadiness::default().token();
        assert!(!readiness.announce(&foreign));
        assert!(readiness.announce(&readiness.token()));
        assert!(readiness.announced());
    }
}

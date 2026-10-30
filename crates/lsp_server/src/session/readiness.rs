//! Host source-discovery and notification epochs, separate from semantic construction.

use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ReadinessToken(Arc<()>);

#[derive(Clone)]
pub(crate) struct DiskRevisionToken(Arc<()>);

#[derive(Clone)]
pub(crate) struct DiagnosticReadiness {
    token: ReadinessToken,
    announced: bool,
    initial: bool,
    source_inputs_pending: bool,
    startup_sources_pending: bool,
    notification_in_flight: bool,
    disk_observations: usize,
    disk_revision: DiskRevisionToken,
}

impl Default for DiagnosticReadiness {
    fn default() -> Self {
        Self {
            token: ReadinessToken(Arc::new(())),
            announced: false,
            initial: true,
            source_inputs_pending: false,
            startup_sources_pending: false,
            notification_in_flight: false,
            disk_observations: 0,
            disk_revision: DiskRevisionToken(Arc::new(())),
        }
    }
}

impl DiagnosticReadiness {
    pub(crate) fn rekey_for_owner(&mut self) {
        self.token = ReadinessToken(Arc::new(()));
        self.disk_observations = 0;
        self.disk_revision = DiskRevisionToken(Arc::new(()));
    }

    pub(crate) fn reset(&mut self, initial: bool) {
        let startup_sources_pending = !initial && self.startup_sources_pending;
        let disk_observations = self.disk_observations;
        *self = Self {
            initial,
            source_inputs_pending: !initial,
            startup_sources_pending,
            disk_observations,
            ..Self::default()
        };
    }

    pub(crate) fn begin_startup_sources(&mut self) {
        self.reset(true);
        self.startup_sources_pending = true;
        self.source_inputs_pending = true;
    }

    pub(crate) fn startup_sources_pending(&self) -> bool {
        self.startup_sources_pending
    }

    pub(crate) fn disk_observations_pending(&self) -> bool {
        self.disk_observations != 0
    }

    pub(crate) fn begin_disk_observation(&mut self) {
        self.disk_observations += 1;
        self.disk_revision = DiskRevisionToken(Arc::new(()));
        self.restart_pending_sources();
    }

    pub(crate) fn finish_disk_observation(&mut self) -> bool {
        self.disk_revision = DiskRevisionToken(Arc::new(()));
        self.disk_observations = self
            .disk_observations
            .checked_sub(1)
            .expect("balanced disk observation");
        self.disk_observations == 0 && self.restart_pending_sources()
    }

    pub(crate) fn disk_revision(&self) -> DiskRevisionToken {
        self.disk_revision.clone()
    }

    pub(crate) fn is_search_current(
        &self,
        token: &ReadinessToken,
        disk: &DiskRevisionToken,
    ) -> bool {
        self.is_current(token)
            && Arc::ptr_eq(&self.disk_revision.0, &disk.0)
            && !self.disk_observations_pending()
    }

    /// An observed disk batch invalidates every in-flight saved-source read. Keep the pending
    /// startup scope so the replacement still discovers workspace sources with the latest roots.
    pub(crate) fn restart_pending_sources(&mut self) -> bool {
        if !self.source_inputs_pending {
            return false;
        }
        self.token = ReadinessToken(Arc::new(()));
        self.announced = false;
        self.notification_in_flight = false;
        true
    }

    pub(crate) fn source_inputs_pending(&self) -> bool {
        self.source_inputs_pending
    }

    pub(crate) fn require_library_sources(&mut self) {
        if !self.source_inputs_pending {
            self.reset(false);
        }
    }

    /// Configured library source discovery completes before semantic construction can run.
    pub(crate) fn admit_source_inputs(&mut self, token: &ReadinessToken) -> bool {
        if !self.is_current(token)
            || !self.source_inputs_pending
            || self.disk_observations_pending()
        {
            return false;
        }
        self.source_inputs_pending = false;
        self.startup_sources_pending = false;
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

    pub(crate) fn needs_publication_notification(&self) -> bool {
        !self.announced && !self.source_inputs_pending && !self.notification_in_flight
    }

    pub(crate) fn begin_notification(&mut self, token: &ReadinessToken) -> bool {
        if !self.is_current(token)
            || self.source_inputs_pending
            || self.announced
            || self.notification_in_flight
        {
            return false;
        }
        self.notification_in_flight = true;
        true
    }

    /// A send completed for this exact project/notification epoch. An older startup or
    /// reindex notification must never unlock diagnostics for its replacement.
    pub(crate) fn announce(&mut self, token: &ReadinessToken) -> bool {
        if !self.is_current(token) || self.source_inputs_pending {
            return false;
        }
        self.announced = true;
        self.notification_in_flight = false;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_scope_and_overlapping_disk_barriers_survive_configuration_replacement() {
        let mut readiness = DiagnosticReadiness::default();
        readiness.begin_startup_sources();
        let old = readiness.token();
        readiness.begin_disk_observation();
        readiness.begin_disk_observation();
        readiness.reset(false);
        assert!(readiness.startup_sources_pending());
        let during = readiness.token();
        assert!(!readiness.admit_source_inputs(&during));
        assert!(!readiness.finish_disk_observation());
        assert!(!readiness.admit_source_inputs(&during));
        assert!(readiness.finish_disk_observation());
        assert!(!readiness.is_current(&old));
        assert!(!readiness.is_current(&during));
        assert!(readiness.admit_source_inputs(&readiness.token()));
        assert!(!readiness.startup_sources_pending());
        assert!(readiness.needs_publication_notification());
    }

    #[test]
    fn readiness_delivery_is_claimed_once_and_stale_sends_cannot_unlock_replacements() {
        let mut readiness = DiagnosticReadiness::default();
        let initial = readiness.token();
        assert!(readiness.begin_notification(&initial));
        assert!(!readiness.begin_notification(&initial));
        assert!(!readiness.needs_publication_notification());
        readiness.reset(false);
        assert!(!readiness.announce(&initial));
        let latest = readiness.token();
        assert!(!readiness.begin_notification(&latest));
        assert!(readiness.admit_source_inputs(&latest));
        assert!(readiness.begin_notification(&latest));
        assert!(readiness.announce(&latest));
        assert!(!readiness.begin_notification(&latest));
    }

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
        assert!(!readiness.needs_publication_notification());
        let foreign = DiagnosticReadiness::default().token();
        assert!(!readiness.announce(&foreign));
        assert!(!readiness.announce(&readiness.token()));
        assert!(!readiness.admit_source_inputs(&old));
        assert!(readiness.admit_source_inputs(&readiness.token()));
        assert!(readiness.needs_publication_notification());
        assert!(readiness.announce(&readiness.token()));
        assert!(readiness.announced());
        assert!(!readiness.needs_publication_notification());
    }
}

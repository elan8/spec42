//! Client document revisions, scoped to one open interval and one project owner.

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

static NEXT_OPEN_INTERVAL: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub(crate) struct EditorRevision {
    open_identity: Arc<()>,
    token: u64,
    version: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EditorEditRejection {
    NotIndexed,
    NotOpen,
    StaleVersion { current: i32, received: i32 },
    Superseded,
}

impl EditorRevision {
    pub(crate) fn new(version: Option<i32>) -> Self {
        Self {
            open_identity: Arc::new(()),
            token: NEXT_OPEN_INTERVAL
                .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
                .expect("editor interval identities exhausted"),
            version,
        }
    }

    pub(crate) fn token(&self) -> u64 {
        self.token
    }

    pub(crate) fn version(&self) -> Option<i32> {
        self.version
    }

    pub(crate) fn prepare(&self, received: i32) -> Result<EditorRevision, EditorEditRejection> {
        if let Some(current) = self.version {
            if received <= current {
                return Err(EditorEditRejection::StaleVersion { current, received });
            }
        }
        Ok(self.clone())
    }

    pub(crate) fn advance(
        &mut self,
        captured: &EditorRevision,
        received: i32,
    ) -> Result<(), EditorEditRejection> {
        if !Arc::ptr_eq(&self.open_identity, &captured.open_identity)
            || self.version != captured.version
        {
            return Err(EditorEditRejection::Superseded);
        }
        if let Some(current) = self.version {
            if received <= current {
                return Err(EditorEditRejection::StaleVersion { current, received });
            }
        }
        self.version = Some(received);
        Ok(())
    }

    pub(crate) fn rekey_for_owner(&mut self) {
        *self = Self::new(self.version);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_increase_without_assuming_positive_numbers_or_consecutive_steps() {
        let mut documents = EditorRevision::new(Some(-4));
        let captured = documents.prepare(8).unwrap();
        documents.advance(&captured, 8).unwrap();
        assert!(matches!(
            documents.prepare(8),
            Err(EditorEditRejection::StaleVersion {
                current: 8,
                received: 8
            })
        ));
        assert!(matches!(
            documents.prepare(-1),
            Err(EditorEditRejection::StaleVersion { .. })
        ));
    }

    #[test]
    fn captures_reject_other_documents_reopen_and_foreign_project_owners() {
        let mut first = EditorRevision::new(Some(1));
        let mut second = EditorRevision::new(Some(1));
        let capture = first.prepare(2).unwrap();
        assert_eq!(
            second.advance(&capture, 2),
            Err(EditorEditRejection::Superseded)
        );
        let mut foreign = first.clone();
        foreign.rekey_for_owner();
        assert_eq!(
            foreign.advance(&capture, 2),
            Err(EditorEditRejection::Superseded)
        );
        first = EditorRevision::new(Some(1));
        assert_eq!(
            first.advance(&capture, 2),
            Err(EditorEditRejection::Superseded)
        );
        let current = first.prepare(2).unwrap();
        first.advance(&current, 2).unwrap();
    }
}

//! Monotonic worker controls. They do not participate in semantic identity or model facts.
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug)]
struct Signal {
    cancelled: AtomicBool,
    parent: Option<Arc<Signal>>,
    #[cfg(test)]
    cancel_before: Option<crate::ConstructionPhase>,
}

#[derive(Debug, Clone)]
pub struct BuildCancellation(Arc<Signal>);

impl PartialEq for BuildCancellation {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for BuildCancellation {}

impl BuildCancellation {
    pub(crate) fn new() -> Self {
        Self(Arc::new(Signal {
            cancelled: AtomicBool::new(false),
            parent: None,
            #[cfg(test)]
            cancel_before: None,
        }))
    }
    #[cfg(test)]
    pub(super) fn at_phase(phase: crate::ConstructionPhase) -> Self {
        Self(Arc::new(Signal {
            cancelled: AtomicBool::new(false),
            parent: None,
            cancel_before: Some(phase),
        }))
    }

    pub(crate) fn child(&self) -> Self {
        Self(Arc::new(Signal {
            cancelled: AtomicBool::new(false),
            parent: Some(Arc::clone(&self.0)),
            #[cfg(test)]
            cancel_before: None,
        }))
    }
    pub(crate) fn cancel(&self) {
        self.0.cancelled.store(true, Ordering::Release);
    }
    pub fn is_cancelled(&self) -> bool {
        let mut signal = Some(&*self.0);
        while let Some(current) = signal {
            if current.cancelled.load(Ordering::Acquire) {
                return true;
            }
            signal = current.parent.as_deref();
        }
        false
    }
    pub(crate) fn checkpoint(
        &self,
        phase: crate::ConstructionPhase,
    ) -> Result<(), crate::BuildFailure> {
        #[cfg(test)]
        {
            let mut signal = Some(&*self.0);
            while let Some(current) = signal {
                if current.cancel_before == Some(phase) {
                    self.cancel();
                    break;
                }
                signal = current.parent.as_deref();
            }
        }
        if self.is_cancelled() {
            Err(crate::BuildFailure::Cancelled { phase })
        } else {
            Ok(())
        }
    }
}

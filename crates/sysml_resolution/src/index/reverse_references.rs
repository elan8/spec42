//! Phase 6: settled references indexed by the declaration they name.

use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;

/// Publication-owned reverse edges from a resolved target to its authored reference sites.
///
/// The declaration occurrence is deliberately not stored here: it has different provenance and
/// remains an explicit `include_declaration` policy in the query API. Reference ids within each
/// target range stay in authored canonical order because the CSR is filled in ascending id order.
#[derive(Debug)]
pub(crate) struct ReverseReferenceIndex {
    pub(crate) ranges: Box<[(u32, u32)]>,
    pub(crate) references: Box<[AuthoredReferenceId]>,
}

impl ReverseReferenceIndex {
    pub(crate) fn build(
        declarations: usize,
        resolution: &ResolutionResults,
    ) -> Result<Self, ResolutionError> {
        let mut counts = vec![0u32; declarations];
        for outcome in resolution.outcomes.iter().copied() {
            if let ResolutionStatus::Resolved(target) = outcome {
                let count = counts
                    .get_mut(target.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                *count = count.checked_add(1).ok_or(ResolutionError::Capacity)?;
            }
        }

        let mut ranges = Vec::with_capacity(declarations);
        let mut starts = Vec::with_capacity(declarations);
        let mut end = 0u32;
        for count in counts {
            let start = end;
            end = end.checked_add(count).ok_or(ResolutionError::Capacity)?;
            ranges.push((start, end));
            starts.push(start);
        }
        let reference_count = usize::try_from(end).map_err(|_| ResolutionError::Capacity)?;
        let mut references = vec![AuthoredReferenceId(0); reference_count];
        for (index, outcome) in resolution.outcomes.iter().copied().enumerate() {
            let ResolutionStatus::Resolved(target) = outcome else {
                continue;
            };
            let cursor = starts
                .get_mut(target.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            let slot = references
                .get_mut(*cursor as usize)
                .ok_or(ResolutionError::InvalidStorage)?;
            *slot =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            *cursor = cursor.checked_add(1).ok_or(ResolutionError::Capacity)?;
        }
        Ok(Self {
            ranges: ranges.into_boxed_slice(),
            references: references.into_boxed_slice(),
        })
    }

    pub(crate) fn references(&self, target: DeclarationId) -> &[AuthoredReferenceId] {
        let Some(&(start, end)) = self.ranges.get(target.index()) else {
            return &[];
        };
        self.references
            .get(start as usize..end as usize)
            .unwrap_or_default()
    }
}

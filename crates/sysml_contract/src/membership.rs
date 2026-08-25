//! The opaque identity of a Membership relationship in one semantic publication.

use std::num::NonZeroU32;

/// One Membership relationship in one publication.
///
/// Like [`crate::SymbolId`], this is a publication-scoped dense handle. It is a distinct identity
/// domain because the OMG model treats a Membership and its member element as different semantic
/// objects even when the implementation stores their facts in aligned slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MembershipId(NonZeroU32);

impl MembershipId {
    /// Mints the handle for a dense membership slot.
    pub fn from_index(index: usize) -> Option<Self> {
        let ordinal = u32::try_from(index).ok()?.checked_add(1)?;
        NonZeroU32::new(ordinal).map(Self)
    }

    /// Returns the dense membership slot addressed by this handle.
    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::MembershipId;
    use std::mem::size_of;

    #[test]
    fn dense_membership_slots_round_trip() {
        for index in [0, 1, 41, u32::MAX as usize - 1] {
            let identity = MembershipId::from_index(index).expect("representable membership slot");
            assert_eq!(identity.index(), index);
        }
        assert!(MembershipId::from_index(u32::MAX as usize).is_none());
    }

    #[test]
    fn membership_identity_preserves_the_nonzero_niche() {
        assert_eq!(size_of::<MembershipId>(), size_of::<u32>());
        assert_eq!(size_of::<Option<MembershipId>>(), size_of::<u32>());
    }
}

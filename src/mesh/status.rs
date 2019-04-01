//! Status flags for mesh items.

extern crate bitflags;

use crate::property::StorageFor;

#[doc(hidden)]
pub type FlagBits = u32;

bitflags! {
    #[doc = "Flags for each vertex/halfedge/edge/face field."]
    flags Status: FlagBits {
        #[doc = "Whether the item is deleted."]
        const DELETED           = 1,
        #[doc = "Whether the item is locked."]
        const LOCKED            = 2,
        #[doc = "Whether the item is selected."]
        const SELECTED          = 4,
        #[doc = "Whether the item is hidden."]
        const HIDDEN            = 8,
        #[doc = "Whether the item is a feature."]
        const FEATURE           = 16,
        #[doc = "Whether the item is a tagged."]
        const TAGGED            = 32,
        #[doc = "Whether the item is a tagged (alternate flag)."]
        const TAGGED2           = 64,
        #[doc = "Whether the item was non-2-manifold and was fixed."]
        const FIXED_NON_MANIFOLD = 128,
    }
}

impl Status {
    /// Inserts the specified flags into `self` if `b` is true; otherwise removes them.
    pub fn update(&mut self, other: Status, b: bool) {
        if b { self.insert(other) } else { self.remove(other) }
    }

    /// Whether the DELETED flag is set.
    pub fn deleted          (self) -> bool { self.contains(DELETED          ) }
    /// Whether the LOCKED flag is set.
    pub fn locked           (self) -> bool { self.contains(LOCKED           ) }
    /// Whether the SELECTED flag is set.
    pub fn selected         (self) -> bool { self.contains(SELECTED         ) }
    /// Whether the HIDDEN flag is set.
    pub fn hidden           (self) -> bool { self.contains(HIDDEN           ) }
    /// Whether the FEATURE flag is set.
    pub fn feature          (self) -> bool { self.contains(FEATURE          ) }
    /// Whether the TAGGED flag is set.
    pub fn tagged           (self) -> bool { self.contains(TAGGED           ) }
    /// Whether the TAGGED2 flag is set.
    pub fn tagged2          (self) -> bool { self.contains(TAGGED2          ) }
    /// Whether the FIXED_NON_MANIFOLD flag is set.
    pub fn fixed_non_manifold(self) -> bool { self.contains(FIXED_NON_MANIFOLD) }

    /// Sets the DELETED flag.
    pub fn set_deleted          (&mut self, b: bool) { self.update(DELETED          , b) }
    /// Sets the LOCKED flag.
    pub fn set_locked           (&mut self, b: bool) { self.update(LOCKED           , b) }
    /// Sets the SELECTED flag.
    pub fn set_selected         (&mut self, b: bool) { self.update(SELECTED         , b) }
    /// Sets the HIDDEN flag.
    pub fn set_hidden           (&mut self, b: bool) { self.update(HIDDEN           , b) }
    /// Sets the FEATURE flag.
    pub fn set_feature          (&mut self, b: bool) { self.update(FEATURE          , b) }
    /// Sets the TAGGED flag.
    pub fn set_tagged           (&mut self, b: bool) { self.update(TAGGED           , b) }
    /// Sets the TAGGED2 flag.
    pub fn set_tagged2          (&mut self, b: bool) { self.update(TAGGED2          , b) }
    /// Sets the FIXED_NON_MANIFOLD flag.
    pub fn set_fixed_non_manifold(&mut self, b: bool) { self.update(FIXED_NON_MANIFOLD, b) }

    /// Iterator to enumerate all `Status` flags.
    pub fn iter() -> Iter {
        Iter {
            cond: Status::all().bits(),
            flag: 1 as FlagBits
        }
    }
}

impl Default for Status {
    fn default() -> Self { Status::empty() }
}

impl StorageFor for Status {
    type Storage = Vec<Status>;
}

////////////////////////////////////////////////////////////////////////////////

/// Iterator to enumerate all `Status` bits/flags.
pub struct Iter {
    cond: FlagBits,
    flag: FlagBits
}

impl Iterator for Iter {
    type Item = Status;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cond == 0 as FlagBits {
            None
        } else {
            let status = Status::from_bits_truncate(self.flag);
            self.flag <<= 1;
            self.cond >>= 1;
            Some(status)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use crate::mesh::status::Status;

    #[test]
    fn test_empty() {
        let flags = Status::empty();
        assert!(!flags.deleted());
        assert!(!flags.locked());
        assert!(!flags.selected());
        assert!(!flags.hidden());
        assert!(!flags.feature());
        assert!(!flags.tagged());
        assert!(!flags.tagged2());
        assert!(!flags.fixed_non_manifold());
    }

    #[test]
    fn test_all() {
        let flags = Status::all();
        assert!(flags.deleted());
        assert!(flags.locked());
        assert!(flags.selected());
        assert!(flags.hidden());
        assert!(flags.feature());
        assert!(flags.tagged());
        assert!(flags.tagged2());
        assert!(flags.fixed_non_manifold());
    }

    #[test]
    fn test_set_all() {
        let mut flags = Status::empty();
        flags.set_deleted(true);
        flags.set_locked(true);
        flags.set_selected(true);
        flags.set_hidden(true);
        flags.set_feature(true);
        flags.set_tagged(true);
        flags.set_tagged2(true);
        flags.set_fixed_non_manifold(true);
        assert_eq!(flags, Status::all());
        flags.set_deleted(false);
        flags.set_locked(false);
        flags.set_selected(false);
        flags.set_hidden(false);
        flags.set_feature(false);
        flags.set_tagged(false);
        flags.set_tagged2(false);
        flags.set_fixed_non_manifold(false);
        assert_eq!(flags, Status::empty());
    }

    #[test]
    fn test_iter() {
        let mut flags = Status::empty();
        for flag in Status::iter() {
            flags.insert(flag);
        }
        assert_eq!(flags, Status::all());
    }
}


//! Status flags for mesh items.

use crate::property::StorageFor;

#[doc(hidden)]
pub(crate) type FlagBits = u32;

bitflags::bitflags! {
    #[doc = "Flags for each vertex/halfedge/edge/face field."]
    pub struct Status: FlagBits {
        #[doc = "Whether the item is deleted."]
        const DELETED            = 1;
        #[doc = "Whether the item is locked."]
        const LOCKED             = 2;
        #[doc = "Whether the item is selected."]
        const SELECTED           = 4;
        #[doc = "Whether the item is hidden."]
        const HIDDEN             = 8;
        #[doc = "Whether the item is a feature."]
        const FEATURE            = 16;
        #[doc = "Whether the item is a tagged."]
        const TAGGED             = 32;
        #[doc = "Whether the item is a tagged (alternate flag)."]
        const TAGGED2            = 64;
        #[doc = "Whether the item was non-2-manifold and was fixed."]
        const FIXED_NON_MANIFOLD = 128;
    }
}

macro_rules! def_methods {
    ($name:ident, $set_name:ident, $flag:ident) => {
        #[doc = concat!("Whether the ", stringify!($flag), " flag is set.")]
        pub fn $name(self) -> bool {
            self.contains(Status::$flag)
        }

        #[doc = concat!("Sets the ", stringify!($flag), " flag.")]
        pub fn $set_name(&mut self, b: bool) {
            self.set(Status::$flag, b);
        }
    };
}

impl Status {
    def_methods!(deleted, set_deleted, DELETED);
    def_methods!(locked, set_locked, LOCKED);
    def_methods!(selected, set_selected, SELECTED);
    def_methods!(hidden, set_hidden, HIDDEN);
    def_methods!(feature, set_feature, FEATURE);
    def_methods!(tagged, set_tagged, TAGGED);
    def_methods!(tagged2, set_tagged2, TAGGED2);
    def_methods!(
        fixed_non_manifold,
        set_fixed_non_manifold,
        FIXED_NON_MANIFOLD
    );

    /// Iterator to enumerate all `Status` flags.
    pub fn iter() -> Iter {
        Iter {
            cond: Status::all().bits(),
            flag: 1 as FlagBits,
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::empty()
    }
}

impl StorageFor for Status {
    type Storage = Vec<Status>;
}

////////////////////////////////////////////////////////////////////////////////

/// Iterator to enumerate all `Status` bits/flags.
pub struct Iter {
    cond: FlagBits,
    flag: FlagBits,
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

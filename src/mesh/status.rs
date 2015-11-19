extern crate bitflags;

pub type FlagBits = u32;

bitflags! {
    #[doc = "Flags for each vertex/halfedge/edge/face field."]
    flags Status: FlagBits {
        const DELETED           = 1,
        const LOCKED            = 2,
        const SELECTED          = 4,
        const HIDDEN            = 8,
        const FEATURE           = 16,
        const TAGGED            = 32,
        const TAGGED2           = 64,
        const FIXED_NONMANIFOLD = 128,
    }
}

impl Status {
    /// Inserts the specified flags into `self` if `b` is true; otherwise removes them.
    pub fn update(&mut self, other: Status, b: bool) {
        if b { self.insert(other) } else { self.remove(other) }
    }

    pub fn deleted          (&self) -> bool { self.contains(DELETED          ) }
    pub fn locked           (&self) -> bool { self.contains(LOCKED           ) }
    pub fn selected         (&self) -> bool { self.contains(SELECTED         ) }
    pub fn hidden           (&self) -> bool { self.contains(HIDDEN           ) }
    pub fn feature          (&self) -> bool { self.contains(FEATURE          ) }
    pub fn tagged           (&self) -> bool { self.contains(TAGGED           ) }
    pub fn tagged2          (&self) -> bool { self.contains(TAGGED2          ) }
    pub fn fixed_nonmanifold(&self) -> bool { self.contains(FIXED_NONMANIFOLD) }

    pub fn set_deleted          (&mut self, b: bool) { self.update(DELETED          , b) }
    pub fn set_locked           (&mut self, b: bool) { self.update(LOCKED           , b) }
    pub fn set_selected         (&mut self, b: bool) { self.update(SELECTED         , b) }
    pub fn set_hidden           (&mut self, b: bool) { self.update(HIDDEN           , b) }
    pub fn set_feature          (&mut self, b: bool) { self.update(FEATURE          , b) }
    pub fn set_tagged           (&mut self, b: bool) { self.update(TAGGED           , b) }
    pub fn set_tagged2          (&mut self, b: bool) { self.update(TAGGED2          , b) }
    pub fn set_fixed_nonmanifold(&mut self, b: bool) { self.update(FIXED_NONMANIFOLD, b) }

    pub fn iter() -> Iter {
        Iter {
            cond: Status::all().bits(),
            flag: 1 as FlagBits
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Iterator to enumerate all Status flags.
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
    use mesh::status::Status;

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
        assert!(!flags.fixed_nonmanifold());
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
        assert!(flags.fixed_nonmanifold());
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
        flags.set_fixed_nonmanifold(true);
        assert_eq!(flags, Status::all());
        flags.set_deleted(false);
        flags.set_locked(false);
        flags.set_selected(false);
        flags.set_hidden(false);
        flags.set_feature(false);
        flags.set_tagged(false);
        flags.set_tagged2(false);
        flags.set_fixed_nonmanifold(false);
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


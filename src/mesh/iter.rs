//! Iterators to enumerate all items in the mesh, skipping DELETED and HIDDEN items.

use mesh::status;
use mesh::mesh::Mesh;
use mesh::status::Status;
use property::Property;
use property::size::Size;
use property::traits::Handle;


/// Supports bidirectional iteration through all elements of a mesh.
trait MeshIterMeta {
    /// The status bits for the handle. If no status bits are stored, Status::empty() is returned.
    fn status_prop(mesh: &Mesh) -> Option<&Property<Status, Self>>
        where Self: Sized;
    /// Count of the number of Handle type elements in the mesh (including ones to be skipped).
    fn size(mesh: &Mesh) -> Size;
}

// Default implementation -- should always be specialized.
// This is here to avoid adding MeshIterMeta as an explicit constraint in the signatures below.
// With this, `H: Handle` automatically assumes that a implementation of `MeshIterMeta` exists.
impl<H: Handle> MeshIterMeta for H {
    default fn status_prop(_mesh: &Mesh) -> Option<&Property<Status, Self>>
        where Self: Sized
        { None }
    default fn size(_mesh: &Mesh) -> Size { unimplemented!() }
}

struct IterBase<'a, H: Handle> {
    mesh: &'a Mesh,
    h: H,
    status_prop: Option<&'a Property<Status, H>>,
    skip_bits: Status,
}

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
// Also, the derive version constrains all type parameters to be `Copy` (shouldn't hurt here,
// though).

impl<'a, H: Handle> ::std::fmt::Debug for IterBase<'a, H> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "IterBase(h={:?}, skip_bits={:?})", self.h, self.skip_bits)
    }
}
impl<'a, H: Handle> Copy for IterBase<'a, H> {}
impl<'a, H: Handle> Clone for IterBase<'a, H> { fn clone(&self) -> Self { *self } }


impl<'a, H: Handle> IterBase<'a, H> {
    fn new(mesh: &'a Mesh, handle: H, skip: bool, is_fwd: bool) -> IterBase<'a, H> {
        // This should be a const, but user-defined operators cannot be used to initialize them.
        let skippable: Status = status::DELETED | status::HIDDEN;

        let mut iter = IterBase {
            mesh,
            h: handle,
            status_prop: <H as MeshIterMeta>::status_prop(mesh),
            skip_bits: if skip { skippable } else { Status::empty() }
        };
        if skip {
            if is_fwd {
                iter.skip_fwd();
            } else {
                iter.skip_bwd();
            }
        }
        iter
    }

    fn should_skip(&self) -> bool {
        self.status_prop
            .map(|prop| !(prop[self.h] & self.skip_bits).is_empty())
            .unwrap_or(false)
    }

    /// If the iterator is on an item to be skipped, it increments the iterator until a
    /// non-skipped item is encounted or the list is exhausted.
    fn skip_fwd(&mut self) {
        if !self.h.is_valid() { return; }
        let mut within_bounds;
        while {
                  within_bounds = self.h.index() < <H as MeshIterMeta>::size(self.mesh);
                  within_bounds
              } && self.should_skip()
        {
            self.h.__increment();
        }
        // Exceeded bounds while skipping.
        if !within_bounds {
            self.h.invalidate();
        }
    }

    /// Traverses to the next non-skipped handle and returns it, or `None` if exhausted.
    fn next_fwd(&mut self) -> Option<H> {
        let res = self.h.to_option();
        if self.h.is_valid() {
            self.h.__increment();
            self.skip_fwd();
        }
        res
    }

    /// If the iterator is on an item to be skipped, it decrements the iterator until a
    /// non-skipped item is encounted or the list is exhausted.
    fn skip_bwd(&mut self) {
        if !self.h.is_valid() { return; }
        assert!(self.h.index() < <H as MeshIterMeta>::size(self.mesh),
                "Handle, if valid, must be within mesh bounds.");
        let mut skip;
        // Important to compute skip before checking index due to end condition.
        while {
                  skip = self.should_skip();
                  skip
              } && self.h.index() > 0
        {
            self.h.__decrement();
        }
        // Reached the end of bounds while skipping, and last time needs to be skipped also.
        if skip {
            self.h.invalidate();
        }
    }

    /// Traverses to the previous non-skipped handle and returns it, or `None` if exhausted.
    fn next_bwd(&mut self) -> Option<H> {
        let res = self.h.to_option();
        if self.h.is_valid() {
            self.h.__decrement(); // can roll over to INVALID_INDEX
            self.skip_bwd();
        }
        res
    }
}


/// Forward iterator through the mesh.
#[derive(Debug)]
pub struct FwdIter<'a, H: Handle>(IterBase<'a, H>);

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
impl<'a, H: Handle> Copy for FwdIter<'a, H> {}
impl<'a, H: Handle> Clone for FwdIter<'a, H> { fn clone(&self) -> Self { *self } }

impl<'a, H: Handle> FwdIter<'a, H> {
    /// Initialize a forward iterator through the mesh starting at the given handle.
    /// If `skip` is true and the mesh stores a status field, then the iterator skips
    /// all elements with DELETED or HIDDEN status.
    pub fn new(mesh: &'a Mesh, handle: H, skip: bool) -> FwdIter<'a, H> {
        FwdIter(IterBase::new(mesh, handle, skip, true /* is_fwd */))
    }
}

impl<'a, H: Handle> Iterator for FwdIter<'a, H> {
    type Item = H;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_fwd()
    }
}


/// Backward iterator through the mesh.
#[derive(Debug)]
pub struct BwdIter<'a, H: Handle>(IterBase<'a, H>);

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
impl<'a, H: Handle> Copy for BwdIter<'a, H> {}
impl<'a, H: Handle> Clone for BwdIter<'a, H> { fn clone(&self) -> Self { *self } }

impl<'a, H: Handle> BwdIter<'a, H> {
    /// Initialize a backward iterator through the mesh starting at the given handle.
    /// If `skip` is true and the mesh stores a status field, then the iterator skips
    /// all elements with DELETED or HIDDEN status.
    pub fn new(mesh: &'a Mesh, handle: H, skip: bool) -> BwdIter<'a, H> {
        BwdIter(IterBase::new(mesh, handle, skip, false /* is_fwd */))
    }
}

impl<'a, H: Handle> Iterator for BwdIter<'a, H> {
    type Item = H;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_bwd()
    }
}


/*
#[cfg(test)]
mod test {
    use super::{MeshIterMeta, FwdIter, BwdIter};
    use mesh::handles::VertexHandle;
    use mesh::status::{Status, DELETED, HIDDEN, SELECTED};
    use property::size::{Index, Size};
    use property::traits::Handle;

    struct Mesh {
        skip: Box<Fn(Index) -> bool>
    }
    impl ::std::fmt::Debug for Mesh {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "Mesh")
        }
    }

    static mut SKIP_INDEX: u32 = 0;

    // TODO: This should be replaced with the actual implementation on VertexHandle and tested
    // there.
    impl MeshIterMeta for VertexHandle {
        fn status(mesh: &Mesh, h: VertexHandle) -> Status {
            if (*mesh.skip)(h.index()) {
                unsafe { SKIP_INDEX += 1; }
                (match unsafe { SKIP_INDEX } % 3 {
                    0 => DELETED,
                    1 => HIDDEN,
                    _ => DELETED | HIDDEN
                } | SELECTED)
            } else {
                SELECTED
            }
        }
        fn size(_mesh: &Mesh) -> Size { 10 }
    }

    fn fwd_list(mesh: Mesh, skip: bool) -> Vec<Index> {
        FwdIter::<Meta>::new(&mesh, VertexHandle::from_index(0), skip)
            .map(|x| x.index()).collect()
    }
    fn bwd_list(mesh: Mesh, skip: bool) -> Vec<Index> {
        BwdIter::<Meta>::new(&mesh, VertexHandle::from_index(9), skip)
            .map(|x| x.index()).collect()
    }

    #[test]
    fn test_fwd_iter_no_skip() {
        let all = &(0..10).collect::<Vec<_>>();
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 0)
        };
        assert_eq!(&fwd_list(mesh, false /* skip */), all);
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 1)
        };
        assert_eq!(&fwd_list(mesh, false /* skip */), all);
    }

    #[test]
    fn test_fwd_iter_skip() {
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 0)
        };
        assert_eq!(&fwd_list(mesh, true /* skip */),
                   &(0..5).map(|x| x * 2 + 1).collect::<Vec<_>>());
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 1)
        };
        assert_eq!(&fwd_list(mesh, true /* skip */),
                   &(0..5).map(|x| x * 2).collect::<Vec<_>>());
    }

    #[test]
    fn test_bwd_iter_no_skip() {
        let all_rev = &(0..10).rev().collect::<Vec<_>>();
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 0)
        };
        assert_eq!(&bwd_list(mesh, false /* skip */), all_rev);
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 1)
        };
        assert_eq!(&bwd_list(mesh, false /* skip */), all_rev);
    }

    #[test]
    fn test_bwd_iter_skip() {
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 0)
        };
        assert_eq!(&bwd_list(mesh, true /* skip */),
                   &(0..5).rev().map(|x| x * 2 + 1).collect::<Vec<_>>());
        let mesh = Mesh {
            skip: Box::new(|x| x % 2 == 1)
        };
        assert_eq!(&bwd_list(mesh, true /* skip */),
                   &(0..5).rev().map(|x| x * 2).collect::<Vec<_>>());
    }
}
*/

//! Iterators to enumerate all items in the mesh, skipping DELETED and HIDDEN items.

use crate::mesh::item_handle::MeshItemHandle;
use crate::mesh::Mesh;
use crate::mesh::status::{self, Status};
use crate::property::Property;


struct IterBase<'a, H: MeshItemHandle> {
    mesh: &'a Mesh,
    h: H,
    status_prop: Option<&'a Property<Status, H>>,
    skip_bits: Status,
}

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
// Also, the derive version constrains all type parameters to be `Copy` (shouldn't hurt here,
// though).

impl<'a, H: MeshItemHandle> ::std::fmt::Debug for IterBase<'a, H> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "IterBase(h={:?}, skip_bits={:?})", self.h, self.skip_bits)
    }
}
impl<'a, H: MeshItemHandle> Copy for IterBase<'a, H> {}
impl<'a, H: MeshItemHandle> Clone for IterBase<'a, H> { fn clone(&self) -> Self { *self } }


impl<'a, H: MeshItemHandle> IterBase<'a, H> {
    fn new(mesh: &'a Mesh, handle: H, skip: bool, is_fwd: bool) -> IterBase<'a, H> {
        // This should be a const, but user-defined operators cannot be used to initialize them.
        let skippable: Status = status::DELETED | status::HIDDEN;

        let mut iter = IterBase {
            mesh,
            h: handle,
            status_prop: <H as MeshItemHandle>::status_prop(mesh),
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
                  within_bounds = self.h.index() < H::len(self.mesh);
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
        assert!(self.h.index() < H::len(self.mesh),
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
pub struct FwdIter<'a, H: MeshItemHandle>(IterBase<'a, H>);

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
impl<'a, H: MeshItemHandle> Copy for FwdIter<'a, H> {}
impl<'a, H: MeshItemHandle> Clone for FwdIter<'a, H> { fn clone(&self) -> Self { *self } }

impl<'a, H: MeshItemHandle> FwdIter<'a, H> {
    /// Initialize a forward iterator through the mesh starting at the given handle.
    /// If `skip` is true and the mesh stores a status field, then the iterator skips
    /// all elements with DELETED or HIDDEN status.
    pub fn new(mesh: &'a Mesh, handle: H, skip: bool) -> FwdIter<'a, H> {
        FwdIter(IterBase::new(mesh, handle, skip, true /* is_fwd */))
    }
}

impl<'a, H: MeshItemHandle> Iterator for FwdIter<'a, H> {
    type Item = H;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_fwd()
    }
}


/// Backward iterator through the mesh.
#[derive(Debug)]
pub struct BwdIter<'a, H: MeshItemHandle>(IterBase<'a, H>);

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
impl<'a, H: MeshItemHandle> Copy for BwdIter<'a, H> {}
impl<'a, H: MeshItemHandle> Clone for BwdIter<'a, H> { fn clone(&self) -> Self { *self } }

impl<'a, H: MeshItemHandle> BwdIter<'a, H> {
    /// Initialize a backward iterator through the mesh starting at the given handle.
    /// If `skip` is true and the mesh stores a status field, then the iterator skips
    /// all elements with DELETED or HIDDEN status.
    pub fn new(mesh: &'a Mesh, handle: H, skip: bool) -> BwdIter<'a, H> {
        BwdIter(IterBase::new(mesh, handle, skip, false /* is_fwd */))
    }
}

impl<'a, H: MeshItemHandle> Iterator for BwdIter<'a, H> {
    type Item = H;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_bwd()
    }
}


#[cfg(test)]
mod test {
    use super::{FwdIter, BwdIter};
    use crate::property::size::{Index, Size};
    use crate::property::traits::Handle; // For constructor.
    use crate::mesh::item_handle::VertexHandle;
    use crate::mesh::status::{DELETED, HIDDEN, SELECTED};
    use crate::mesh::Mesh;

    fn fwd_list(mesh: Mesh, skip: bool) -> Vec<Index> {
        FwdIter::new(&mesh, VertexHandle::from_index(0), skip)
            .map(|x| x.index()).collect()
    }
    fn bwd_list(mesh: Mesh, skip: bool) -> Vec<Index> {
        let end = mesh.vertices().len() as Size - 1;
        BwdIter::new(&mesh, VertexHandle::from_index(end), skip)
            .map(|x| x.index()).collect()
    }

    /// Adds skippable status on all vertices for whom `skip_index_fn` returns true on its handle.
    /// It circulates between variations of skippable statuses.
    fn with_status(mesh: Mesh, skip_index_fn: for<'a> fn(&'a Index) -> bool) -> Mesh {
        let mut mesh = mesh;
        mesh.request_vertex_status();
        let prop = mesh.get_vertex_status_mut().unwrap();
        let mut skip_type = 0;
        for (i, status) in prop.iter_internal_mut().enumerate() {
            *status =
                if skip_index_fn(&(i as Index)) {
                    skip_type += 1;
                    (match skip_type % 3 {
                        0 => DELETED,
                        1 => HIDDEN,
                        _ => DELETED | HIDDEN,
                    } | SELECTED)
                } else {
                    SELECTED
                }
        }
        mesh
    }
    
    fn is_even(i: &Index) -> bool { i % 2 == 0 }
    fn is_odd(i: &Index) -> bool { i % 2 == 1 }

    #[test]
    fn test_fwd_iter_no_skip() {
        let all = &(0..15).collect::<Vec<_>>();

        let mesh = with_status(Mesh::debug_triangles(5), is_even);
        assert_eq!(&fwd_list(mesh, false /* skip */), all);

        let mesh = with_status(Mesh::debug_triangles(5), is_odd);
        assert_eq!(&fwd_list(mesh, false /* skip */), all);
    }

    #[test]
    fn test_fwd_iter_skip() {
        let mesh = with_status(Mesh::debug_triangles(5), is_even);
        assert_eq!(&fwd_list(mesh, true /* skip */),
                   &(0..15).filter(is_odd).collect::<Vec<_>>());

        let mesh = with_status(Mesh::debug_triangles(5), is_odd);
        assert_eq!(&fwd_list(mesh, true /* skip */),
                   &(0..15).filter(is_even).collect::<Vec<_>>());
    }

    #[test]
    fn test_bwd_iter_no_skip() {
        let all_rev = &(0..15).rev().collect::<Vec<_>>();

        let mesh = with_status(Mesh::debug_triangles(5), is_even);
        assert_eq!(&bwd_list(mesh, false /* skip */), all_rev);

        let mesh = with_status(Mesh::debug_triangles(5), is_odd);
        assert_eq!(&bwd_list(mesh, false /* skip */), all_rev);
    }

    #[test]
    fn test_bwd_iter_skip() {
        let mesh = with_status(Mesh::debug_triangles(5), is_even);
        assert_eq!(&bwd_list(mesh, true /* skip */),
                   &(0..15).rev().filter(is_odd).collect::<Vec<_>>());

        let mesh = with_status(Mesh::debug_triangles(5), is_odd);
        assert_eq!(&bwd_list(mesh, true /* skip */),
                   &(0..15).rev().filter(is_even).collect::<Vec<_>>());
    }
}

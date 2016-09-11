use mesh::status;
use mesh::status::Status;
use property::size::Size;
use property::traits::Handle;


pub trait MeshIterMeta {
    type Mesh: ::std::fmt::Debug;
    type Handle: Handle;

    /// Whether a `Status` field is provided for this Handle type in the mesh.
    fn has_status() -> bool;
    /// The status bits for the handle.
    fn status(mesh: &Self::Mesh, h: Self::Handle) -> Status;
    /// Count of the number of Handle type elements in the mesh (including ones to be skipped).
    fn size(mesh: &Self::Mesh) -> Size;
}


#[derive(Debug)]
struct MeshIterBase<'a, Meta: MeshIterMeta>
    where Meta::Mesh: 'a
{
    mesh: &'a Meta::Mesh,
    h: Meta::Handle,
    skip_bits: Status
}

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
// Also, the derive version constrains all type parameters to be `Copy` (shouldn't hurt here,
// though).

impl<'a, Meta: MeshIterMeta> Copy for MeshIterBase<'a, Meta> where Meta::Mesh: 'a {}
impl<'a, Meta: MeshIterMeta> Clone for MeshIterBase<'a, Meta> where Meta::Mesh: 'a
{ fn clone(&self) -> Self { *self } }


impl<'a, Meta: MeshIterMeta> MeshIterBase<'a, Meta>
    where Meta::Mesh: 'a
{
    fn new(mesh: &'a Meta::Mesh, handle: Meta::Handle, skip: bool, is_fwd: bool)
        -> MeshIterBase<'a, Meta>
    {
        // This should be a const, but user-defined operators cannot be used to initialize them.
        let skippable: Status = status::DELETED | status::HIDDEN;

        let skip = skip && Meta::has_status();
        let mut iter = MeshIterBase {
            mesh: mesh,
            h: handle,
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

    // If the iterator is on an item to be skipped, it increments the iterator until a
    // non-skipped item is encounted or the list is exhausted.
    fn skip_fwd(&mut self) {
        if !self.h.is_valid() { return; }
        let mut within_bounds;
        while {
                  within_bounds = self.h.index() < Meta::size(self.mesh);
                  within_bounds
              } && !(Meta::status(self.mesh, self.h) & self.skip_bits).is_empty()
        {
            self.h.__increment();
        }
        // Exceeded bounds while skipping.
        if !within_bounds {
            self.h.invalidate();
        }
    }

    // Traverses to the next non-skipped handle and returns it, or `None` if exhausted.
    fn next_fwd(&mut self) -> Option<Meta::Handle> {
        let res = self.h.to_option();
        if self.h.is_valid() {
            self.h.__increment();
            self.skip_fwd();
        }
        res
    }

    // If the iterator is on an item to be skipped, it decrements the iterator until a
    // non-skipped item is encounted or the list is exhausted.
    fn skip_bwd(&mut self) {
        if !self.h.is_valid() { return; }
        assert!(self.h.index() < Meta::size(self.mesh),
                "Handle, if valid, must be within mesh bounds.");
        let mut skip;
        // Important to compute skip before checking index due to end condition.
        while {
                  skip = !(Meta::status(self.mesh, self.h) & self.skip_bits).is_empty();
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

    // Traverses to the previous non-skipped handle and returns it, or `None` if exhausted.
    fn next_bwd(&mut self) -> Option<Meta::Handle> {
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
pub struct MeshFwdIter<'a, Meta: MeshIterMeta>(MeshIterBase<'a, Meta>)
    where Meta::Mesh: 'a;

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
impl<'a, Meta: MeshIterMeta> Copy for MeshFwdIter<'a, Meta> where Meta::Mesh: 'a {}
impl<'a, Meta: MeshIterMeta> Clone for MeshFwdIter<'a, Meta> where Meta::Mesh: 'a
{ fn clone(&self) -> Self { *self } }

impl<'a, Meta: MeshIterMeta> MeshFwdIter<'a, Meta>
    where Meta::Mesh: 'a
{
    pub fn new(mesh: &'a Meta::Mesh, handle: Meta::Handle, skip: bool)
        -> MeshFwdIter<'a, Meta>
    {
        MeshFwdIter(MeshIterBase::new(mesh, handle, skip, true /* is_fwd */))
    }
}

impl<'a, Meta: MeshIterMeta> Iterator for MeshFwdIter<'a, Meta>
    where Meta::Mesh: 'a
{
    type Item = Meta::Handle;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_fwd()
    }
}


/// Backward iterator through the mesh.
#[derive(Debug)]
pub struct MeshBwdIter<'a, Meta: MeshIterMeta>(MeshIterBase<'a, Meta>)
    where Meta::Mesh: 'a;

// Manually implement `Copy`, `Clone` due to https://github.com/rust-lang/rust/issues/32872.
impl<'a, Meta: MeshIterMeta> Copy for MeshBwdIter<'a, Meta> where Meta::Mesh: 'a {}
impl<'a, Meta: MeshIterMeta> Clone for MeshBwdIter<'a, Meta> where Meta::Mesh: 'a
{ fn clone(&self) -> Self { *self } }

impl<'a, Meta: MeshIterMeta> MeshBwdIter<'a, Meta>
    where Meta::Mesh: 'a
{
    /// Initialize a forward iterator through the mesh starting at the given handle.
    /// If `skip` is true and the mesh stores a status field, then the iterator skips
    /// all elements with DELETED or HIDDEN status.
    pub fn new(mesh: &'a Meta::Mesh, handle: Meta::Handle, skip: bool)
        -> MeshBwdIter<'a, Meta>
    {
        MeshBwdIter(MeshIterBase::new(mesh, handle, skip, false /* is_fwd */))
    }
}

impl<'a, Meta: MeshIterMeta> Iterator for MeshBwdIter<'a, Meta>
    where Meta::Mesh: 'a
{
    type Item = Meta::Handle;

    /// Initialize a backward iterator through the mesh starting at the given handle.
    /// If `skip` is true and the mesh stores a status field, then the iterator skips
    /// all elements with DELETED or HIDDEN status.
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_bwd()
    }
}


#[cfg(test)]
mod test {
    use super::{MeshIterMeta, MeshFwdIter, MeshBwdIter};
    use mesh::handles::VertexHandle;
    use mesh::status::{Status, DELETED, HIDDEN, SELECTED};
    use property::size::{Index, Size};
    use property::traits::Handle;

    #[derive(Clone, Debug)]
    struct Meta;

    struct Mesh {
        skip: Box<Fn(Index) -> bool>
    }
    impl ::std::fmt::Debug for Mesh {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "Mesh")
        }
    }

    static mut skip_index: u32 = 0;

    impl MeshIterMeta for Meta {
        type Mesh = Mesh;
        type Handle = VertexHandle;
        fn has_status() -> bool { true }
        fn status(mesh: &Mesh, h: Self::Handle) -> Status {
            if (*mesh.skip)(h.index()) {
                unsafe { skip_index += 1; }
                (match unsafe { skip_index } % 3 {
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
        MeshFwdIter::<Meta>::new(&mesh, VertexHandle::from_index(0), skip)
            .map(|x| x.index()).collect()
    }
    fn bwd_list(mesh: Mesh, skip: bool) -> Vec<Index> {
        MeshBwdIter::<Meta>::new(&mesh, VertexHandle::from_index(9), skip)
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

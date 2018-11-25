//! See documentation for `RcPropHandle`.
use mesh::item_handle::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle,
    MeshItemHandle,
};
use mesh::mesh::Mesh;
use property::traits::{self, PropertyFor};
use property::PropertyContainer;
use property::handle::PropHandle;

/// Ref-counted property handle.
///
/// This is used in `Mesh` to keep property lists around only while there are any outstanding uses
/// of them. This type is internal to `Mesh` and stored in the `Mesh` itself, thereby not requiring
/// a heap allocation.
#[derive(Clone, Default)]
pub(crate) struct RcPropHandle<H: MeshItemHandle, T: traits::Value> {
    handle: PropHandle<H, T>,
    ref_count: u32,
}

impl<H: MeshItemHandle, T: traits::Value> RcPropHandle<H, T> {
    /// Returns a `RcPropHandle` with an invalid handle an 0 ref count.
    pub(crate) fn new() -> RcPropHandle<H, T> { Default::default() }

    // TODO: Figure out how this is used and wrap request/release in an object like Rc.

    /// Request a property on the mesh if it doesn't already exist. It increases the ref count.
    pub(crate) fn request(&mut self, m: &mut Mesh) {
        self.ref_count += 1;
        if self.ref_count == 1 {
            debug_assert!(self.handle == Default::default());
            let name = H::with_prefix("status");
            self.handle = m.props_mut::<H>().add::<T>(Some(name));
        }
    }

    /// Request a property on the mesh if it doesn't already exist. It increases the ref count.
    pub(crate) fn release(&mut self, m: &mut Mesh) {
        if self.ref_count == 0 { return; }
        self.ref_count -= 1;
        if self.ref_count == 0 {
            m.props_mut::<H>().remove::<T>(&mut self.handle);;
        }
    }

    /// Get the status `Property` if it exists.
    pub(crate) fn get_prop<'a>(&self, props: &'a PropertyContainer<H>)
        -> Option<&'a <T as PropertyFor<H>>::Property> {
        props.get(self.handle)
    }
}


/// Reference-counted handle for a specific vertex property.
pub(crate) type RcVPropHandle<T> = RcPropHandle<VertexHandle, T>;
/// Reference-counted handle for a specific halfedge property.
pub(crate) type RcHPropHandle<T> = RcPropHandle<HalfedgeHandle, T>;
/// Reference-counted handle for a specific edge property.
pub(crate) type RcEPropHandle<T> = RcPropHandle<EdgeHandle, T>;
/// Reference-counted handle for a specific face property.
pub(crate) type RcFPropHandle<T> = RcPropHandle<FaceHandle, T>;


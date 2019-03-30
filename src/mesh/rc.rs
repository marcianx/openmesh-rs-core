//! See documentation for `RcPropHandle`.
use crate::mesh::item_handle::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle,
    MeshItemHandle,
};
use crate::mesh::mesh::Mesh;
use crate::mesh::status::Status;
use crate::property::traits::{self, PropertyFor};
use crate::property::PropertyContainer;
use crate::property::handle::PropHandle;
use crate::property::size::Size;
use crate::property::traits::Handle;   // For methods.

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

/// Request a property on the mesh if it doesn't already exist. It increases the ref count.
fn request_prop<H, T>(name: &'static str, props: &mut PropertyContainer<H>,
                      rc_handle: &mut RcPropHandle<H, T>, len: Size)
    where H: MeshItemHandle,
          T: traits::Value,
{
    rc_handle.ref_count += 1;
    if rc_handle.ref_count == 1 {
        debug_assert!(rc_handle.handle == Default::default());
        let name = H::with_prefix(name);
        rc_handle.handle = props.add::<T>(Some(name), len);
    }
}

/// Request a property on the mesh if it doesn't already exist. It increases the ref count.
fn release_prop<H, T>(props: &mut PropertyContainer<H>, rc_handle: &mut RcPropHandle<H, T>)
    where H: MeshItemHandle,
          T: traits::Value,
{
    if rc_handle.ref_count == 0 { return; }
    rc_handle.ref_count -= 1;
    if rc_handle.ref_count == 0 {
        props.remove::<T>(rc_handle.handle);
        rc_handle.handle.invalidate();
    }
}

macro_rules! def_prop_rc {
    ($Handle:ty, $props_field: ident, $rc_field:ident, $name:expr, $request_fn:ident,
     $release_fn:ident, $get_fn:ident, $get_fn_mut:ident) => {
        #[doc="Requests the corresponding property on the mesh if it doesn't already exist. It"]
        #[doc=" increases the ref count."]
        pub fn $request_fn(&mut self) {
            let size = self.props::<$Handle>().len();
            request_prop($name, &mut self.$props_field, &mut self.$rc_field, size);
        }

        #[doc="Releases the corresponding property on the mesh if it exists. If the ref count"]
        #[doc=" becomes 0, then it deallocates the property."]
        pub fn $release_fn(&mut self) {
            release_prop(&mut self.$props_field, &mut self.$rc_field);
        }

        #[doc="Gets the corresponding `Property` list if it exists."]
        pub fn $get_fn(&self) -> Option<&<Status as PropertyFor<$Handle>>::Property> {
            self.$props_field.get(self.$rc_field.handle)
        }
    };
}

impl Mesh {
    def_prop_rc!(VertexHandle, v_props, v_status, "status",
                 request_vertex_status, release_vertex_status,
                 get_vertex_status, get_vertex_status_mut);
    def_prop_rc!(HalfedgeHandle, h_props, h_status, "status",
                 request_halfedge_status, release_halfedge_status,
                 get_halfedge_status, get_halfedge_status_mut);
    def_prop_rc!(EdgeHandle, e_props, e_status, "status",
                 request_edge_status, release_edge_status,
                 get_edge_status, get_edge_status_mut);
    def_prop_rc!(FaceHandle, f_props, f_status, "status",
                 request_face_status, release_face_status,
                 get_face_status, get_face_status_mut);
}


/// Reference-counted handle for a specific vertex property.
pub(crate) type RcVPropHandle<T> = RcPropHandle<VertexHandle, T>;
/// Reference-counted handle for a specific halfedge property.
pub(crate) type RcHPropHandle<T> = RcPropHandle<HalfedgeHandle, T>;
/// Reference-counted handle for a specific edge property.
pub(crate) type RcEPropHandle<T> = RcPropHandle<EdgeHandle, T>;
/// Reference-counted handle for a specific face property.
pub(crate) type RcFPropHandle<T> = RcPropHandle<FaceHandle, T>;


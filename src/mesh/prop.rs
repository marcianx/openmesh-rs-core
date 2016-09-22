use std::ops::{Deref, DerefMut};
use mesh::handles::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle,
    PropHandle,
};
use property::{Property, PropertyContainer};
use property::size::Size;
use property::traits;

// Solely for trait methods.
use property::traits::PropHandle as PropHandleTrait;

////////////////////////////////////////////////////////////

/// Provides access to `Item` (vertex, halfedge, edge, face) properties.
///
/// It is returned by each of the following methods on `mesh::Mesh`:
///
/// - `mesh.vertices()`, `mesh.vertices_mut()`
/// - `mesh.halfedges()`, `mesh.halfedges_mut()`
/// - `mesh.edges()`, `mesh.edges_mut()`
/// - `mesh.faces()`, `mesh.faces_mut()`
/// - `mesh.mesh()`, `mesh.mesh_mut()`
pub struct Props<RefContainer> {
    props: RefContainer,
    len: Size,
}

#[allow(missing_docs)] pub type ItemProps<'a, Handle> = Props<&'a PropertyContainer<Handle>>;
#[allow(missing_docs)] pub type VProps<'a> = ItemProps<'a, VertexHandle>;
#[allow(missing_docs)] pub type HProps<'a> = ItemProps<'a, HalfedgeHandle>;
#[allow(missing_docs)] pub type EProps<'a> = ItemProps<'a, EdgeHandle>;
#[allow(missing_docs)] pub type FProps<'a> = ItemProps<'a, FaceHandle>;
#[allow(missing_docs)] pub type MProps<'a> = ItemProps<'a, MeshHandle>;
#[allow(missing_docs)] pub type ItemPropsMut<'a, Handle> = Props<&'a mut PropertyContainer<Handle>>;
#[allow(missing_docs)] pub type VPropsMut<'a> = ItemPropsMut<'a, VertexHandle>;
#[allow(missing_docs)] pub type HPropsMut<'a> = ItemPropsMut<'a, HalfedgeHandle>;
#[allow(missing_docs)] pub type EPropsMut<'a> = ItemPropsMut<'a, EdgeHandle>;
#[allow(missing_docs)] pub type FPropsMut<'a> = ItemPropsMut<'a, FaceHandle>;
#[allow(missing_docs)] pub type MPropsMut<'a> = ItemPropsMut<'a, MeshHandle>;


impl<Handle, RefContainer> Props<RefContainer>
    where Handle: traits::Handle,
          RefContainer: Deref<Target=PropertyContainer<Handle>>
{
    /// Instantiates an `Item` property interface struct.
    pub fn new(props: RefContainer, len: usize) -> Self {
        Props {
            props: props,
            len: len as Size,
        }
    }

    /// Number of elements of the given type.
    pub fn len(&self) -> Size { self.len }

    /// Returns the `Property<T>`, if any, corresponding to `prop_handle`.
    pub fn property<T: traits::Value>(&self, prop_handle: PropHandle<Handle, T>)
        -> Option<&Property<T, Handle>>
    {
        self.props.get(prop_handle.to_base())
    }
}

impl<Handle, RefContainer> Props<RefContainer>
    where Handle: traits::Handle,
          RefContainer: DerefMut<Target=PropertyContainer<Handle>>
{
    /// Adds a `Property<T>` for the associated item type (vertex, halfedge, edge, face, mesh).
    /// 
    /// Returns a valid property handle on success, or an invalid property handle on failure.
    /// 
    /// Constraints on `name`:
    /// 
    /// - Max length: 256 characters
    /// - Names matching `/^[vhefm]:/` or `/^<.*>$/` are reserved for internal use.
    pub fn add_property<T: traits::Value>(&mut self, name: Option<String>) -> PropHandle<Handle, T> {
        let prop_handle = self.props.add::<T>(name);
        PropHandle::<Handle, T>::from_base(prop_handle)
    }

    /// Removes a `Property<T>` for associated item type if `prop_handle` is valid, and it
    /// invalidates `prop_handle`.
    pub fn remove_property<T: traits::Value>(&mut self, prop_handle: &mut PropHandle<Handle, T>) {
        self.props.remove(prop_handle.to_base());
        prop_handle.invalidate();
    }

    /// Returns the `Property<T>`, if any, corresponding to `prop_handle`.
    pub fn property_mut<T: traits::Value>(&mut self, prop_handle: PropHandle<Handle, T>)
        -> Option<&mut Property<T, Handle>>
    {
        self.props.get_mut(prop_handle.to_base())
    }
}


// TODO from BaseKernel
// - Copy all properties from one item to another.
// - Stats for property (output stream or string). (Also add PropertyStats on Mesh itself.)
// - Property Iterator
//
// Should probably be in `Property`.
// - Access an item's property from `Property`.
// - Copy Property field from one item to another via static dispatch.
//
// // Only needed by non-native Kernel. Should be protected in original impl.
// - Number of properties.
// - Get `Property` trait object by name (mut and non-mut).
// - Get `Property` trait object by index or BasePropHandle (mut and non-mut).

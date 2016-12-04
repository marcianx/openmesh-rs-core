//! Defines a helper struct `Props` to add, remove, or access property lists (associated with
//! vectices, halfedge, edge, and faces) within a mesh.

use std::ops::{Deref, DerefMut};
use mesh::handles::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle,
    PropHandle,
};
use property::PropertyContainer;
use property::size::Size;
use property::traits::{self, PropertyFor};

// Solely for trait methods.
use property::traits::PropHandle as PropHandleTrait;
use property::traits::Property as PropertyTrait;

////////////////////////////////////////////////////////////

/// Provides access to item (vertex, halfedge, edge, face) properties.
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

/// For immutable access to property lists.
pub type ItemProps<'a, Handle> = Props<&'a PropertyContainer<Handle>>;
/// For immutable access to vertex properties.
pub type VProps<'a> = ItemProps<'a, VertexHandle>;
/// For immutable access to halfedge properties.
pub type HProps<'a> = ItemProps<'a, HalfedgeHandle>;
/// For immutable access to edge properties.
pub type EProps<'a> = ItemProps<'a, EdgeHandle>;
/// For immutable access to face properties.
pub type FProps<'a> = ItemProps<'a, FaceHandle>;
/// For immutable access to mesh properties.
pub type MProps<'a> = ItemProps<'a, MeshHandle>;
/// For mutable access to property lists.
pub type ItemPropsMut<'a, Handle> = Props<&'a mut PropertyContainer<Handle>>;
/// For mutable access to vertex properties.
pub type VPropsMut<'a> = ItemPropsMut<'a, VertexHandle>;
/// For mutable access to halfedge properties.
pub type HPropsMut<'a> = ItemPropsMut<'a, HalfedgeHandle>;
/// For mutable access to edge properties.
pub type EPropsMut<'a> = ItemPropsMut<'a, EdgeHandle>;
/// For mutable access to face properties.
pub type FPropsMut<'a> = ItemPropsMut<'a, FaceHandle>;
/// For mutable access to mesh properties.
pub type MPropsMut<'a> = ItemPropsMut<'a, MeshHandle>;


impl<Handle, RefContainer> Props<RefContainer>
    where Handle: traits::Handle,
          RefContainer: Deref<Target=PropertyContainer<Handle>>
{
    /// Instantiates an item property interface struct.
    pub fn new(props: RefContainer, len: Size) -> Self {
        Props {
            props: props,
            len: len,
        }
    }

    /// Number of elements of the given type.
    pub fn len(&self) -> Size { self.len }

    /// Returns the `Property<T>` or `PropertyBits` (for `T = bool`), if any, corresponding to
    /// `prop_handle`.
    pub fn property<T: traits::Value>(&self, prop_handle: PropHandle<Handle, T>)
        -> Option<&<T as PropertyFor<Handle>>::Property>
    {
        self.props.get::<T>(prop_handle.to_base())
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

    /// Returns the `Property<T>` or `PropertyBits` (for `T = bool`), if any, corresponding to
    /// `prop_handle`.
    pub fn property_mut<T: traits::Value>(&mut self, prop_handle: PropHandle<Handle, T>)
        -> Option<&mut <T as PropertyFor<Handle>>::Property>
    {
        self.props.get_mut::<T>(prop_handle.to_base())
    }

    /// Copies a single property from one item to another of the same type.
    /// It is a noop if any of the handles is invalid.
    pub fn copy_property<T: traits::Value>(
        &mut self, prop_handle: PropHandle<Handle, T>, h1: Handle, h2: Handle) {
        if h1.is_valid() && h2.is_valid() {
            self.property_mut(prop_handle).map(|p| p.copy(h1, h2));
        }
    }

    /// Copies all properties from one item to another of the same type.
    /// It is a noop if either handle is invalid.
    pub fn copy_all_properties<T: traits::Value>(&mut self, h_src: Handle, h_dst: Handle) {
        if h_src.is_valid() && h_dst.is_valid() {
            self.props.copy_all(h_src, h_dst);
        }
    }
}


impl<Handle, RefContainer> ::std::fmt::Debug for Props<RefContainer>
    where Handle: traits::Handle,
          RefContainer: Deref<Target=PropertyContainer<Handle>>
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.props.fmt(f)
    }
}


// TODO from BaseKernel
// - Property Iterator
//
// - See if `Property` trait ever needs to be exposed in the API.

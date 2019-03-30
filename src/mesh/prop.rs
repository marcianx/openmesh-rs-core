//! Defines a helper struct `Props` to add, remove, or access property lists (associated with
//! vectices, halfedge, edge, and faces) within a mesh.

use crate::mesh::item_handle::{VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle, MeshHandle};
use crate::property::PropertyContainer;
use crate::property::handle::PropHandle;
use crate::property::size::Size;
use crate::property::traits::{self, PropertyFor};
use crate::property::traits::Handle;   // For methods.

// Solely for trait methods.
use crate::property::traits::Property as PropertyTrait;

////////////////////////////////////////////////////////////

/// Provides immutable access to item (vertex, halfedge, edge, face) properties.
///
/// It is returned by each of the following methods on `mesh::Mesh`:
/// `mesh.vprops()`, `mesh.hprops()`, `mesh.eprops()`, `mesh.fprops()`,
/// `mesh.mprops()`.
pub struct Props<'a, H: traits::ItemHandle> {
    props: &'a PropertyContainer<H>,
    len: Size,
}

impl<'a, H> Props<'a, H>
    where H: traits::ItemHandle,
{
    /// Instantiates an item property interface struct.
    pub(crate) fn new(props: &'a PropertyContainer<H>, len: Size) -> Self {
        Props {
            props,
            len,
        }
    }
}

/// Provides mutable access to item (vertex, halfedge, edge, face) properties.
///
/// It is returned by each of the following methods on `mesh::Mesh`:
/// `mesh.vprops_mut()`, `mesh.hprops_mut()`, `mesh.eprops_mut()`,
/// `mesh.fprops_mut()`, `mesh.mprops_mut()`.
pub struct PropsMut<'a, H: traits::ItemHandle> {
    props: &'a mut PropertyContainer<H>,
    len: Size,
}

impl<'a, H> PropsMut<'a, H>
    where H: traits::ItemHandle,
{
    /// Instantiates an item property interface struct.
    pub(crate) fn new(props: &'a mut PropertyContainer<H>, len: Size) -> Self {
        PropsMut {
            props,
            len,
        }
    }
}

/// For immutable access to vertex properties.
pub type VProps<'a> = Props<'a, VertexHandle>;
/// For immutable access to halfedge properties.
pub type HProps<'a> = Props<'a, HalfedgeHandle>;
/// For immutable access to edge properties.
pub type EProps<'a> = Props<'a, EdgeHandle>;
/// For immutable access to face properties.
pub type FProps<'a> = Props<'a, FaceHandle>;
/// For immutable access to mesh properties.
pub type MProps<'a> = Props<'a, MeshHandle>;
/// For mutable access to vertex properties.
pub type VPropsMut<'a> = PropsMut<'a, VertexHandle>;
/// For mutable access to halfedge properties.
pub type HPropsMut<'a> = PropsMut<'a, HalfedgeHandle>;
/// For mutable access to edge properties.
pub type EPropsMut<'a> = PropsMut<'a, EdgeHandle>;
/// For mutable access to face properties.
pub type FPropsMut<'a> = PropsMut<'a, FaceHandle>;
/// For mutable access to mesh properties.
pub type MPropsMut<'a> = PropsMut<'a, MeshHandle>;


macro_rules! impl_props {
    ($Props:ident) => {
        impl<'a, H> $Props<'a, H>
            where H: traits::ItemHandle,
        {
            #[doc="Number of elements of the given type."]
            pub fn len(&self) -> Size { self.len }

            #[doc="Returns the handle with the given name if any exists and corresponds to a"]
            #[doc="property of type `T`. Otherwise, it returns an invalid handle."]
            pub fn handle<T: traits::Value>(&self, name: &str) -> PropHandle<H, T> {
                self.props.handle::<T>(name)
            }
        }
    }
}

impl_props!(Props);
impl_props!(PropsMut);

impl<'a, H> Props<'a, H>
    where H: traits::ItemHandle
{
    /// Returns the `Property<T>` or `PropertyBits` (for `T = bool`), if any,"]
    ///  corresponding to `prop_handle`.
    pub fn get<T: traits::Value>(&self, prop_handle: PropHandle<H, T>)
        -> Option<&'a <T as PropertyFor<H>>::Property>
    {
        self.props.get::<T>(prop_handle)
    }
}


impl<'a, H> PropsMut<'a, H>
    where H: traits::ItemHandle,
{
    /// Adds a `Property<T>` for the associated item type (vertex, halfedge, edge, face, mesh).
    /// 
    /// Returns a valid property handle on success, or an invalid property handle on failure.
    /// 
    /// Constraints on `name`:
    /// 
    /// - Max length: 256 characters
    /// - Names matching `/^[vhefm]:/` or `/^<.*>$/` are reserved for internal use.
    pub fn add<T: traits::Value>(&mut self, name: Option<String>) -> PropHandle<H, T> {
        self.props.add::<T>(name)
    }

    /// Removes a `Property<T>` for associated item type if `prop_handle` is valid, and it
    /// invalidates `prop_handle`.
    pub fn remove<T: traits::Value>(&mut self, prop_handle: &mut PropHandle<H, T>) {
        self.props.remove(*prop_handle);
        prop_handle.invalidate();
    }

    /// Returns the `Property<T>` or `PropertyBits` (for `T = bool`), if any, corresponding to
    /// `prop_handle`. This consumes `Self` since rust can't yet express re-borrows from existing
    /// mutable borrows within `Self`. https://users.rust-lang.org/t/22836
    pub fn get<T: traits::Value>(self, prop_handle: PropHandle<H, T>)
        -> Option<&'a <T as PropertyFor<H>>::Property>
    {
        self.props.get::<T>(prop_handle)
    }

    /// Returns the `Property<T>` or `PropertyBits` (for `T = bool`), if any, corresponding to
    /// `prop_handle`. This consumes `Self` since rust can't yet express re-borrows from existing
    /// mutable borrows within `Self`. https://users.rust-lang.org/t/22836
    pub fn get_mut<T: traits::Value>(self, prop_handle: PropHandle<H, T>)
        -> Option<&'a mut <T as PropertyFor<H>>::Property>
    {
        self.props.get_mut::<T>(prop_handle)
    }

    /// Copies a single property from one item to another of the same type.
    /// It is a noop if any of the handles is invalid.
    pub fn copy<T: traits::Value>(
        &mut self, prop_handle: PropHandle<H, T>, h_src: H, h_dst: H) {
        if h_src.is_valid() && h_dst.is_valid() {
            if let Some(p) = self.props.get_mut(prop_handle) {
                p.copy(h_src, h_dst);
            }
        }
    }

    /// Copies all properties from one item to another of the same type.
    /// It is a noop if either handle is invalid.
    pub fn copy_all<T: traits::Value>(&mut self, h_src: H, h_dst: H) {
        if h_src.is_valid() && h_dst.is_valid() {
            self.props.copy_all(h_src, h_dst);
        }
    }
}


impl<'a, H> ::std::fmt::Debug for Props<'a, H>
    where H: traits::ItemHandle,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.props.fmt(f)
    }
}


impl<'a, H> ::std::fmt::Debug for PropsMut<'a, H>
    where H: traits::ItemHandle,
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.props.fmt(f)
    }
}


// TODO from BaseKernel
// - Property Iterator
//
// - See if `Property` trait ever needs to be exposed in the API.

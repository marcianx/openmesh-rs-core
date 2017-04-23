//! Defines the core primitives (`Vertex`, `Halfedge`, `Edge`, `Face`) encoding mesh connectivity,
//! and operations on collections of these items.

use mesh::handles::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle,
};
use mesh::prop::{Props, PropsMut};
use property::PropertyContainer;
use property::traits::{self, Handle as HandleTrait};
use property::size::Size;

////////////////////////////////////////////////////////////
// Half-edge data structure connectivity fields

/// Vertex fields for HDS topology.
#[derive(Clone, Default)]
pub struct Vertex {
    /// An outgoing halfedge, if any, from this vertex.
    pub hh: HalfedgeHandle,
}

/// Halfedge fields for HDS topology.
#[derive(Clone, Default)]
pub struct Halfedge {
    /// The face, if any, to which this halfedge belongs.
    pub fh: FaceHandle,
    /// The vertex this halfedge points to.
    pub vh: VertexHandle,
    /// The next halfedge going counter-clockwise around the face.
    pub hnext: HalfedgeHandle,
    /// The previous halfedge - i.e. the next one going clockwise around the face.
    pub hprev: HalfedgeHandle,
}

/// Edge fields for HDS topology.
#[derive(Default)]
pub struct Edge {
    /// The pair of halfedges constituting the edge.
    /// IMPORTANT: For the purpose of computing handles from a provided halfedge reference,
    /// it is assumed, that &self == &self.halfedge[0] (in terms of pointer comparison).
    pub halfedges: [Halfedge; 2],
}
// Explicitly implement `Clone` to avoid requiring `Copy` on `Halfedge`.
impl Clone for Edge {
    fn clone(&self) -> Self {
        Edge {
            halfedges: [self.halfedges[0].clone(), self.halfedges[1].clone()]
        }
    }
}

/// Face fields for HDS topology.
#[derive(Clone, Default)]
pub struct Face {
    /// A halfedge bounding this face.
    pub hh: HalfedgeHandle,
}

////////////////////////////////////////////////////////////////////////////////
// For mapping between item type and its corresponding handle type.

/// Trait to map a mesh item handle to its corresponding mesh item.
pub trait MeshItemFor: traits::Handle {
    /// Mesh item type corresponding to `Self` which is one of `Vertex`, `Halfedge`, `Edge`, or
    /// `Face`.
    type Item: Default;
    /// Storage item type containing `Self::Item`. Specifically, `Vertex`/`Edge`/`Face` is stored
    /// as itself, but each `Halfedge` is stored in an `Edge`.
    type ContainerItem: Default;
}
impl MeshItemFor for VertexHandle   { type Item = Vertex;   type ContainerItem = Vertex; }
impl MeshItemFor for HalfedgeHandle { type Item = Halfedge; type ContainerItem = Edge; }
impl MeshItemFor for EdgeHandle     { type Item = Edge;     type ContainerItem = Edge; }
impl MeshItemFor for FaceHandle     { type Item = Face;     type ContainerItem = Face; }

////////////////////////////////////////////////////////////////////////////////
// For accessing each item type from the mesh connectivity.

/// Captures the differences between how `Vertex`/`Edge`/`Face` are stored and how `Halfedge` is
/// stored and retrieved.
pub trait MeshMeta: MeshItemFor {
    /// Number of items of type `Self` in the underlying storage vector.
    fn len(vec: &Vec<Self::ContainerItem>) -> usize;
    /// Gets item of type `Self` from the underlying storage vector.
    fn get(vec: &Vec<Self::ContainerItem>, handle: Self) -> Option<&Self::Item>;
    /// Gets item of type `Self` mutably from the underlying storage vector.
    fn get_mut(vec: &mut Vec<Self::ContainerItem>, handle: Self) -> Option<&mut Self::Item>;
}

macro_rules! impl_default_mesh_item {
    ($Handle:ty) => {
        impl MeshMeta for $Handle {
            fn len(vec: &Vec<Self::ContainerItem>) -> usize { vec.len() }
            fn get(vec: &Vec<Self::ContainerItem>, handle: Self) -> Option<&Self::Item> {
                vec.get(handle.index_us())
            }
            fn get_mut(vec: &mut Vec<Self::ContainerItem>, handle: Self) -> Option<&mut Self::Item> {
                vec.get_mut(handle.index_us())
            }
        }
    }
}
impl_default_mesh_item!(VertexHandle);
impl_default_mesh_item!(EdgeHandle);
impl_default_mesh_item!(FaceHandle);

impl MeshMeta for HalfedgeHandle {
    fn len(vec: &Vec<Self::ContainerItem>) -> usize {
        debug_assert!(vec.len() <= usize::max_value() / 2);
        vec.len() * 2
    }
    fn get(vec: &Vec<Self::ContainerItem>, handle: Self) -> Option<&Self::Item> {
        let index = handle.index_us();
        vec.get(index / 2).map(|edge| &edge.halfedges[index % 2])
    }
    fn get_mut(vec: &mut Vec<Self::ContainerItem>, handle: Self) -> Option<&mut Self::Item> {
        let index = handle.index_us();
        vec.get_mut(index / 2).map(|edge| &mut edge.halfedges[index % 2])
    }
}

////////////////////////////////////////////////////////////

pub(crate) type ContainerVec<Handle> = Vec<<Handle as MeshItemFor>::ContainerItem>;

/// Manages immutable operations on the list of a particular mesh item type.
/// These are created by `Mesh`'s methods:
/// `vertices()`, `halfedges()`, `edges()`, `faces()`.
pub struct Items<'a, Handle: MeshItemFor> {
    /// Item connectivity.
    items: &'a ContainerVec<Handle>,
    /// Item properties.
    props: &'a PropertyContainer<Handle>,
    _marker: ::std::marker::PhantomData<Handle>,
}

impl<'a, Handle: MeshItemFor> Items<'a, Handle> {
    /// Instantiates an item + property interface struct.
    pub(crate) fn new(items: &'a ContainerVec<Handle>, props: &'a PropertyContainer<Handle>) -> Self
    {
        Items {
            items: items,
            props: props,
            _marker: ::std::marker::PhantomData,
        }
    }
}


/// Manages immutable and mutable operations on the list of a particular mesh item type.
/// These are created by `Mesh`'s methods:
/// `vertices_mut()`, `halfedges_mut()`, `edges_mut()`, `faces_mut()`.
pub struct ItemsMut<'a, Handle: MeshItemFor> {
    /// Item connectivity.
    items: &'a mut ContainerVec<Handle>,
    /// Item properties.
    props: &'a mut PropertyContainer<Handle>,
    _marker: ::std::marker::PhantomData<Handle>,
}

impl<'a, Handle: MeshItemFor> ItemsMut<'a, Handle> {
    /// Instantiates an item + property mutable interface struct.
    pub(crate) fn new(items: &'a mut ContainerVec<Handle>, props: &'a mut PropertyContainer<Handle>) -> Self
    {
        ItemsMut {
            items: items,
            props: props,
            _marker: ::std::marker::PhantomData,
        }
    }
}

/// Immutable access to vertex items encapulating mesh connectivity.
pub type VItems<'a> = Items<'a, VertexHandle>;
/// Immutable access to halfedge items encapulating mesh connectivity.
pub type HItems<'a> = Items<'a, HalfedgeHandle>;
/// Immutable access to edge items encapulating mesh connectivity.
pub type EItems<'a> = Items<'a, EdgeHandle>;
/// Immutable access to face items encapulating mesh connectivity.
pub type FItems<'a> = Items<'a, FaceHandle>;
/// Mutable access to vertex items encapulating mesh connectivity.
pub type VItemsMut<'a> = ItemsMut<'a, VertexHandle>;
/// Mutable access to halfedge items encapulating mesh connectivity.
pub type HItemsMut<'a> = ItemsMut<'a, HalfedgeHandle>;
/// Mutable access to edge items encapulating mesh connectivity.
pub type EItemsMut<'a> = ItemsMut<'a, EdgeHandle>;
/// Mutable access to face items encapulating mesh connectivity.
pub type FItemsMut<'a> = ItemsMut<'a, FaceHandle>;

macro_rules! impl_items {
    ($Items:ident) => {
        // Methods with immutable self for both `Items` and `ItemsMut`.
        impl<'a, Handle> $Items<'a, Handle>
            where Handle: MeshMeta,
        {
            #[doc="Number of items of the given item type."]
            fn len_us(&self) -> usize { <Handle as MeshMeta>::len(&self.items) }

            #[doc="Number of items of the given item type."]
            pub fn len(&self) -> Size {
                debug_assert!(self.len_us() <= Size::max_value() as usize);
                self.len_us() as Size
            }

            #[doc="Whether the handle is within the range of the underlying container."]
            #[doc="Even if valid, the handle could pointed to a deleted item."]
            #[doc="This method is useful mostly for debugging."]
            pub fn is_valid(&self, handle: Handle) -> bool {
                let idx = handle.index();

                // In case index is ever changed to a signed type, also check against 0.
                #[allow(unused_comparisons)] // Requires explicit return to turn next line into a statement.
                return 0 <= idx && idx < self.len();
            }

            #[doc="Computes the `Handle` from the given `Item` reference. The `Item`"]
            #[doc="must be from the mesh from which `self` was generated."]
            pub fn handle(&self, item: &Handle::Item) -> Handle {
                debug_assert!(0 < self.len());
                let diff =
                    (item as *const Handle::Item as isize) -
                    (&self.items[0] as *const Handle::ContainerItem as isize);
                let size_of_item = ::std::mem::size_of::<Handle::Item>() as isize;
                debug_assert!(diff % size_of_item == 0);
                let index = diff / size_of_item;
                assert!(0 <= index && index < self.len_us() as isize);
                Handle::from_index(index as Size)
            }

            #[doc="Gets the item at the handle."]
            pub fn get(&self, handle: Handle) -> Option<&Handle::Item> {
                <Handle as MeshMeta>::get(self.items, handle)
            }

            #[doc="Returns the properties container associated with the mesh item type."]
            pub fn props(&self) -> Props<Handle> {
                Props::new(self.props, self.len())
            }

            // TODO
            // - empty() method
        }
    }
}

impl_items!(Items);
impl_items!(ItemsMut);


// Methods for mutable self.
impl<'a, Handle> ItemsMut<'a, Handle>
    where Handle: MeshMeta,
{
    /// Gets the mutable item at the handle.
    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut Handle::Item> {
        <Handle as MeshMeta>::get_mut(&mut self.items, handle)
    }

    /// Returns the mutable properties container associated with the mesh item type.
    pub fn props_mut(&mut self) -> PropsMut<Handle> {
        let len = self.len();
        PropsMut::new(&mut self.props, len)
    }
}


// Only applies to mesh items used for storage. In particular, it doesn't apply to `Halfedge`.
impl<'a, Handle> ItemsMut<'a, Handle>
    where Handle: MeshMeta,
{
    /// Adds a new item and returns it.
    /// NOTE
    /// - This cannot be exposed in the public API: the resizing must be done in concert with the
    ///   property lists.
    /// - This thus does not check for overflow of `Size`.
    /// TODO: Could also generalize this method to also append to the property lists, but then, the
    ///     `Edge` version has to update both the `Edge` and the `Halfedge` property lists.
    pub(crate) fn append(&mut self) -> &mut Handle::ContainerItem {
        self.items.push(Default::default());
        let last_idx = self.items.len() - 1;
        unsafe { self.items.get_unchecked_mut(last_idx) }
        // TODO: Return index like new_* methods in OpenMesh?
    }
}

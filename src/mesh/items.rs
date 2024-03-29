//! Defines the core primitives (`Vertex`, `Halfedge`, `Edge`, `Face`) encoding mesh connectivity,
//! and operations on collections of these items.

use crate::mesh::item_handle::{
    EdgeHandle, FaceHandle, HalfedgeHandle, MeshItemHandle, VertexHandle,
};
use crate::mesh::prop::{Props, PropsMut};
use crate::property::PropertyContainer;
use crate::property::Size;

////////////////////////////////////////////////////////////
// Half-edge data structure connectivity fields

/// Vertex fields for HDS topology.
#[derive(Clone, Default)]
pub struct Vertex {
    /// An outgoing halfedge, if any, from this vertex.
    pub(crate) hh: HalfedgeHandle,
}

/// Halfedge fields for HDS topology.
#[derive(Clone, Default)]
pub struct Halfedge {
    /// The face, if any, to which this halfedge belongs.
    pub(crate) fh: FaceHandle,
    /// The vertex this halfedge points to.
    pub(crate) vh: VertexHandle,
    /// The next halfedge going counter-clockwise around the face.
    pub(crate) hnext: HalfedgeHandle,
    /// The previous halfedge - i.e. the next one going clockwise around the face.
    pub(crate) hprev: HalfedgeHandle,
}

/// Edge fields for HDS topology.
#[derive(Clone, Default)]
/// The pair of halfedges constituting the edge.
/// IMPORTANT: For the purpose of computing handles from a provided halfedge reference,
/// it is assumed, that &self == &self.halfedge[0] (in terms of pointer comparison).
pub struct Edge(pub(crate) [Halfedge; 2]);

/// Face fields for HDS topology.
#[derive(Clone, Default)]
pub struct Face {
    /// A halfedge bounding this face.
    pub hh: HalfedgeHandle,
}

////////////////////////////////////////////////////////////

type ContainerSlice<H> = [<H as MeshItemHandle>::ContainerItem];
type ContainerVec<H> = Vec<<H as MeshItemHandle>::ContainerItem>;

/// Manages immutable operations on the list of a particular mesh item type.
/// These are created by `Mesh`'s methods:
/// `vertices()`, `halfedges()`, `edges()`, `faces()`.
pub struct Items<'a, H: MeshItemHandle> {
    /// Item connectivity.
    items: &'a ContainerSlice<H>,
    /// Item properties.
    props: &'a PropertyContainer<H>,
    _marker: ::std::marker::PhantomData<H>,
}

impl<'a, H: MeshItemHandle> Items<'a, H> {
    /// Instantiates an item + property interface struct.
    pub(crate) fn new(items: &'a ContainerSlice<H>, props: &'a PropertyContainer<H>) -> Self {
        Items {
            items,
            props,
            _marker: ::std::marker::PhantomData,
        }
    }
}

/// Manages immutable and mutable operations on the list of a particular mesh item type.
/// These are created by `Mesh`'s methods:
/// `vertices_mut()`, `halfedges_mut()`, `edges_mut()`, `faces_mut()`.
/// Addition and removal methods are not implemented on `ItemsMut<'a, HalfedgeHandle>` since
/// halfedges belong to edges which must be added/removed instead.
pub struct ItemsMut<'a, H: MeshItemHandle> {
    /// Item connectivity.
    items: &'a mut ContainerVec<H>,
    /// Item properties.
    props: &'a mut PropertyContainer<H>,
    _marker: ::std::marker::PhantomData<H>,
}

impl<'a, H: MeshItemHandle> ItemsMut<'a, H> {
    /// Instantiates an item + property mutable interface struct.
    pub(crate) fn new(items: &'a mut ContainerVec<H>, props: &'a mut PropertyContainer<H>) -> Self {
        ItemsMut {
            items,
            props,
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
        impl<'a, H> $Items<'a, H>
        where
            H: MeshItemHandle,
        {
            #[doc = "Number of items of the item type."]
            fn len_us(&self) -> usize {
                <H as MeshItemHandle>::num_items(&self.items)
            }

            #[doc = "Number of items of the item type."]
            pub fn len(&self) -> Size {
                debug_assert!(self.len_us() <= Size::max_value() as usize);
                self.len_us() as Size
            }

            #[doc = "Whether the handle is within the range of the underlying container."]
            #[doc = "Even if valid, the handle could pointed to a deleted item."]
            #[doc = "This method is useful mostly for debugging."]
            pub fn is_valid(&self, handle: H) -> bool {
                let idx = handle.index();

                // In case index is ever changed to a signed type, also check against 0.
                #[allow(unused_comparisons)]
                // Requires explicit return to turn next line into a statement.
                return 0 <= idx && idx < self.len();
            }

            #[doc = "Computes the handle from the given `Item` reference. The `Item`"]
            #[doc = "must be from the mesh from which `self` was generated."]
            pub fn handle(&self, item: &H::Item) -> H {
                debug_assert!(0 < self.len());
                let diff = (item as *const H::Item as isize)
                    - (&self.items[0] as *const H::ContainerItem as isize);
                let size_of_item = ::std::mem::size_of::<H::Item>() as isize;
                debug_assert!(diff % size_of_item == 0);
                let index = diff / size_of_item;
                assert!(0 <= index && index < self.len_us() as isize);
                H::from_index(index as Size)
            }

            #[doc = "Gets the item at the handle."]
            pub fn get(&self, handle: H) -> Option<&H::Item> {
                <H as MeshItemHandle>::get(self.items, handle)
            }

            #[doc = "Returns the properties container associated with the mesh item type."]
            pub fn props(&self) -> Props<'_, H> {
                Props::new(self.props, self.len())
            }

            #[doc = "Returns the properties container associated with the mesh item type."]
            pub fn into_props(self) -> Props<'a, H> {
                Props::new(self.props, self.len())
            }

            #[doc = "Returns true when there are no items of the mesh item type."]
            pub fn empty(&self) -> bool {
                self.len_us() == 0
            }
        }
    };
}

impl_items!(Items);
impl_items!(ItemsMut);

// Methods for mutable self.
impl<'a, H> ItemsMut<'a, H>
where
    H: MeshItemHandle,
{
    /// Gets the mutable item at the handle.
    pub fn get_mut(&mut self, handle: H) -> Option<&mut H::Item> {
        <H as MeshItemHandle>::get_mut(&mut self.items, handle)
    }

    /// Returns the mutable properties container associated with the mesh item type.
    pub fn props_mut(&mut self) -> PropsMut<'_, H> {
        let len = self.len();
        PropsMut::new(&mut self.props, len)
    }

    /// Returns the mutable properties container associated with the mesh item type.
    pub fn into_props_mut(self) -> PropsMut<'a, H> {
        let len = self.len();
        PropsMut::new(self.props, len)
    }
}

// Only applies to mesh items used for storage. In particular, it doesn't apply to `Halfedge`.
impl<'a, H> ItemsMut<'a, H>
where
    H: MeshItemHandle<Item = <H as MeshItemHandle>::ContainerItem>,
{
    /// Adds/appends a new item and returns it.
    ///
    /// TODO: Reconsider the privacy of this method.
    /// - Should this just be a helper for higher-level append operations?
    /// - Should it be exposed publicly to allow for more advanced mesh constructions?
    pub fn append(&mut self) -> H {
        let old_len = self.items.len();
        assert!(
            old_len < Size::max_value() as usize,
            "Cannot add more than {} items. Already have {}.",
            Size::max_value(),
            old_len
        );
        let old_len = old_len as Size;
        self.items.push(Default::default());
        self.props.push_all();
        H::from_index(old_len)
    }

    /// Reserves space for `n` items.
    ///
    /// NOTE: This behaves differently than `Vec::reserve` which takes the additional amount to
    /// reserve as opposed to the total amount to reserve.
    /// `Mesh::reserve()` calls this.
    pub(crate) fn reserve(&mut self, n: Size) {
        self.props.reserve_all(n);
        if (n as usize) < self.items.len() {
            let additional = n as usize - self.items.len();
            self.items.reserve(additional);
        }
    }

    // NOTE: None of the methods below can be part of the public API since removing items of one
    // item type without regard to the other item types could result in dangling handles which can
    // be confusing.

    /// Resizes to having exactly `n` items.
    pub(crate) fn resize(&mut self, n: Size) {
        self.props.resize_all(n);
        self.items.resize(n as usize, Default::default());
    }

    /// Resizes to having exactly `n` items.
    pub(crate) fn clear(&mut self) {
        self.props.clear_all();
        self.items.clear();
    }
}

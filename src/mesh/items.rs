//! Defines the core primitives (`Vertex`, `Halfedge`, `Edge`, `Face`) encoding mesh connectivity,
//! and operations on collections of these items.

use std::ops::{Deref, DerefMut};
use mesh::handles::{
    VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle,
};
use mesh::prop::{ItemProps, ItemPropsMut};
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
pub trait MeshItemFor {
    /// Mesh item type corresponding to `Self` which is one of `Vertex`, `Halfedge`, `Edge`, or
    /// `Face`.
    type Item: MeshItem + MeshHandleFor;
}
impl MeshItemFor for VertexHandle   { type Item = Vertex; }
impl MeshItemFor for HalfedgeHandle { type Item = Halfedge; }
impl MeshItemFor for EdgeHandle     { type Item = Edge; }
impl MeshItemFor for FaceHandle     { type Item = Face; }
/// Maps an `Item` implementing `MeshHandleFor` to its handle type.
pub type ItemForHandle<Handle> = <Handle as MeshItemFor>::Item;


/// Trait to map a mesh item to its corresponding mesh handle.
pub trait MeshHandleFor {
    /// Mesh handle type corresponding to `Self` which is one of `Vertex`, `Halfedge`, `Edge`, or
    /// `Face`.
    type Handle: traits::Handle + MeshItemFor;
}
impl MeshHandleFor for Vertex   { type Handle = VertexHandle; }
impl MeshHandleFor for Halfedge { type Handle = HalfedgeHandle; }
impl MeshHandleFor for Edge     { type Handle = EdgeHandle; }
impl MeshHandleFor for Face     { type Handle = FaceHandle; }
/// Maps an `Item` implementing `MeshHandleFor` to its handle type.
pub type HandleForItem<Item> = <Item as MeshHandleFor>::Handle;

////////////////////////////////////////////////////////////////////////////////
// For accessing each item type from the mesh connectivity.

/// Captures the differences between how `Vertex`/`Edge`/`Face` are stored and how `Halfedge` is
/// stored for the purpose of implementing `Items` for each type.
pub trait MeshItem: Default + MeshHandleFor {
    /// Storage item type containing `Self`. Specifically, `Vertex`/`Edge`/`Face` is stored
    /// as itself, but each `Halfedge` is stored in an `Edge`.
    type ContainerItem;
    /// Number of items of type `Self` in the underlying storage vector.
    fn len(vec: &Vec<Self::ContainerItem>) -> usize;
    /// Gets item of type `Self` from the underlying storage vector.
    fn get(vec: &Vec<Self::ContainerItem>, handle: HandleForItem<Self>) -> Option<&Self>;
    /// Gets item of type `Self` mutably from the underlying storage vector.
    fn get_mut(vec: &mut Vec<Self::ContainerItem>, handle: HandleForItem<Self>) -> Option<&mut Self>;
}
/// Maps a mesh item to the mesh item actually represented for storage. Specifically,
/// `Vertex`/`Edge`/`Face` is stored as itself, but each `Halfedge` is stored in an `Edge`.
pub type ContainerItem<Item> = <Item as MeshItem>::ContainerItem;

macro_rules! impl_default_mesh_item {
    ($Item:ty) => {
        impl MeshItem for $Item {
            type ContainerItem = Self;
            fn len(vec: &Vec<Self::ContainerItem>) -> usize { vec.len() }
            fn get(vec: &Vec<Self::ContainerItem>, handle: HandleForItem<Self>) -> Option<&Self> {
                vec.get(handle.index_us())
            }
            fn get_mut(vec: &mut Vec<Self::ContainerItem>, handle: HandleForItem<Self>) -> Option<&mut Self> {
                vec.get_mut(handle.index_us())
            }
        }
    }
}
impl_default_mesh_item!(Vertex);
impl_default_mesh_item!(Edge);
impl_default_mesh_item!(Face);

impl MeshItem for Halfedge {
    type ContainerItem = Edge;
    fn len(vec: &Vec<Self::ContainerItem>) -> usize {
        debug_assert!(vec.len() <= usize::max_value() / 2);
        vec.len() * 2
    }
    fn get(vec: &Vec<Self::ContainerItem>, handle: HandleForItem<Self>) -> Option<&Self> {
        let index = handle.index_us();
        vec.get(index / 2).map(|edge| &edge.halfedges[index % 2])
    }
    fn get_mut(vec: &mut Vec<Self::ContainerItem>, handle: HandleForItem<Self>) -> Option<&mut Self> {
        let index = handle.index_us();
        vec.get_mut(index / 2).map(|edge| &mut edge.halfedges[index % 2])
    }
}

////////////////////////////////////////////////////////////

/// Manages operations on the list of a particular mesh item type.
/// These are created by `Mesh`'s methods: `vertices()`, `halfedges()`, `edges()`, `faces()`.
pub struct Items<Item, Handle, RefContainer, RefPropContainer> {
    // Item connectivity and properties.
    items: RefContainer,
    props: RefPropContainer,
    _marker: ::std::marker::PhantomData<(Item, Handle)>,
}

/// For immutable access to item lists and their properties.
pub type ItemsWithProps<'a, Item, Handle> =
    Items<
        Item, Handle,
        &'a Vec<ContainerItem<Item>>,
        &'a PropertyContainer<Handle>>;
/// Immutable access to vertex items encapulating mesh connectivity.
pub type VItems<'a> = ItemsWithProps<'a, Vertex, VertexHandle>;
/// Immutable access to halfedge items encapulating mesh connectivity.
pub type HItems<'a> = ItemsWithProps<'a, Halfedge, HalfedgeHandle>;
/// Immutable access to edge items encapulating mesh connectivity.
pub type EItems<'a> = ItemsWithProps<'a, Edge, EdgeHandle>;
/// Immutable access to face items encapulating mesh connectivity.
pub type FItems<'a> = ItemsWithProps<'a, Face, FaceHandle>;
/// For mutable access to item lists and their properties.
pub type ItemsWithPropsMut<'a, Item, Handle> =
    Items<
        Item, Handle,
        &'a mut Vec<ContainerItem<Item>>,
        &'a mut PropertyContainer<Handle>>;
/// Mutable access to vertex items encapulating mesh connectivity.
pub type VItemsMut<'a> = ItemsWithPropsMut<'a, Vertex, VertexHandle>;
/// Mutable access to halfedge items encapulating mesh connectivity.
pub type HItemsMut<'a> = ItemsWithPropsMut<'a, Halfedge, HalfedgeHandle>;
/// Mutable access to edge items encapulating mesh connectivity.
pub type EItemsMut<'a> = ItemsWithPropsMut<'a, Edge, EdgeHandle>;
/// Mutable access to face items encapulating mesh connectivity.
pub type FItemsMut<'a> = ItemsWithPropsMut<'a, Face, FaceHandle>;

// Methods with immutable self for both `ItemsWithProps` and `ItemsWithPropsMut`.
impl<Item, Handle, RefContainer, RefPropContainer>
    Items<Item, Handle, RefContainer, RefPropContainer>
    where Item: MeshItem + MeshHandleFor<Handle=Handle>,
          Handle: traits::Handle + MeshItemFor<Item=Item>,
          RefContainer: Deref<Target=Vec<ContainerItem<Item>>>,
          RefPropContainer: Deref<Target=PropertyContainer<Handle>>,
{
    /// Instantiates an item property interface struct.
    pub(crate) fn new(items: RefContainer, props: RefPropContainer) -> Self {
        Items {
            items: items,
            props: props,
            _marker: ::std::marker::PhantomData,
        }
    }

    /// Number of items of the given item type.
    fn len_us(&self) -> usize { <Item as MeshItem>::len(&self.items) }

    /// Number of items of the given item type.
    pub fn len(&self) -> Size {
        debug_assert!(self.len_us() <= Size::max_value() as usize);
        self.len_us() as Size
    }

    /// Whether the handle is within the range of the underlying container.
    /// Even if valid, the handle could pointed to a deleted item.
    /// This method is useful mostly for debugging.
    pub fn is_valid(&self, handle: Handle) -> bool {
        let idx = handle.index();

        // In case index is ever changed to a signed type, also check against 0.
        #[allow(unused_comparisons)] // Requires explicit return to turn next line into a statement.
        return 0 <= idx && idx < self.len();
    }

    /// Computes the `HandleForItem` from the given `Item` reference. The `Item` must be from the mesh
    /// from which `self` was generated.
    pub fn handle(&self, item: &Item) -> Handle {
        debug_assert!(0 < self.len());
        let diff =
            (item as *const Item as isize) -
            (&self.items[0] as *const ContainerItem<Item> as isize);
        let size_of_item = ::std::mem::size_of::<Item>() as isize;
        debug_assert!(diff % size_of_item == 0);
        let index = diff / size_of_item;
        assert!(0 <= index && index < self.len_us() as isize);
        Handle::from_index(index as Size)
    }

    /// Gets the item at the handle.
    pub fn get(&self, handle: Handle) -> Option<&Item> {
        <Item as MeshItem>::get(self.items.deref(), handle)
    }

    /// Returns the properties container associated with the mesh item type.
    pub fn props(&self) -> ItemProps<Handle> {
        ItemProps::new(self.props.deref(), self.len())
    }
}


// Methods for mutable self.
impl<'a, Item, Handle> ItemsWithPropsMut<'a, Item, Handle>
    where Item: MeshItem + MeshHandleFor<Handle=Handle>,
          Handle: traits::Handle + MeshItemFor<Item=Item>,
{
    /// Gets the mutable item at the handle.
    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut Item> {
        <Item as MeshItem>::get_mut(&mut self.items, handle)
    }

    /// Returns the mutable properties container associated with the mesh item type.
    pub fn props_mut(&mut self) -> ItemPropsMut<Handle> {
        let len = self.len();
        ItemPropsMut::new(&mut self.props, len)
    }
}


// Only applies to mesh items used for storage. In particular, it doesn't apply to `Halfedge`.
impl<'a, Item, Handle> ItemsWithPropsMut<'a, Item, Handle>
    where Item: MeshItem<ContainerItem=Item> + MeshHandleFor<Handle=Handle>,
          Handle: traits::Handle + MeshItemFor<Item=Item>,
{
    /// Adds a new item and returns it.
    /// NOTE
    /// - This cannot be exposed in the public API: the resizing must be done in concert with the
    ///   property lists.
    /// - This thus does not check for overflow of `Size`.
    /// TODO: Could also generalize this method to also append to the property lists, but then, the
    ///     `Edge` version has to update both the `Edge` and the `Halfedge` property lists.
    pub(crate) fn append(&mut self) -> &mut Item {
        self.items.push(Default::default());
        let last_idx = self.items.deref().len() - 1;
        unsafe { self.items.deref_mut().get_unchecked_mut(last_idx) }
    }
}

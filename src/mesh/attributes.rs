use bitflags;

#[doc(hidden)]
pub(crate) type FlagBits = u32;

// TODO: Check if usable in this form to be able to select primitives at compile time.
bitflags! {
    #[doc = "Used to define a standard property at compile time."]
    pub struct Attributes: FlagBits {
        #[doc = "Clear all attribute bits"]
        const NONE          = 0;
        #[doc = "Add normals to mesh item (vertices/faces)"]
        const NORMAL        = 1;
        #[doc = "Add colors to mesh item (vertices/faces/edges)"]
        const COLOR         = 2;
        #[doc = "Add storage for previous halfedge (halfedges). The bit is set by default in the DefaultTraits."]
        const PREV_HALFEDGE = 4;
        #[doc = "Add status to mesh item (all items)"]
        const STATUS        = 8;
        #[doc = "Add 1D texture coordinates (vertices, halfedges)"]
        const TEX_COORD_1D  = 16;
        #[doc = "Add 2D texture coordinates (vertices, halfedges)"]
        const TEX_COORD_2D  = 32;
        #[doc = "Add 3D texture coordinates (vertices, halfedges)"]
        const TEX_COORD_3D  = 64;
        #[doc = "Add texture index (faces)"]
        const TEXTURE_INDEX = 128;
    }
}

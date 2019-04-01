use bitflags;

// TODO: Check if usable in this form to be able to select primitives at compile time.
bitflags! {
    #[doc = "Used to define a standard property at compile time."]
    flags Attributes: u32 {
        #[doc = "Clear all attribute bits"]
        const None          = 0,  
        #[doc = "Add normals to mesh item (vertices/faces)"]
        const Normal        = 1,  
        #[doc = "Add colors to mesh item (vertices/faces/edges)"]
        const Color         = 2,  
        #[doc = "Add storage for previous halfedge (halfedges). The bit is set by default in the DefaultTraits."]
        const PrevHalfedge  = 4,  
        #[doc = "Add status to mesh item (all items)"]
        const Status        = 8,  
        #[doc = "Add 1D texture coordinates (vertices, halfedges)"]
        const TexCoord1D    = 16, 
        #[doc = "Add 2D texture coordinates (vertices, halfedges)"]
        const TexCoord2D    = 32, 
        #[doc = "Add 3D texture coordinates (vertices, halfedges)"]
        const TexCoord3D    = 64, 
        #[doc = "Add texture index (faces)"]
        const TextureIndex  = 128 
    }
}


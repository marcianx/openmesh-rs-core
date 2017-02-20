// TODO: This file is not currently being used.
use property::traits::Handle;
use geometry::vector::Vec3d;
use mesh::attributes::Attributes;
use mesh::handles::{VertexHandle, HalfedgeHandle, EdgeHandle, FaceHandle};

// TODO: WHERE DOES THIS BELONG?
/// Trait for collections of types needed to define a mesh.
pub trait MeshTrait {
    type Point;             //typename Traits::Point                           
    type Scalar;            //typename vector_traits<Point>::value_type                         
    type Normal;            //typename Traits::Normal                           
    type Color;             //typename Traits::Color                           
    type TexCoord1D;        //typename Traits::TexCoord1D                           
    type TexCoord2D;        //typename Traits::TexCoord2D                           
    type TexCoord3D;        //typename Traits::TexCoord3D                           
    type TextureIndex;      //typename Traits::TextureIndex                           

    const VertexAttributes: Attributes;
    const HalfedgeAttributes: Attributes;
    const EdgeAttributes: Attributes;
    const FaceAttributes: Attributes;
}


// TODO: WHERE DOES THIS BELONG? VertexT, HalfedgeT, EdgeT, FaceT need to depend on it.
/// Trait for collections of types needed to define a mesh.
pub trait MeshRefs {
    type Point;             //typename Traits::Point                           
    type Scalar;            //typename vector_traits<Point>::value_type                         
    type Normal;            //typename Traits::Normal                           
    type Color;             //typename Traits::Color                           
    type TexCoord1D;        //typename Traits::TexCoord1D                           
    type TexCoord2D;        //typename Traits::TexCoord2D                           
    type TexCoord3D;        //typename Traits::TexCoord3D                           
    type TextureIndex;      //typename Traits::TextureIndex                           
    type VertexHandle: Handle;
    type FaceHandle: Handle;
    type EdgeHandle: Handle;
    type HalfedgeHandle: Handle;
}


/// Collection of types needed to define a mesh.
pub enum DefaultMeshRefs<Trait: MeshTrait> {}
pub impl MeshRefs for DefaultMeshRefs<Trait> {
    type Scalar         = Trait::Scalar;
    type Point          = Trait::Point;
    type Normal         = Trait::Point;
    type Color          = Trait::Color;
    type TexCoord1D     = Trait::TexCoord1D;
    type TexCoord2D     = Trait::TexCoord2D;
    type TexCoord3D     = Trait::TexCoord3D;
    type TextureIndex   = Trait::TextureIndex;
    type VertexHandle   = VertexHandle;
    type FaceHandle     = FaceHandle;
    type EdgeHandle     = EdgeHandle;
    type HalfedgeHandle = HalfedgeHandle;
}

// TODO: These need to depend on the Attributes
pub struct DefaultVertexData;
pub struct DefaultHalfedgeData;
pub struct DefaultEdgeData;
pub struct DefaultFaceData;

/// Definition of the mesh entities (items).
pub trait FinalMeshItems<Trait: MeshTrait /*, IsTriMesh: bool */> {
    type Refs: MeshRefs = DefaultMeshRefs;

    // Export Refs Types.
    type Point        = Refs::Point;
    type Scalar       = Refs::Scalar;
    type Normal       = Refs::Normal;
    type Color        = Refs::Color;
    type TexCoord1D   = Refs::TexCoord1D;
    type TexCoord2D   = Refs::TexCoord2D;
    type TexCoord3D   = Refs::TexCoord3D;
    type TextureIndex = Refs::TextureIndex;

    // Attribute bits to determine default fields.
    const VAttribs: Attributes = Traits::VertexAttributes;
    const HAttribs: Attributes = Traits::HalfedgeAttributes;
    const EAttribs: Attributes = Traits::EdgeAttributes;
    const FAttribs: Attributes = Traits::FaceAttributes;

    // Vertex data.
    type VertexData = DefaultVertexData;
    type HalfedgeData = DefaultHalfedgeData;
    type EdgeData = DefaultEdgeData;
    type FaceData = DefaultFaceData;
}

pub enum DefaultFinalMeshItems<Trait: MeshTrait> {}
impl<Trait: MeshTrait> FinalMeshItems<Trait /*, true */> for DefaultFinalMeshItems<Trait> {}


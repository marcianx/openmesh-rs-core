
# TODO

- [x] `Util/BaseProperty`
- [x] `Util/Property`
- [x] `Util/PropertyContainer`

C++ OpenMesh Traits (+ Attributes)

- [ ] typedefs and an enum with an `attributes.rs` value that defines which attributes are available.
- [ ] `DefaultTraits` (base class for all traits)
- [ ] `MergeTraits` that OR's the attributes from two classes (at compile time - not supported by Rust).


- Circulators (generic over mesh)
  - Relies on mesh connectivity traversal operations

----------------------------------------------------------------------

# Mesh type construction

["Specifying your Mesh"][1]
["Interface concepts"][2]

[1]: http://www.openmesh.org/media/Documentations/OpenMesh-5.0-Documentation/a00020.html
[2]: http://www.openmesh.org/media/Documentations/OpenMesh-5.0-Documentation/a00743.html


2 types of mesh
- Triangle-only
- General polygonal

## Handle

`BaseHandle`
- [x] `VertexHandle`
- [x] `HalfedgeHandle`
- [x] `EdgeHandle`
- [x] `FaceHandle`
- [x] `BasePropHandle<T>` - type of dereferenced data
  - [x] `VPropHandle<T>`
  - [x] `HPropHandle<T>`
  - [x] `EPropHandle<T>`
  - [x] `FPropHandle<T>`


## Kernel

**FinalMeshItemsT<Trait = DefaultTrait, isTriMesh: bool>**
- Bunch of typedefs, and an enum (effectively, const) with the attribute flag
  values for each object type (vertex, edge, etc).
- Defines extendible `VertexData`, `EdgeData`, etc, via CRTP to support merging
  fields in different meshes (so meshes can extend each others' `VertexData`,
  etc). Initially it extends a local empty struct.

`ArrayItems`
- [x] class defs: Definitions for `Vertex`, `Edge`, etc, structs.

`BaseKernel`
- [x] fields: property containers for `Vertex`, `Edge`, etc.
- methods
  - [x] property container (parallel lists): size/stats
  - [x] append/reserve/resize/clear
  - [ ] iterator
  - [x] add/remove properties (templated)
  - [x] string -> property handles -> property (list) -> value
  - [x] copy properties between items of the same type
- typedefs: `prop_iterator`

`ArrayKernel` < `BaseKernel`, `ArrayItems`
- fields:
  - [x] Containers for `Vertex, `Edge, etc (halfedge-edge) connectivity objects.
  - [x] Containers for `Vertex, `Edge, etc STATUS
  - [x] Ref counts for each STATUS container type
  - [ ] StatusSet API
    - `StatusSetT`, `AutoStatusSetT`, `ExtStatusSetT`
    - `BitMaskContainer`
- methods:
  - Garbage collection
  - Constructors for `Vertex`, `HalfEdge`, and `Face`
  - Container (array) access and modification operations
    - accessors by type
      - edge-handle = halfedge-handle-index / 2
    - handle validity checks
    - garbage collection (based on deleted status flag)
    - status accessors
  - Connectivity checks
    - vertex/face <-> halfedge
    - halfedge <-> opposite, prev, rotated (about vertex)

`PolyConnectivity` < `ArrayKernel`
`TriConnectivity` < `PolyConnectivity`

`AttribKernel<MeshItems, Connectivity>` < `Connectivity`
- Adds all standard (requested) properties to the kernel (with refcount!).
- Used to decorate Connectivity when defining the final MeshKernel below.

`PolyMesh_ArrayKernel<Traits = DefaultTrait>` (type used only for FinalMeshItems)
`TriMesh_ArrayKernel<Traits = DefaultTrait>`
- Must include 3 types, e.g.:
  - `typedef FinalMeshItemsT<Traits, true>               MeshItems;`
  - `typedef AttribKernelT<MeshItems, TriConnectivity>   AttribKernel;`
  - `typedef TriMeshT<AttribKernel>                      Mesh;`


## MESH

IGNORE: `Mesh/BaseMesh` "Common base class of all meshes." Not used anywhere!

`PolyMeshT<Kernel>` < `Kernel`
`TriMeshT<Kernel>` < `PolyMeshT<Kernel>`


----------------------------------------------------------------------

EVENTUALLY (Eg. when implementing algorithms)
  - AutoPropertyHandle
  - PropertyManager
    - What are these used for? Absolutely nuthin, WUH! (Potentially useful for
      external algorithms wanting to store properties for the duration of an
      algorithm, however. Seem extremely simple in behavior.)
  - ArrayKernel: StatusSet API (see above)

TODO:
- IMPORTANT: Restrict visibility of all implementation details.

- Linkify docs when structure solidified.

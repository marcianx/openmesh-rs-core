//! I/O options.

use bitflags;

#[doc(hidden)]
pub type FlagBits = u32;

bitflags! {
    #[doc = "OpenMesh I/O options a represented by bit-flags."]
    flags Options: FlagBits {
        #[doc = "No options"]
        const DEFAULT           = 0x0000,
        #[doc = "Set binary mode for r/w"]
        const BINARY            = 0x0001,
        #[doc = "Assume big endian byte ordering"]
        const MSB               = 0x0002,
        #[doc = "Assume little endian byte ordering"]
        const LSB               = 0x0004,
        #[doc = "Swap byte order in binary mode"]
        const SWAP              = 0x0008,
        #[doc = "Has (r) / store (w) vertex normals"]
        const VERTEX_NORMAL     = 0x0010,
        #[doc = "Has (r) / store (w) vertex colors"]
        const VERTEX_COLOR      = 0x0020,
        #[doc = "Has (r) / store (w) texture coordinates"]
        const VERTEX_TEX_COORD  = 0x0040,
        #[doc = "Has (r) / store (w) edge colors"]
        const EDGE_COLOR        = 0x0080,
        #[doc = "Has (r) / store (w) face normals"]
        const FACE_NORMAL       = 0x0100,
        #[doc = "Has (r) / store (w) face colors"]
        const FACE_COLOR        = 0x0200,
        #[doc = "Has (r) / store (w) face texture coordinates"]
        const FACE_TEX_COORD    = 0x0400,
        #[doc = "Has (r) / store (w) alpha values for colors"]
        const COLOR_ALPHA       = 0x0800,
        #[doc = "Has (r) / store (w) float values for colors (currently only implemented for PLY and OFF files)"]
        const COLOR_FLOAT       = 0x1000,
        #[doc = "Has (r) custom properties (currently only implemented in PLY Reader ASCII version"]
        const CUSTOM            = 0x2000,
    }
}

macro_rules! def_getter {
    ($method:ident, $value:ident) => {
        #[allow(missing_docs)]
        pub fn $method(self) -> bool {
            self.contains($value)
        }
    };
}

impl Options {
    /// Inserts the specified flags into `self` if `b` is true; otherwise removes them.
    pub fn update(&mut self, other: Options, b: bool) {
        if b {
            self.insert(other)
        } else {
            self.remove(other)
        }
    }

    def_getter! { is_binary          , BINARY }
    def_getter! { vertex_has_normal  , VERTEX_NORMAL }
    def_getter! { vertex_has_color   , VERTEX_COLOR }
    def_getter! { vertex_has_texcoord, VERTEX_TEX_COORD }
    def_getter! { edge_has_color     , EDGE_COLOR }
    def_getter! { face_has_normal    , FACE_NORMAL }
    def_getter! { face_has_color     , FACE_COLOR }
    def_getter! { face_has_texcoord  , FACE_TEX_COORD }
    def_getter! { color_has_alpha    , COLOR_ALPHA }
    def_getter! { color_is_float     , COLOR_FLOAT }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use crate::io::options::Options;

    #[test]
    fn test_empty() {
        let flags = Options::empty();
        assert!(!flags.is_binary());
        assert!(!flags.vertex_has_normal());
        assert!(!flags.vertex_has_color());
        assert!(!flags.vertex_has_texcoord());
        assert!(!flags.edge_has_color());
        assert!(!flags.face_has_normal());
        assert!(!flags.face_has_color());
        assert!(!flags.face_has_texcoord());
        assert!(!flags.color_has_alpha());
        assert!(!flags.color_is_float());
    }

    #[test]
    fn test_all() {
        let flags = Options::all();
        assert!(flags.is_binary());
        assert!(flags.vertex_has_normal());
        assert!(flags.vertex_has_color());
        assert!(flags.vertex_has_texcoord());
        assert!(flags.edge_has_color());
        assert!(flags.face_has_normal());
        assert!(flags.face_has_color());
        assert!(flags.face_has_texcoord());
        assert!(flags.color_has_alpha());
        assert!(flags.color_is_float());
    }
}

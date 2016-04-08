extern crate num;
use std::fmt::{Display, Formatter};
use util::property::size::Index;
use util::property::traits;

#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
/// BaseHandle for all index types.
pub struct Handle(pub Index);

impl traits::Handle for Handle {
    fn index(&self) -> Index { self.0 }
    fn set_index(&mut self, idx: Index) { self.0 = idx; }
}

// Display trait implementation.
impl Display for Handle {
    fn fmt(&self, formatter: &mut Formatter) -> ::std::fmt::Result { self.0.fmt(formatter) }
}

#[macro_export]
macro_rules! def_handle {
    ($handle: ident) => {
        #[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
        pub struct $handle($crate::util::property::Handle);
        impl $crate::util::property::traits::HandleConstructor for $handle {
            fn new() -> Self {
                $handle($crate::util::property::Handle($crate::util::property::size::INVALID_INDEX))
            }
            fn from_index(idx: $crate::util::property::size::Index) -> Self {
                assert!(idx != $crate::util::property::size::INVALID_INDEX);
                $handle($crate::util::property::Handle(idx))
            }
        }
        impl ::std::ops::Deref for $handle {
            type Target = $crate::util::property::Handle;
            fn deref(&self) -> &Self::Target { &self.0 }
        }
        impl ::std::ops::DerefMut for $handle {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
        impl ::std::fmt::Display for $handle {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }
    }
}


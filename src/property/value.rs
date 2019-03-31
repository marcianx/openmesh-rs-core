use crate::io::binary::Binary;
use crate::property::StorageFor;

/// Trait for all types of values that are allowed to be stored in a `Property` list.
pub trait Value: ::std::any::Any + Binary + StorageFor + Clone + Default + 'static {}
impl<T: ::std::any::Any + Binary + Clone + Default + 'static> Value for T {}


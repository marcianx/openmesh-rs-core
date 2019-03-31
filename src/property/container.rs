use crate::property::handle::PropHandle;
use crate::property::{
    INVALID_INDEX, Index, Size, Value,
    ItemHandle,
};
use crate::property::{PropertyList, ResizeableProperty};
use crate::property::ConstructableProperty; // for ::new() on property
use crate::property::Handle; // for `PropHandle` methods

/// Contains a parallel collection of `Property` trait objects.
#[derive(Clone, Default)]
pub struct PropertyContainer<H> {
    // TODO: Should there be a `RefCell` here to allow getting multiple properties mutably?
    /// List of all the properties, whose lengths are kept in sync.
    vec: Vec<Option<Box<ResizeableProperty<Handle=H>>>>,
}


impl<H: ItemHandle> ::std::fmt::Debug for PropertyContainer<H> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for opt_prop in self.vec.iter() {
            opt_prop.as_ref()
                 .map(|prop| prop.fmt(f))
                 .unwrap_or_else(|| "[deleted]".fmt(f))?;
            '\n'.fmt(f)?;
        }
        Ok(())
    }
}


impl<H: ItemHandle> PropertyContainer<H>
{
    ////////////////////////////////////////////////////////////////////////////////
    // Addition/getting/removal of properties.

    /// Returns the length of the `Property`-holder. The number of stored properties does not
    /// exceed this length.
    pub fn len(&self) -> Size { self.vec.len() as Size }

    /// Whether the container is empty.
    pub fn is_empty(&self) -> bool { self.vec.is_empty() }

    /// Adds a property whose elements are of type `T`.
    /// Panics in the unlikely case that the number of properties reaches `INVALID_INDEX`.
    pub fn add<T>(&mut self, name: Option<String>, len: Size) -> PropHandle<H, T>
        where T: Value
    {
        let name = name.unwrap_or_else(|| "<unknown>".to_owned());
        let pos = self.vec.iter().position(Option::is_none);
        let pos = match pos {
            Some(n) => n,
            None => {
                self.vec.push(None);
                self.vec.len() - 1
            }
        };
        self.vec[pos] = Some(Box::new(PropertyList::<T, H>::new(name, len)));
        if pos >= INVALID_INDEX as usize {
            panic!("Number of properties {} exceeds bounds {}-1", pos, INVALID_INDEX);
        }
        PropHandle::from_index(pos as Size)
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get<T>(&self, prop_handle: PropHandle<H, T>) -> Option<&PropertyList<T, H>>
        where T: Value
    {
        // NOTE: This handles prop_handle.index() == INVALID_INDEX just fine.
        self.vec
            .get(prop_handle.index() as usize)
            // &Option<Box<dyn Property>> -> Option<&Box<dyn Property>>
            .and_then(|opt_prop| opt_prop.as_ref()) // unwrap the `Option` wrapping the `Box`
            // prop: &Box<dyn Property>
            .and_then(|prop| prop.as_property().downcast_ref::<_>())
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get_mut<T>(&mut self, prop_handle: PropHandle<H, T>)
        -> Option<&mut PropertyList<T, H>>
        where T: Value
    {
        // NOTE: This handles prop_handle.index() == INVALID_INDEX just fine.
        self.vec
            .get_mut(prop_handle.index() as usize)
            // &Option<Box<dyn Property>> -> Option<&mut Box<dyn Property>>
            .and_then(|opt_prop| opt_prop.as_mut()) // unwrap the `Option` wrapping the `Box`
            // prop: &mut Box<dyn Property>
            .and_then(|prop| prop.as_property_mut().downcast_mut::<_>())
    }

    /// Removes the property at the given handle if any exists and if the `BasePropHandle`'s
    /// value type `T` matches that of the pointed-to property type. Returns true iff something
    /// was removed.
    pub fn remove<T>(&mut self, prop_handle: PropHandle<H, T>) -> bool
        where T: Value
    {
        // NOTE: This handles prop_handle.index() == INVALID_INDEX just fine.
        self.vec
            .get_mut(prop_handle.index() as usize)
            .and_then(|opt_prop| {
                // Require that `T` match the underlying property's type.
                opt_prop.as_ref().and_then(|box_prop| {
                    // Map finally to `Option<()>` to avoid borrowing from `opt_prop` for next map.
                    box_prop.as_property().downcast_ref::<PropertyList<T, H>>().map(|_| ())
                })
                .map(|_| opt_prop) // Return `opt_prop` only if `T` matched.
            })
            // Explicitly typed to catch errors since any `&mut Option` would compile successfully.
            .map(|opt_prop: &mut Option<Box<dyn ResizeableProperty<Handle=H>>>| {
                ::std::mem::swap(opt_prop, &mut None);
            })
            .is_some()
    }

    /// Removes all properties.
    pub fn clear(&mut self) {
        for opt_prop in self.vec.iter_mut() {
            ::std::mem::swap(opt_prop, &mut None);
        }
    }

    /// Returns the handle with the given name if any exists and corresponds to a property of type
    /// `T`. Otherwise, it returns an invalid handle.
    pub fn handle<T: Value>(&self, name: &str) -> PropHandle<H, T> {
        self.vec.iter()
            .position(|opt_prop| opt_prop.as_ref().map(|prop| prop.name() == name).unwrap_or(false))
            .and_then(|index| {
                // Return the index only if the found property corresponds to that for type `T`.
                // &Option<Box<dyn Property>> -> Option<&Box<dyn Property>>
                // -> &Box<dyn Property> -> &dyn Property -?-> PropertyList
                self.vec[index].as_ref().and_then(|box_prop| {
                    box_prop.as_property().downcast_ref::<PropertyList<T, H>>()
                }).map(|_| PropHandle::from_index(index as Index))
            })
            .unwrap_or_else(PropHandle::new)
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Collectively managing active property lists.

    /// Copies a all properties from one item to another of the same type.
    /// It may panic if either handle is invalid.
    pub fn copy_all(&mut self, h_src: H, h_dst: H) {
        for opt_prop in self.vec.iter_mut() {
            if let Some(prop) = opt_prop.as_mut() {
                prop.copy(h_src, h_dst);
            }
        }
    }

    /// Clears the contents of each active property list.
    pub fn clear_all(&mut self) {
        for opt_prop in self.vec.iter_mut() {
            if let Some(prop) = opt_prop.as_mut() {
                prop.clear();
            }
        }
    }

    /// Reserves space for `n` items in each active property list.
    pub fn reserve_all(&mut self, n: Size) {
        for opt_prop in self.vec.iter_mut() {
            if let Some(prop) = opt_prop.as_mut() {
                prop.reserve(n);
            }
        }
    }

    /// Resizes each active property list.
    pub fn resize_all(&mut self, n: Size) {
        for opt_prop in self.vec.iter_mut() {
            if let Some(prop) = opt_prop.as_mut() {
                prop.resize(n);
            }
        }
    }

    /// Pushes an item to the end of the property list.
    pub fn push_all(&mut self) {
        for opt_prop in self.vec.iter_mut() {
            if let Some(prop) = opt_prop.as_mut() {
                prop.push();
            }
        }
    }

    /// Swaps a pair of items in each active property list.
    /// TODO: Return an error if the indices were out of bounds.
    pub fn swap_all(&mut self, i0: H, i1: H) {
        for opt_prop in self.vec.iter_mut() {
            if let Some(prop) = opt_prop.as_mut() {
                prop.swap(i0, i1);
            }
        }
    }
}


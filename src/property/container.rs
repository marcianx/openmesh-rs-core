use property::BasePropHandle;
use property::size;
use property::traits::{self, PropertyFor};
use property::traits::ConstructableProperty; // for ::new() on property
use property::traits::Handle as HandleTrait; // for `BasePropHandle` methods

/// Contains a parallel collection of `Property` trait objects.
pub struct PropertyContainer<H> {
    // TODO: Should there be a `RefCell` here to allow getting multiple properties mutably?
    /// List of all the properties, whose lengths are kept in sync.
    vec: Vec<Option<Box<traits::ResizeableProperty<Handle=H>>>>,
    /// Length of each property.
    prop_len: size::Size,
}


impl<H: traits::Handle> Clone for PropertyContainer<H>
{
    fn clone(&self) -> Self {
        PropertyContainer {
            vec: self.vec.clone(),
            prop_len: self.prop_len,
        }
    }
}


impl<H: traits::Handle> ::std::fmt::Debug for PropertyContainer<H> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for opt_prop in self.vec.iter() {
            try!(opt_prop.as_ref()
                 .map(|prop| prop.fmt(f))
                 .unwrap_or("[deleted]".fmt(f)));
            '\n'.fmt(f)?;
        }
        Ok(())
    }
}


// Helper for brevity.
type TargetProperty<T, H> = <T as PropertyFor<H>>::Property;


impl<H: traits::Handle> PropertyContainer<H>
{
    ////////////////////////////////////////////////////////////////////////////////
    // Addition/getting/removal of properties.

    /// Returns the length of the `Property`-holder. The number of stored properties does not
    /// exceed this length.
    pub fn len(&self) -> size::Size { self.vec.len() as size::Size }

    /// Adds a property whose elements are of type `T`.
    /// Panics in the unlikely case that the number of properties reaches `size::INVALID_INDEX`.
    pub fn add<T>(&mut self, name: Option<String>) -> BasePropHandle<T>
        where T: traits::Value
    {
        let name = name.unwrap_or("<unknown>".to_owned());
        let pos = self.vec.iter().position(Option::is_none);
        let pos = match pos {
            Some(n) => n,
            None => {
                self.vec.push(None);
                self.vec.len() - 1
            }
        };
        self.vec[pos] = Some(Box::new(<T as PropertyFor<H>>::Property::new(name, self.prop_len)));
        if pos >= size::INVALID_INDEX as usize {
            panic!("Number of properties {} exceeds bounds {}-1", pos, size::INVALID_INDEX);
        }
        BasePropHandle::from_index(pos as size::Size)
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get<T>(&self, prop_handle: BasePropHandle<T>) -> Option<&<T as PropertyFor<H>>::Property>
        where T: traits::Value
    {
        // NOTE: This handles prop_handle.index() == size::INVALID_INDEX just fine.
        self.vec
            .get(prop_handle.index() as usize)
            // &Option<Box<traits::Property>> -> Option<&Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_ref()) // unwrap the `Option` wrapping the `Box`
            // prop: &Box<traits::Property>
            .and_then(|prop| prop.as_property().downcast_ref::<_>())
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get_mut<T>(&mut self, prop_handle: BasePropHandle<T>)
        -> Option<&mut <T as PropertyFor<H>>::Property>
        where T: traits::Value
    {
        // NOTE: This handles prop_handle.index() == size::INVALID_INDEX just fine.
        self.vec
            .get_mut(prop_handle.index() as usize)
            // &Option<Box<traits::Property>> -> Option<&mut Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_mut()) // unwrap the `Option` wrapping the `Box`
            // prop: &mut Box<traits::Property>
            .and_then(|prop| prop.as_property_mut().downcast_mut::<_>())
    }

    /// Removes the property at the given handle if any exists and if the `BasePropHandle`'s
    /// value type `T` matches that of the pointed-to property type. Returns true iff something
    /// was removed.
    pub fn remove<T>(&mut self, prop_handle: BasePropHandle<T>) -> bool
        where T: traits::Value
    {
        // NOTE: This handles prop_handle.index() == size::INVALID_INDEX just fine.
        self.vec
            .get_mut(prop_handle.index() as usize)
            .and_then(|opt_prop| {
                // Require that `T` match the underlying property's type.
                opt_prop.as_ref().and_then(|box_prop| {
                    // Map finally to `Option<()>` to avoid borrowing from `opt_prop` for next map.
                    box_prop.as_property().downcast_ref::<TargetProperty<T, H>>().map(|_| ())
                })
                .map(|_| opt_prop) // Return `opt_prop` only if `T` matched.
            })
            // Explicitly typed to catch errors since any `&mut Option` would compile successfully.
            .map(|opt_prop: &mut Option<Box<traits::ResizeableProperty<Handle=H>>>| {
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
    pub fn handle<T>(&self, name: &str) -> BasePropHandle<T>
        where T: traits::Value
    {
        self.vec.iter()
            .position(|opt_prop| opt_prop.as_ref().map(|prop| prop.name() == name).unwrap_or(false))
            .and_then(|index| {
                // Return the index only if the found property corresponds to that for type `T`.
                // &Option<Box<traits::Property>> -> Option<&Box<traits::Property>>
                // -> &Box<traits::Property> -> &traits::Property -?-> TargetProperty
                self.vec[index].as_ref().and_then(|box_prop| {
                    box_prop.as_property().downcast_ref::<TargetProperty<T, H>>()
                }).map(|_| BasePropHandle::from_index(index as size::Index))
            })
            .unwrap_or(BasePropHandle::new())
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Collectively managing active property lists.

    /// Copies a all properties from one item to another of the same type.
    /// It may panic if either handle is invalid.
    pub fn copy_all(&mut self, h_src: H, h_dst: H) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.copy(h_src, h_dst));
        }
    }

    /// Clears the contents of each active property list.
    pub fn clear_all(&mut self) {
        self.prop_len = 0;
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.clear());
        }
    }

    /// Reserves space for `n` items in each active property list.
    pub fn reserve_all(&mut self, n: size::Size) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.reserve(n));
        }
    }

    /// Resizes each active property list.
    pub fn resize_all(&mut self, n: size::Size) {
        self.prop_len = n;
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.resize(n));
        }
    }

    /// Pushes an item to the end of the property list.
    pub fn push_all(&mut self) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.push());
        }
    }

    /// Swaps a pair of items in each active property list.
    /// TODO: Return an error if the indices were out of bounds.
    pub fn swap_all(&mut self, i0: H, i1: H) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.swap(i0, i1));
        }
    }
}


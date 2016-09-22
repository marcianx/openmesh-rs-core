use property::{BasePropHandle, Property};
use property::size;
use property::traits;
use property::traits::Handle as HandleTrait; // for `BasePropHandle` methods

/// Contains a parallel collection of `Property` trait objects.
pub struct PropertyContainer<H> {
    // TODO: Should there be a `RefCell` here to allow getting multiple properties mutably?
    /// List of all the properties, whose lengths are kept in sync.
    vec: Vec<Option<Box<traits::ResizeableProperty<H>>>>,
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

impl<H: traits::Handle> PropertyContainer<H>
{
    ////////////////////////////////////////////////////////////////////////////////
    // Addition/getting/removal of properties.

    /// Returns the length of the `Property`-holder. The number of stored properties does not
    /// exceed this length.
    pub fn len(&self) -> size::Size { self.vec.len() as size::Size }

    /// Adds a property whose elements are of type `T`.
    /// Panics in the unlikely case that the number of properties reaches `size::INVALID_INDEX`.
    pub fn add<T>(&mut self, name: Option<String>) -> BasePropHandle
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
        self.vec[pos] = Some(Box::new(Property::<T, H>::new(name, self.prop_len)));
        if pos >= size::INVALID_INDEX as usize {
            panic!("Number of properties {} exceeds bounds {}-1", pos, size::INVALID_INDEX);
        }
        BasePropHandle::from_index(pos as size::Size)
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get<T>(&self, prop_handle: BasePropHandle) -> Option<&Property<T, H>>
        where T: traits::Value
    {
        // NOTE: This handles prop_handle.index() == size::INVALID_INDEX just fine.
        self.vec
            .get(prop_handle.index() as usize)
            // &Option<Box<traits::Property>> -> Option<&Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_ref()) // unwrap the `Option` wrapping the `Box`
            // prop: &Box<traits::Property>
            .and_then(|prop| prop.as_property().downcast_ref::<Property<T, H>>())
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get_mut<T>(&mut self, prop_handle: BasePropHandle) -> Option<&mut Property<T, H>>
        where T: traits::Value
    {
        // NOTE: This handles prop_handle.index() == size::INVALID_INDEX just fine.
        self.vec
            .get_mut(prop_handle.index() as usize)
            // &Option<Box<traits::Property>> -> Option<&mut Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_mut()) // unwrap the `Option` wrapping the `Box`
            // prop: &mut Box<traits::Property>
            .and_then(|prop| prop.as_property_mut().downcast_mut::<Property<T, H>>())
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn remove(&mut self, prop_handle: BasePropHandle) {
        // NOTE: This handles prop_handle.index() == size::INVALID_INDEX just fine.
        self.vec
            .get_mut(prop_handle.index() as usize)
            // &Option<Box<traits::Property>> -> &None
            .map(|opt_prop| ::std::mem::swap(opt_prop, &mut None));
    }

    /// Removes all properties.
    pub fn clear(&mut self) {
        for opt_prop in self.vec.iter_mut() {
            ::std::mem::swap(opt_prop, &mut None);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Collectively managing active property lists.

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

    /// Swaps a pair of items in each active property list.
    /// TODO: Return an error if the indices were out of bounds.
    pub fn swap_all(&mut self, i0: H, i1: H) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.swap(i0, i1));
        }
    }

    // TODO: Add methods for bit-vectors (`PropertyBits`) as well.
}


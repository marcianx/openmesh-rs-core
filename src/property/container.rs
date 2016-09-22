use property::{BasePropHandle, Property};
use property::size;
use property::traits;
use property::traits::Handle as HandleTrait; // for `BasePropHandle` methods

/// Contains a parallel collection of `Property` trait objects.
pub struct PropertyContainer<Handle> {
    /// List of all the properties, whose lengths are kept in sync.
    vec: traits::Properties<Handle>,
    /// Length of each property.
    prop_len: size::Size,
}


impl<Handle> Clone for PropertyContainer<Handle>
    where traits::Properties<Handle>: Clone
{
    fn clone(&self) -> Self {
        PropertyContainer {
            vec: self.vec.clone(),
            prop_len: self.prop_len,
        }
    }
}

impl<H: traits::Handle> traits::PropertyContainer<H> for PropertyContainer<H>
{
    ////////////////////////////////////////////////////////////////////////////////
    // Addition/getting/removal of properties.

    fn len(&self) -> size::Size { self.vec.len() as size::Size }

    fn vec(&self) -> &traits::Properties<H> { &self.vec }

    fn add<T>(&mut self, name: Option<String>) -> BasePropHandle
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

    fn get<T>(&self, prop_handle: BasePropHandle) -> Option<&Property<T, H>>
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

    fn get_mut<T>(&mut self, prop_handle: BasePropHandle) -> Option<&mut Property<T, H>>
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

    fn remove(&mut self, prop_handle: BasePropHandle) {
        // NOTE: This handles prop_handle.index() == size::INVALID_INDEX just fine.
        self.vec
            .get_mut(prop_handle.index() as usize)
            // &Option<Box<traits::Property>> -> &None
            .map(|opt_prop| ::std::mem::swap(opt_prop, &mut None));
    }

    fn clear(&mut self) {
        for opt_prop in self.vec.iter_mut() {
            ::std::mem::swap(opt_prop, &mut None);
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Collectively managing active property lists.

    fn clear_all(&mut self) {
        self.prop_len = 0;
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.clear());
        }
    }

    fn reserve_all(&mut self, n: size::Size) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.reserve(n));
        }
    }

    fn resize_all(&mut self, n: size::Size) {
        self.prop_len = n;
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.resize(n));
        }
    }

    fn swap_all(&mut self, i0: H, i1: H) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.swap(i0, i1));
        }
    }
}


use io::binary::Binary;
use util::property::handle::{Handle, HandleProvider};
use util::property::traits;
use util::property::property;

// TODO: Should there be a `RefCell` here to allow getting multiple properties mutably?
pub type Properties = Vec<Option<Box<traits::Property>>>;

/// Contains a parallel collection of `Property` trait objects.
#[derive(Clone)]
pub struct PropertyContainer {
    vec: Properties
}

impl PropertyContainer {
    ////////////////////////////////////////////////////////////////////////////////
    // Addition/getting/removal of properties.

    /// Returns the length of the `Property`-holder. The number of stored properties does not
    /// exceed this length.
    pub fn len(&self) -> usize { self.vec.len() }

    /// The underlying property vector.
    pub fn vec(&self) -> &Properties { &self.vec }

    /// Adds a property whose elements are of type `T`.
    pub fn add<T: Binary + Clone + Default + 'static>(&mut self, name: Option<String>) -> Handle
        where property::Property<T>: ::std::any::Any,
              Vec<T>: Binary
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
        self.vec[pos] = Some(Box::new(property::Property::<T>::new(name)));
        Handle::from_index(pos)
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get<T, H>(&mut self, handle: &HandleProvider) -> Option<&property::Property<T>>
        where T: Binary + Clone + Default + 'static,
              H: HandleProvider,
              property::Property<T>: ::std::any::Any,
              Vec<T>: Binary
    {
        self.vec
            .get(handle.handle().index() as usize)
            // &Option<Box<traits::Property>> -> Option<&Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_ref()) // unwrap the `Option` wrapping the `Box`
            // prop: &Box<traits::Property>
            .and_then(|prop| prop.downcast_ref::<property::Property<T>>())
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn get_mut<T, H>(&mut self, handle: &HandleProvider) -> Option<&mut property::Property<T>>
        where T: Binary + Clone + Default + 'static,
              H: HandleProvider,
              property::Property<T>: ::std::any::Any,
              Vec<T>: Binary
    {
        self.vec
            .get_mut(handle.handle().index() as usize)
            // &Option<Box<traits::Property>> -> Option<&mut Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_mut()) // unwrap the `Option` wrapping the `Box`
            // prop: &mut Box<traits::Property>
            .and_then(|prop| prop.downcast_mut::<property::Property<T>>())
    }

    /// Returns the property at the given handle if any exists and if the return type matches.
    pub fn remove<H>(&mut self, handle: &HandleProvider) {
        self.vec
            .get_mut(handle.handle().index() as usize)
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
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.clear());
        }
    }

    /// Reserves space for `n` items in each active property list.
    pub fn reserve_all(&mut self, n: usize) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.reserve(n));
        }
    }

    /// Resizes each active property list.
    pub fn resize_all(&mut self, n: usize) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.resize(n));
        }
    }

    /// Swaps a pair of items in each active property list.
    pub fn swap_all(&mut self, i0: usize, i1: usize) {
        for opt_prop in self.vec.iter_mut() {
            opt_prop.as_mut().map(|prop| prop.swap(i0, i1));
        }
    }

    // TODO: Change all usize to Index (u32)?

    // TODO: Add methods for bit-vectors (`PropertyBits`) as well.
}


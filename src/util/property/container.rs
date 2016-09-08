use io::binary::Binary;
use util::property::property;
use util::property::size;
use util::property::traits;

/// Contains a parallel collection of `Property` trait objects.
pub struct PropertyContainer<Handle> {
    vec: traits::Properties<Handle>
}


impl<Handle> Clone for PropertyContainer<Handle>
    where traits::Properties<Handle>: Clone
{
    fn clone(&self) -> Self {
        PropertyContainer {
            vec: self.vec.clone()
        }
    }
}


impl<H: traits::Handle> traits::PropertyContainer<H> for PropertyContainer<H>
{
    ////////////////////////////////////////////////////////////////////////////////
    // Addition/getting/removal of properties.

    fn len(&self) -> size::Size { self.vec.len() as size::Size }

    fn vec(&self) -> &traits::Properties<H> { &self.vec }

    fn add<T>(&mut self, name: Option<String>) -> H
        where T: Binary + Clone + Default + 'static,
              property::Property<T, H>: ::std::any::Any
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
        self.vec[pos] = Some(Box::new(property::Property::<T, H>::new(name)));
        if pos >= size::INVALID_INDEX as usize {
            panic!("Number of properties {} exceeds bounds {}-1", pos, size::INVALID_INDEX);
        }
        H::from_index(pos as size::Size)
    }

    fn get<T>(&mut self, handle: H) -> Option<&property::Property<T, H>>
        where T: Binary + Clone + Default + 'static,
              property::Property<T, H>: ::std::any::Any
    {
        self.vec
            .get(handle.index() as usize)
            // &Option<Box<traits::Property>> -> Option<&Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_ref()) // unwrap the `Option` wrapping the `Box`
            // prop: &Box<traits::Property>
            .and_then(|prop| prop.downcast_ref::<property::Property<T, H>>())
    }

    fn get_mut<T>(&mut self, handle: H) -> Option<&mut property::Property<T, H>>
        where T: Binary + Clone + Default + 'static,
              property::Property<T, H>: ::std::any::Any
    {
        self.vec
            .get_mut(handle.index() as usize)
            // &Option<Box<traits::Property>> -> Option<&mut Box<traits::Property>>
            .and_then(|opt_prop| opt_prop.as_mut()) // unwrap the `Option` wrapping the `Box`
            // prop: &mut Box<traits::Property>
            .and_then(|prop| prop.downcast_mut::<property::Property<T, H>>())
    }

    fn remove(&mut self, handle: H) {
        self.vec
            .get_mut(handle.index() as usize)
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


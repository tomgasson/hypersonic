use std::{
    ops::Index,
    sync::{Arc, Mutex, atomic::AtomicUsize, atomic::Ordering},
    usize,
};

pub type ContainerItem<T> = Arc<Mutex<ContainerEntry<T>>>;

pub enum ContainerEntry<T> {
    Value(T),
    Undefined,
}

impl<T> ContainerEntry<T> {
    // pub fn is_value(&self) -> bool {
    //     match *self {
    //         ContainerEntry::Value(_) => true,
    //         _ => false,
    //     }
    // }

    // pub fn get_value(&self) -> Option<&T> {
    //     match self {
    //         ContainerEntry::Value(v) => Some(v),
    //         _ => None,
    //     }
    // }

    pub fn get_value_mut(&mut self) -> Option<&mut T> {
        match self {
            ContainerEntry::Value(v) => Some(v),
            _ => None,
        }
    }

    // pub fn is_undefined(&self) -> bool {
    //     match *self {
    //         ContainerEntry::Value(_) => false,
    //         _ => true,
    //     }
    // }
}

#[derive(Clone)]
pub struct StaticContainer<T> {
    capacity: usize,
    size: Arc<AtomicUsize>,
    items: Arc<Box<[ContainerItem<T>]>>,
}

impl<T> StaticContainer<T> {
    pub fn new(capacity: usize) -> Arc<StaticContainer<T>> {
        let mut items = Vec::<ContainerItem<T>>::with_capacity(capacity);

        for _ in 0..capacity {
            let container = Arc::new(Mutex::new(ContainerEntry::Undefined));
            items.push(container.clone());
        }

        let slice = items.into_boxed_slice();

        return Arc::new(StaticContainer {
            capacity,
            size: Arc::new(AtomicUsize::new(0)),
            items: Arc::new(slice),
        });
    }

    pub fn len(&self) -> usize {
        return self.size.load(Ordering::Acquire);
    }

    pub fn index(&self, index: usize) -> ContainerItem<T> {
        let item = self.items.index(index);
        return item.clone();
    }

    // pub fn iter(&self) -> Iter<'_, ContainerItem<T>> {
    //     return self.items.iter();
    // }

    pub fn push(&self, item: T) -> usize {
        let new_size = self.size.fetch_add(1, Ordering::Relaxed);
        if new_size > (self.capacity) {
            // TODO expand
            panic!("PANIC: Container has reached size limit");
        }

        let container = self.index(new_size);
        let lock_result = container.lock();
        if lock_result.is_err() {
            todo!();
        }
        let mut guard = lock_result.unwrap();
        *guard = ContainerEntry::Value(item);
        return  new_size;
    }
}

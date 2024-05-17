use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::ModuleListener;

/// Allows for module to listen to Event `T`.
///
/// **WARNING: For this to work you need to add the event type to the associated type `<Self as Module>::ListeningTo`**
pub trait Listener<T>: 'static {
    fn on_event(&mut self, event: &mut T, queue: &mut EventQueue);
}

/// Queue of events to be executed
pub struct EventQueue {
    inner: Vec<Box<dyn Any>>,
}

impl EventQueue {
    pub(crate) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(crate) fn drain(&mut self) -> Vec<Box<dyn Any>> {
        let l = self.inner.len();
        std::mem::replace(&mut self.inner, Vec::with_capacity(l))
    }

    pub fn is_empty(&mut self) -> bool {
        self.inner.is_empty()
    }

    /// Pushes a new event `T` into the event queue to be dispatched.
    pub fn push<T: 'static>(&mut self, event: T) {
        self.inner.push(Box::new(event))
    }
}

/// Simply a tuple of Events, for examples: `()`, `(EventA,)` or `(EventA, EventB, EventC)`.
/// But the generic type `T` must implement [`Listener<E>`](Listener) for every event `E` in the tuple.
///
/// In other terms:
/// `(A, .., Z,): EventList<T>` is valid only if `T: Listener<A> + .. + Listener<Z>`
pub trait EventList<T> {
    fn raw_listeners() -> ModuleListener<T>;
}
impl<T> EventList<T> for () {
    fn raw_listeners() -> ModuleListener<T> {
        HashMap::new()
    }
}

macro_rules! _impl {
    ($($name:tt)*) => {
        impl<T, $($name: 'static),*> EventList<T> for ($($name,)*) where T: 'static $( + Listener<$name>)* {
            fn raw_listeners() -> ModuleListener<T> {
                let mut map = HashMap::new();
                $(
                    let callback:  Box<dyn Fn(&mut T, &mut Box<dyn Any>, &mut EventQueue)> = Box::new(|_self, any_event, event_queue| {
                        Listener::<$name>::on_event(_self, any_event.downcast_mut().unwrap(), event_queue)
                    });
                    map.insert(TypeId::of::<$name>(), callback);
                )*
                map
            }
        }
    };
}

#[rustfmt::skip] mod _impl16 { use super::*; _impl!(A);_impl!(A B); _impl!(A B C);_impl!(A B C D);_impl!(A B C D E);_impl!(A B C D E F);_impl!(A B C D E F G);_impl!(A B C D E F G H);_impl!(A B C D E F G H I);_impl!(A B C D E F G H I J);_impl!(A B C D E F G H I J K);_impl!(A B C D E F G H I J K L);_impl!(A B C D E F G H I J K L M);_impl!(A B C D E F G H I J K L M N);_impl!(A B C D E F G H I J K L M N O);_impl!(A B C D E F G H I J K L M N O P);}

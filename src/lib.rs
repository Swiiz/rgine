use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    error::Error,
    fmt::Display,
    marker::PhantomData,
    rc::Rc,
};

use crate::{
    events::{EventList, EventQueue},
    standards::events::OnStart,
};

pub mod events;
pub mod standards;

/// Modules are an easy way to decouple the game engine code.
/// Those can be loaded from the `Engine` struct.
///
/// Self is the Module `State`
/// ListeningTo is a list of events that the module is listening to `(EventA, .., EventZ,)` using the `Listener<SomeEvent>` trait
///
/// TODO:
///  - Allow for module config (how?)
///  - Allow for unloading, and maybe even hot reloading if possible
///  - Allow for debug informations on a per module basis.
pub trait Module: Any + Sized {
    type ListeningTo: EventList<Self>;

    fn new(ctx: &mut Engine) -> AnyResult<Self>;
}

#[derive(Debug)]
pub enum ModuleError {
    InitError(Box<dyn Error>),
    AlreadyExist,
}

impl Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitError(_) => write!(f, "Failed to initialize module"),
            Self::AlreadyExist => write!(f, "Module already exist"),
        }
    }
}

impl Error for ModuleError {}

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

pub type Modules = HashMap<TypeId, AnyModule>;
pub type EventModuleSubscribers = HashMap<TypeId, Vec<TypeId>>;

pub struct Engine {
    modules: Modules,
    subscribers: EventModuleSubscribers,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            modules: Modules::new(),
            subscribers: EventModuleSubscribers::new(),
        }
    }
}

impl Engine {
    pub fn load_module<T: Module>(&mut self) -> Result<Dependency<T>, ModuleError> {
        let tid = TypeId::of::<T>();

        if !self.is_loaded::<T>() {
            let module = AnyModule::new(T::new(self).map_err(ModuleError::InitError)?);
            for event in module.listeners.keys() {
                self.subscribers
                    .entry(*event)
                    .or_default()
                    .push(TypeId::of::<T>());
            }
            self.modules.insert(tid, module);
            Ok(self.dependency()?)
        } else {
            Err(ModuleError::AlreadyExist)
        }
    }

    pub fn is_loaded<T: Module>(&self) -> bool {
        self.modules.contains_key(&TypeId::of::<T>())
    }

    pub fn dependency<T: Module>(&mut self) -> Result<Dependency<T>, ModuleError> {
        if !self.is_loaded::<T>() {
            return self.load_module::<T>();
        }
        Ok(Dependency::new(
            self.modules.get(&TypeId::of::<T>()).unwrap(),
        ))
    }

    pub fn run(&mut self) {
        let mut event_queue = EventQueue::new();
        event_queue.push(OnStart);

        while !event_queue.is_empty() {
            for mut event in event_queue.drain() {
                let Some(modules) = self.subscribers.get(&(&*event).type_id()) else {
                    continue;
                };

                for tid in modules {
                    self.modules
                        .get_mut(tid)
                        .map(|m| m.handle_event(&mut event, &mut event_queue));
                }
            }
        }
    }
}

pub type ModuleListener<T> =
    HashMap<TypeId, Box<dyn Fn(&mut T, &mut Box<dyn Any>, &mut EventQueue)>>;
type AnyListener = Box<dyn Fn(RefMut<dyn Any>, &mut Box<dyn Any>, &mut EventQueue)>;

type ModuleState = Rc<RefCell<dyn Any>>;

pub struct AnyModule {
    state: ModuleState,
    listeners: HashMap<TypeId, Box<dyn Fn(RefMut<dyn Any>, &mut Box<dyn Any>, &mut EventQueue)>>,
}

pub struct Dependency<T: Module> {
    _marker: PhantomData<T>,
    state: ModuleState,
}

impl<T: Module> Dependency<T> {
    fn new(module: &AnyModule) -> Self {
        Self {
            _marker: PhantomData,
            state: module.state.clone(),
        }
    }

    pub fn read_state(&self) -> Ref<'_, T> {
        Ref::map((*self.state).borrow(), |state| {
            state.downcast_ref::<T>().unwrap()
        })
    }
}

impl AnyModule {
    fn new<T: Module>(state: T) -> AnyModule {
        Self {
            state: Rc::new(RefCell::new(state)),
            listeners: T::ListeningTo::raw_listeners()
                .into_iter()
                .map(|(tid, callback)| {
                    (
                        tid,
                        Box::new(
                            move |mut any_self: RefMut<dyn Any>,
                                  any_event: &mut Box<dyn Any>,
                                  event_queue: &mut EventQueue| {
                                callback(any_self.downcast_mut().unwrap(), any_event, event_queue)
                            },
                        ) as AnyListener,
                    )
                })
                .collect(),
        }
    }

    // Should only be called if the module have subscribed to the event!
    fn handle_event(&mut self, event: &mut Box<dyn Any>, event_queue: &mut EventQueue) {
        if let Some(callback) = self.listeners.get(&(&**event).type_id()) {
            callback((*self.state).borrow_mut(), event, event_queue)
        };
    }
}

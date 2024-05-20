//! Modules are an easy way to decouple code.
//! This crate allows for usage of those as well as events.
//!
//! # How does it work?
//! - [`Module`] is trait that you can implement on a struct to make it a module and make the struct data become it's state.
//! - `Events` can be anything and modules can listen for them to modify themselves like in a state machine using the [`Listener<_>`](events::Listener) trait.
//! - [`Engine`] does the heavy-lifting and allows for loading of events.
//!
//! # What's the point?
//! - A module state is deterministic over it's events (unless using interior mutability), which allows for easy networking, debugging...
//! - Modules are like plugins as you simply need to add them or remove them for your needs and they still work on their own. (taking dependencies into account)
//! - Can lead to better codebase structure with less coupling.
//!
//! # Optional features
//! - `standards`: often used events (game engine related), useful for compatibility between modules (enabled by default)

use std::{
    any::{type_name, Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    error::Error,
    fmt::Display,
    marker::PhantomData,
    rc::Rc,
    time::Instant,
};

use events::Event;

use crate::events::{DebugName, EventList, EventQueue};

pub mod events;
#[cfg(feature = "standards")]
pub mod standards;

/// Modules are an easy way to decouple code.
/// Those can be loaded from the `Engine` struct.
///
/// - Self is the Module `State`
/// - ListeningTo is a list of events that the module is listening to `(EventA, .., EventZ,)` using the `Listener<SomeEvent>` trait
///
/// TODO:
///  - Allow for debug informations on a per module basis.
///  - Maybe allow for hot reloading if possible
pub trait Module: Any + Sized {
    type ListeningTo: EventList<Self>;

    fn new(ctx: &mut Engine) -> AnyResult<Self>;
}

#[derive(Debug)]
/// Error relative to modules
pub enum ModuleError {
    /// Error occured during initialization
    InitError(Box<dyn Error>),
    /// Error occured because the engine can't support two instances of the same module
    AlreadyExist,
    /// Error occured because the target module could not be found
    NotFound,
    /// Error occured because the target module is in use and thus can't be unloaded
    InUse,
}

impl Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitError(_) => write!(f, "Failed to initialize module"),
            Self::AlreadyExist => write!(
                f,
                "The engine can't support two instances of the same module"
            ),
            Self::NotFound => write!(f, "The target module could not be found"),
            Self::InUse => write!(f, "The target module is in use and thus can't be unloaded"),
        }
    }
}

impl Error for ModuleError {}

/// A result with any error
pub type AnyResult<T> = Result<T, Box<dyn Error>>;

type Modules = HashMap<TypeId, AnyModule>;
type EventModuleSubscribers = HashMap<TypeId, Vec<TypeId>>;

/// Allows for instantiation, storage and event dispatching of modules
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

    /// Returns the module `T` as a `Dependency<T>`, loading it if not found.
    ///
    /// In case the initialization fail, an error is returned instead.
    pub fn dependency<T: Module>(&mut self) -> Result<Dependency<T>, ModuleError> {
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
            return Ok(self.dependency()?);
        }
        Ok(Dependency::new(self.modules.get(&tid).unwrap()))
    }

    /// Unloads the module `T` and returns its current state.
    ///
    /// In case the module is not already loaded, an error is returned instead.
    pub fn unload_module<T: Module>(&mut self) -> Result<T, ModuleError> {
        let tid = TypeId::of::<T>();

        if self.is_loaded::<T>() {
            let module = self.modules.remove(&tid).unwrap();
            for event in module.listeners.keys() {
                self.subscribers.remove(event);
            }
            let state = Rc::into_inner(module.state).ok_or(ModuleError::InUse)?;
            Ok(*state.into_inner().downcast::<T>().unwrap())
        } else {
            Err(ModuleError::NotFound)
        }
    }

    /// Check if a module is loadedd
    pub fn is_loaded<T: Module>(&self) -> bool {
        self.modules.contains_key(&TypeId::of::<T>())
    }

    /// Dispatch the event [`standards::events::OnStart`] to all subscribed modules
    /// and continue dispatching events until the [`EventQueue`] is empty.
    #[cfg(feature = "standards")]
    pub fn start(&mut self) {
        self.run_with(standards::events::OnStart)
    }

    /// Dispatch the event `T` to all subscribed modules
    /// and continue dispatching events until the [`EventQueue`] is empty.
    pub fn run_with<T: Event>(&mut self, event: T) {
        let mut root_event_queue = EventQueue::new();
        root_event_queue.push(event);

        #[cfg(feature = "debuglog")]
        println!("New Root Event: {}", type_name::<T>());
        while !root_event_queue.is_empty() {
            for event in root_event_queue.drain() {
                #[cfg(feature = "debuglog")]
                print!("- Executing: {}", DebugName::of(&*event));

                let mut event = event.as_any();
                let Some(modules) = self.subscribers.get(&(&*event).type_id()) else {
                    #[cfg(feature = "debuglog")]
                    println!("  ~ Empty Schedule ~");
                    continue;
                };

                let mut event_queue = EventQueue::new();

                for tid in modules {
                    self.modules
                        .get_mut(tid)
                        .map(|m| m.handle_event(event.as_mut(), &mut event_queue));
                }

                root_event_queue = root_event_queue.merge_after(event_queue);
            }
        }
    }
}

type ModuleListener<T> = HashMap<TypeId, Box<dyn Fn(&mut T, &mut dyn Any, &mut EventQueue)>>;
type AnyListener = Box<dyn Fn(RefMut<Box<dyn Any>>, &mut dyn Any, &mut EventQueue)>;

type ModuleState = Rc<RefCell<Box<dyn Any>>>;

struct AnyModule {
    state: ModuleState,
    listeners: HashMap<TypeId, Box<dyn Fn(RefMut<Box<dyn Any>>, &mut dyn Any, &mut EventQueue)>>,
}

impl AnyModule {
    fn new<T: Module>(state: T) -> AnyModule {
        Self {
            state: Rc::new(RefCell::new(Box::new(state))),
            listeners: T::ListeningTo::raw_listeners()
                .into_iter()
                .map(|(tid, callback)| {
                    (
                        tid,
                        Box::new(
                            move |mut any_self: RefMut<Box<dyn Any>>,
                                  any_event: &mut dyn Any,
                                  event_queue: &mut EventQueue| {
                                callback(
                                    any_self.as_mut().downcast_mut().unwrap(),
                                    any_event,
                                    event_queue,
                                )
                            },
                        ) as AnyListener,
                    )
                })
                .collect(),
        }
    }

    // Should only be called if the module have subscribed to the event!
    fn handle_event(&mut self, event: &mut dyn Any, event_queue: &mut EventQueue) {
        if let Some(callback) = self.listeners.get(&(&*event).type_id()) {
            callback((*self.state).borrow_mut(), event, event_queue)
        };
    }
}

/// An immutable handle to a `Module`.
///
/// You can read it's state with the `read_state(&self)` method.
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

    /// Read the module state immutably
    pub fn read_state(&self) -> Ref<'_, T> {
        Ref::map((*self.state).borrow(), |state| {
            state.downcast_ref::<T>().unwrap()
        })
    }
}

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use rgine_modules::{
    events::{EventQueue, Listener},
    utils::Take,
    AnyResult, Engine, Module,
};

pub type AssetLoader = Box<dyn FnOnce() -> Box<dyn Any>>;

pub enum AssetsEvent {
    Load { value: Take<Box<dyn Any>> },
    Reset,
}

pub struct AssetsModule {
    loaders: HashMap<TypeId, AssetLoader>,
    loaded: HashMap<TypeId, Box<dyn Any>>,
}

impl AssetsModule {
    pub fn is_loaded(&self, type_id: &TypeId) -> bool {
        self.loaded.contains_key(type_id)
    }

    pub fn get<T: 'static>(&self) -> &T {
        self.loaded
            .get(&TypeId::of::<T>())
            .expect("Tried to use asset that has not been loaded!")
            .downcast_ref()
            .unwrap()
    }
}

impl Module for AssetsModule {
    type ListeningTo = (AssetsEvent,);
    fn new(_: &mut Engine) -> AnyResult<Self> {
        Ok(Self {
            loaders: HashMap::new(),
            loaded: HashMap::new(),
        })
    }
}

impl Listener<AssetsEvent> for AssetsModule {
    fn on_event(&mut self, event: &mut AssetsEvent, _: &mut EventQueue) {
        match event {
            AssetsEvent::Load { value } => {
                let v = value.take();
                self.loaded.insert((&*v).type_id(), v);
            }
            AssetsEvent::Reset => self.loaders.clear(),
        }
    }
}

pub trait AssetsEventQueueExt {
    fn load_asset<T: 'static>(&mut self, asset: T);
}
impl AssetsEventQueueExt for EventQueue {
    fn load_asset<T: 'static>(&mut self, asset: T) {
        self.push(AssetsEvent::Load {
            value: Take::new(Box::new(asset)),
        })
    }
}

use rgine::{
    modules::{
        events::{EventQueue, Listener},
        standards::events::{OnRender, OnStart, OnUpdate},
        AnyResult, Engine, Module,
    },
    platform::window::{WindowPlatformConfig, WindowPlatformEngineExt},
};

fn main() {
    let mut engine = Engine::new();

    engine.load_module::<MyModule>().unwrap();

    engine
        .run_windowed(WindowPlatformConfig::default())
        .unwrap();
}

struct MyModule;
impl Module for MyModule {
    type ListeningTo = (OnStart, OnUpdate, OnRender);
    fn new(_: &mut Engine) -> AnyResult<Self> {
        Ok(MyModule)
    }
}

impl Listener<OnStart> for MyModule {
    fn on_event(&mut self, _: &mut OnStart, _: &mut EventQueue) {
        // Init...
    }
}
impl Listener<OnUpdate> for MyModule {
    fn on_event(&mut self, _: &mut OnUpdate, _: &mut EventQueue) {
        // Update...
    }
}
impl Listener<OnRender> for MyModule {
    fn on_event(&mut self, _: &mut OnRender, _: &mut EventQueue) {
        // Render...
    }
}

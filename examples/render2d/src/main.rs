use rgine::{
    graphics::RenderEvent,
    modules::{
        events::{EventQueue, Listener},
        standards::events::OnStart,
        AnyResult, Engine, Module,
    },
    platform::window::{WindowPlatformConfig, WindowPlatformEngineExt},
};

fn main() -> AnyResult<()> {
    let mut engine = Engine::new();

    engine.dependency::<MyModule>().unwrap();

    engine.run_windowed(WindowPlatformConfig::default())
}

struct MyModule;
impl Module for MyModule {
    type ListeningTo = (OnStart, RenderEvent);
    fn new(_: &mut Engine) -> AnyResult<Self> {
        Ok(MyModule)
    }
}

impl Listener<OnStart> for MyModule {
    fn on_event(&mut self, _: &mut OnStart, _: &mut EventQueue) {
        // Init...
    }
}
impl Listener<RenderEvent> for MyModule {
    fn on_event(&mut self, _: &mut RenderEvent, _: &mut EventQueue) {
        // Render...
    }
}

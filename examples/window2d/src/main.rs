use rgine::{
    graphics::OnRender,
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
    type ListeningTo = (OnStart, OnRender);
    fn new(_: &mut Engine) -> AnyResult<Self> {
        Ok(MyModule)
    }
}

impl Listener<OnStart> for MyModule {
    fn on_event(&mut self, _: &mut OnStart, _: &mut EventQueue) {
        // Init...
    }
}
impl Listener<OnRender> for MyModule {
    fn on_event(&mut self, _: &mut OnRender, _: &mut EventQueue) {
        // Render...
    }
}

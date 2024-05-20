use rgine_modules::{
    events::{EventQueue, Listener},
    standards::events::{OnShutdown, OnStart},
    Engine, Module,
};
use rgine_platform::window::{
    module::RenderReadyEvent, OnWindowPlatformUpdate, WindowPlatformConfig, WindowPlatformEngineExt,
};

fn main() {
    let mut engine = Engine::new();
    engine.dependency::<ExampleModule>().unwrap();
    engine
        .run_windowed(WindowPlatformConfig::default())
        .unwrap();
}

struct ExampleModule;
impl Module for ExampleModule {
    type ListeningTo = (OnStart, OnWindowPlatformUpdate, RenderReadyEvent, OnShutdown);
    fn new(_: &mut Engine) -> rgine_modules::AnyResult<Self> {
        Ok(ExampleModule)
    }
}

impl Listener<OnStart> for ExampleModule {
    fn on_event(&mut self, _: &mut OnStart, _: &mut EventQueue) {
        println!("On start!")
    }
}
impl Listener<OnWindowPlatformUpdate> for ExampleModule {
    fn on_event(&mut self, _: &mut OnWindowPlatformUpdate, _: &mut EventQueue) {
        println!("On update!")
    }
}
impl Listener<RenderReadyEvent> for ExampleModule {
    fn on_event(&mut self, _: &mut RenderReadyEvent, _: &mut EventQueue) {
        println!("On render rady!")
    }
}
impl Listener<OnShutdown> for ExampleModule {
    fn on_event(&mut self, _: &mut OnShutdown, _: &mut EventQueue) {
        println!("On shutdown!")
    }
}

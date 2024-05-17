use rgine_modules::{
    events::{EventQueue, Listener},
    standards::events::{OnRender, OnShutdown, OnStart, OnUpdate},
    Engine, Module,
};
use rgine_platform::window::{WindowPlatformConfig, WindowPlatformEngineExt};

fn main() {
    let mut engine = Engine::new();
    engine.load_module::<ExampleModule>().unwrap();
    engine
        .run_windowed(WindowPlatformConfig::default())
        .unwrap();
}

struct ExampleModule;
impl Module for ExampleModule {
    type ListeningTo = (OnStart, OnUpdate, OnRender, OnShutdown);
    fn new(_: &mut Engine) -> rgine_modules::AnyResult<Self> {
        Ok(ExampleModule)
    }
}

impl Listener<OnStart> for ExampleModule {
    fn on_event(&mut self, _: &mut OnStart, _: &mut EventQueue) {
        println!("On start!")
    }
}
impl Listener<OnUpdate> for ExampleModule {
    fn on_event(&mut self, _: &mut OnUpdate, _: &mut EventQueue) {
        println!("On update!")
    }
}
impl Listener<OnRender> for ExampleModule {
    fn on_event(&mut self, _: &mut OnRender, _: &mut EventQueue) {
        println!("On render!")
    }
}
impl Listener<OnShutdown> for ExampleModule {
    fn on_event(&mut self, _: &mut OnShutdown, _: &mut EventQueue) {
        println!("On shutdown!")
    }
}

use rgine_logger::info;
use rgine_modules::prelude::*;
use rgine_platform::window::{
    module::WindowRenderReadyEvent, OnWindowPlatformUpdate, WindowPlatformConfig,
    WindowPlatformEngineExt,
};

fn main() {
    Engine::new::<ExampleModule>().run_windowed(WindowPlatformConfig::default());
}

struct ExampleModule;
impl Module for ExampleModule {
    type ListeningTo = (
        StartEvent,
        OnWindowPlatformUpdate,
        WindowRenderReadyEvent,
        ShutdownEvent,
    );
    fn new(_: &mut Engine) -> rgine_modules::AnyResult<Self> {
        Ok(ExampleModule)
    }
}

impl Listener<StartEvent> for ExampleModule {
    fn on_event(&mut self, _: &mut StartEvent, _: &mut EventQueue) {
        info!("On start!")
    }
}
impl Listener<OnWindowPlatformUpdate> for ExampleModule {
    fn on_event(&mut self, _: &mut OnWindowPlatformUpdate, _: &mut EventQueue) {
        info!("On update!")
    }
}
impl Listener<WindowRenderReadyEvent> for ExampleModule {
    fn on_event(&mut self, _: &mut WindowRenderReadyEvent, _: &mut EventQueue) {
        info!("On render rady!")
    }
}
impl Listener<ShutdownEvent> for ExampleModule {
    fn on_event(&mut self, _: &mut ShutdownEvent, _: &mut EventQueue) {
        info!("On shutdown!")
    }
}

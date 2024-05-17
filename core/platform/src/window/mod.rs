use std::error::Error;

use self::module::WindowPlatformModule;
use rgine_modules::{
    standards::events::{OnShutdown, OnUpdate},
    Engine,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

pub mod module;
pub use winit::window::WindowAttributes;

pub trait WindowPlatformEngineExt {
    fn run_windowed(&mut self, config: WindowPlatformConfig) -> Result<(), Box<dyn Error>>;
}

impl WindowPlatformEngineExt for Engine {
    fn run_windowed(&mut self, config: WindowPlatformConfig) -> Result<(), Box<dyn Error>> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        self.load_module::<WindowPlatformModule>()?;
        self.start();

        let mut platform_layer = EngineWindowPlatformWrapper::new(self, config);
        event_loop.run_app(&mut platform_layer)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct WindowPlatformConfig {
    pub window_attributes: WindowAttributes,
}

impl Default for WindowPlatformConfig {
    fn default() -> Self {
        Self {
            window_attributes: WindowAttributes::default().with_title("Rgine window"),
        }
    }
}

struct EngineWindowPlatformWrapper<'a> {
    engine: &'a mut Engine,
    config: WindowPlatformConfig,
}

impl<'a> EngineWindowPlatformWrapper<'a> {
    fn new(engine: &'a mut Engine, config: WindowPlatformConfig) -> Self {
        Self { engine, config }
    }
}

impl<'a> ApplicationHandler for EngineWindowPlatformWrapper<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine
            .dependency::<WindowPlatformModule>()
            .unwrap()
            .read_state()
            .window
            .set(
                event_loop
                    .create_window(self.config.window_attributes.clone())
                    .unwrap(),
            )
            .unwrap();
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _wid: WindowId, event: WindowEvent) {
        self.engine.run_with(event);
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.engine.run_with(event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.engine.run_with(OnUpdate);
        if self
            .engine
            .dependency::<WindowPlatformModule>()
            .unwrap()
            .read_state()
            .should_close
        {
            event_loop.exit();
            self.engine.run_with(OnShutdown);
        }
    }
}

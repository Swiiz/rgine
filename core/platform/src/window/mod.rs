use std::sync::Arc;

use self::module::WindowPlatformModule;
use rgine_modules::{
    standards::{ShutdownEvent, StartEvent},
    Engine,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

pub mod module;
pub use winit::window::{Window, WindowAttributes};

pub trait WindowPlatformEngineExt {
    // Take self as owned so that it can't be called when running the engine
    fn run_windowed(self, config: WindowPlatformConfig);
}

impl WindowPlatformEngineExt for Engine {
    fn run_windowed(mut self, config: WindowPlatformConfig) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        self.dependency::<WindowPlatformModule>().expect(
            "Failed to load window platform module from platform layer on window platform.",
        );
        self.run_with(StartEvent);

        let mut platform_layer = EngineWindowPlatformWrapper::new(&mut self, config);
        event_loop.run_app(&mut platform_layer).unwrap();
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

pub struct WindowReadyEvent;
pub struct OnWindowPlatformUpdate;

impl<'a> ApplicationHandler for EngineWindowPlatformWrapper<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine
            .dependency::<WindowPlatformModule>()
            .unwrap()
            .read_state()
            .window
            .set(Arc::new(
                event_loop
                    .create_window(self.config.window_attributes.clone())
                    .unwrap(),
            ))
            .unwrap();

        self.engine.run_with(WindowReadyEvent);
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
        self.engine.run_with(OnWindowPlatformUpdate);
        if self
            .engine
            .dependency::<WindowPlatformModule>()
            .unwrap()
            .read_state()
            .should_close
        {
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        self.engine.run_with(ShutdownEvent);
    }
}

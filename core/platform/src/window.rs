use rgine_modules::Engine;
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub trait WindowPlatformEngineExt {
    fn run_windowed(&mut self) -> Result<(), EventLoopError>;
}

impl WindowPlatformEngineExt for Engine {
    fn run_windowed(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let mut platform_layer = EngineWindowPlatformWrapper::new(self);
        event_loop.run_app(&mut platform_layer)?;
        Ok(())
    }
}

struct EngineWindowPlatformWrapper<'a> {
    engine: &'a mut Engine,
    window: Option<Window>,
}

impl<'a> EngineWindowPlatformWrapper<'a> {
    fn new(engine: &'a mut Engine) -> Self {
        Self {
            engine,
            window: None,
        }
    }
}

impl<'a> ApplicationHandler for EngineWindowPlatformWrapper<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _wid: WindowId, event: WindowEvent) {
        self.engine.run_with(event.clone());
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw(); //TODO: maybe move into graphics module?
            }
            _ => (),
        }
    }
}

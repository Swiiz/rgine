use std::cell::OnceCell;

use rgine_modules::{
    events::{EventQueue, Listener},
    standards::events::OnRender,
    Engine, Module,
};
use winit::{
    event::{DeviceEvent, WindowEvent},
    window::Window,
};

pub struct OnRenderFinished;

pub struct WindowPlatformModule {
    pub should_close: bool,
    pub window: OnceCell<Window>,
}
impl Module for WindowPlatformModule {
    type ListeningTo = (WindowEvent, DeviceEvent, OnRenderFinished);
    fn new(_: &mut Engine) -> rgine_modules::AnyResult<Self> {
        Ok(Self {
            should_close: false,
            window: OnceCell::new(),
        })
    }
}
impl Listener<WindowEvent> for WindowPlatformModule {
    fn on_event(&mut self, event: &mut WindowEvent, queue: &mut EventQueue) {
        match event {
            WindowEvent::CloseRequested => {
                self.should_close = true;
            }
            WindowEvent::RedrawRequested => {
                queue.push(OnRender);
                queue.push(OnRenderFinished); //TODO: Could we use After<OnRender> instead? (todo in the modules engine)
            }
            _ => (),
        }
    }
}
impl Listener<OnRenderFinished> for WindowPlatformModule {
    fn on_event(&mut self, _: &mut OnRenderFinished, _: &mut EventQueue) {
        self.window.get().unwrap().request_redraw()
    }
}
impl Listener<DeviceEvent> for WindowPlatformModule {
    fn on_event(&mut self, _: &mut DeviceEvent, _: &mut EventQueue) {
        //...
    }
}

use std::{cell::OnceCell, sync::Arc};

use rgine_modules::{
    events::{EventQueue, Listener},
    Engine, Module,
};
use winit::{
    event::{DeviceEvent, WindowEvent},
    window::Window,
};

pub struct RequestWindowRedrawEvent;
pub struct RenderReadyEvent;
pub struct WindowResizeEvent;

pub struct WindowPlatformModule {
    pub should_close: bool,
    pub window: OnceCell<Arc<Window>>,
}
impl WindowPlatformModule {
    pub fn window_size(&self) -> Option<(u32, u32)> {
        self.window.get().map(|w| w.inner_size().into())
    }
}
impl Module for WindowPlatformModule {
    type ListeningTo = (WindowEvent, DeviceEvent, RequestWindowRedrawEvent);
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
                queue.push(RenderReadyEvent);
            }
            _ => (),
        }
    }
}
impl Listener<RequestWindowRedrawEvent> for WindowPlatformModule {
    fn on_event(&mut self, _: &mut RequestWindowRedrawEvent, _: &mut EventQueue) {
        self.window.get().unwrap().request_redraw()
    }
}
impl Listener<DeviceEvent> for WindowPlatformModule {
    fn on_event(&mut self, _: &mut DeviceEvent, _: &mut EventQueue) {
        //...
    }
}

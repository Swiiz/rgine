use std::{cell::OnceCell, sync::Arc};

use rgine_modules::{
    events::{EventQueue, Listener},
    Engine, Module,
};
use winit::{
    event::{DeviceEvent, WindowEvent},
    window::Window,
};

pub struct OnRequestWindowRedraw;
pub struct OnRenderReady;

pub struct WindowPlatformModule {
    pub should_close: bool,
    pub window: OnceCell<Arc<Window>>,
}
impl Module for WindowPlatformModule {
    type ListeningTo = (WindowEvent, DeviceEvent, OnRequestWindowRedraw);
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
                queue.push(OnRenderReady);
            }
            _ => (),
        }
    }
}
impl Listener<OnRequestWindowRedraw> for WindowPlatformModule {
    fn on_event(&mut self, _: &mut OnRequestWindowRedraw, _: &mut EventQueue) {
        self.window.get().unwrap().request_redraw()
    }
}
impl Listener<DeviceEvent> for WindowPlatformModule {
    fn on_event(&mut self, _: &mut DeviceEvent, _: &mut EventQueue) {
        //...
    }
}

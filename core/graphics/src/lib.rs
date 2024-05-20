use ctx::{Frame, GraphicsCtx};
use rgine_modules::{
    events::{EventQueue, Listener},
    AnyResult, Dependency, Engine, Module,
};
use rgine_platform::window::{
    module::{RenderReadyEvent, RequestWindowRedrawEvent, WindowPlatformModule, WindowResizeEvent},
    WindowReadyEvent,
};

pub mod color;
pub mod ctx;

pub struct RenderEvent;
pub struct RenderPresentEvent;

pub struct GraphicsModule {
    platform: Dependency<WindowPlatformModule>,

    pub ctx: Option<GraphicsCtx>,
    pub current_frame: Option<Frame>,
}

impl Module for GraphicsModule {
    type ListeningTo = (
        WindowReadyEvent,
        WindowResizeEvent,
        RenderReadyEvent,
        RenderPresentEvent,
    );

    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        let platform = ctx.dependency::<WindowPlatformModule>()?;

        Ok(Self {
            ctx: None,
            platform,
            current_frame: None,
        })
    }
}
impl Listener<WindowReadyEvent> for GraphicsModule {
    fn on_event(&mut self, _: &mut WindowReadyEvent, _: &mut EventQueue) {
        self.ctx = Some(GraphicsCtx::new(
            self.platform.read_state().window.get().unwrap().clone(),
        ))
    }
}
impl Listener<WindowResizeEvent> for GraphicsModule {
    fn on_event(&mut self, _: &mut WindowResizeEvent, _: &mut EventQueue) {
        if let Some(ctx) = &mut self.ctx {
            ctx.resize(self.platform.read_state().window_size().unwrap())
        }
    }
}
impl Listener<RenderReadyEvent> for GraphicsModule {
    fn on_event(&mut self, _: &mut RenderReadyEvent, queue: &mut EventQueue) {
        if let Some(frame) = self.ctx.as_ref().unwrap().next_frame() {
            self.current_frame = Some(frame);
            queue.push(RenderEvent);
            queue.push(RenderPresentEvent);
        }
    }
}
impl Listener<RenderPresentEvent> for GraphicsModule {
    fn on_event(&mut self, _: &mut RenderPresentEvent, queue: &mut EventQueue) {
        self.current_frame.take().map(|frame| {
            frame.present();
            queue.push(RequestWindowRedrawEvent);
        });
    }
}

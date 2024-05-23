use ctx::{Frame, GraphicsCtx};
use rgine_modules::{
    events::{EventQueue, Listener},
    standards::ShutdownEvent,
    AnyResult, Dependency, Engine, Module,
};
use rgine_platform::window::module::{
    RequestWindowRedrawEvent, WindowPlatformModule, WindowRenderReadyEvent,
};

pub mod color;
pub mod ctx;

pub use rgine_platform::window::{
    module::WindowResizeEvent as SurfaceResizeEvent, WindowReadyEvent,
};
pub struct PreSubmitRenderEvent;
pub struct SubmitRenderEvent;
pub struct RenderPresentEvent;

pub struct GraphicsModule {
    platform: Dependency<WindowPlatformModule>,

    pub ctx: Option<GraphicsCtx>,
    pub current_frame: Option<Frame>,
}

impl GraphicsModule {
    pub fn window_size(&self) -> Option<(u32, u32)> {
        self.platform.read_state().window_size()
    }
}

impl Module for GraphicsModule {
    type ListeningTo = (
        WindowReadyEvent,
        SurfaceResizeEvent,
        WindowRenderReadyEvent,
        RenderPresentEvent,
        ShutdownEvent,
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
impl Listener<SurfaceResizeEvent> for GraphicsModule {
    fn on_event(&mut self, _: &mut SurfaceResizeEvent, _: &mut EventQueue) {
        if let Some(ctx) = &mut self.ctx {
            ctx.resize(self.platform.read_state().window_size().unwrap())
        }
    }
}
impl Listener<WindowRenderReadyEvent> for GraphicsModule {
    fn on_event(&mut self, _: &mut WindowRenderReadyEvent, queue: &mut EventQueue) {
        if let Some(frame) = self.ctx.as_ref().unwrap().next_frame() {
            self.current_frame = Some(frame);
            queue.push(PreSubmitRenderEvent);
            queue.push(SubmitRenderEvent);
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
impl Listener<ShutdownEvent> for GraphicsModule {
    fn on_event(&mut self, _: &mut ShutdownEvent, _: &mut EventQueue) {
        // Prevent STATUS_ACCESS_VIOLATION because of platform dependency dropped before this module and so surface is pointing at null window, i guess?
        self.ctx.take();
    }
}

use ctx::GraphicsCtx;
use rgine_modules::{
    events::{EventQueue, Listener},
    AnyResult, Dependency, Engine, Module,
};
use rgine_platform::window::{
    module::{OnRenderReady, OnRequestWindowRedraw, WindowPlatformModule},
    OnWindowPlatformResumed,
};
use wgpu::{SurfaceTexture, TextureView};

pub mod color;
pub mod ctx;

pub struct OnRender {
    pub view: TextureView,
}
pub struct OnRenderPresent;

pub struct GraphicsModule {
    platform: Dependency<WindowPlatformModule>,

    pub ctx: Option<GraphicsCtx>,
    pub current_frame: Option<SurfaceTexture>,
}

impl Module for GraphicsModule {
    type ListeningTo = (OnWindowPlatformResumed, OnRenderReady, OnRenderPresent);

    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        let platform = ctx.dependency::<WindowPlatformModule>()?;

        Ok(Self {
            ctx: None,
            platform,
            current_frame: None,
        })
    }
}
impl Listener<OnWindowPlatformResumed> for GraphicsModule {
    fn on_event(&mut self, _: &mut OnWindowPlatformResumed, _: &mut EventQueue) {
        self.ctx = Some(GraphicsCtx::new(
            self.platform.read_state().window.get().unwrap().clone(),
        ))
    }
}
impl Listener<OnRenderReady> for GraphicsModule {
    fn on_event(&mut self, _: &mut OnRenderReady, queue: &mut EventQueue) {
        if let Some((frame, view)) = self.ctx.as_ref().unwrap().next_frame() {
            self.current_frame = Some(frame);
            queue.push(OnRender { view });
            queue.push(OnRenderPresent);
        }
    }
}
impl Listener<OnRenderPresent> for GraphicsModule {
    fn on_event(&mut self, _: &mut OnRenderPresent, queue: &mut EventQueue) {
        self.current_frame.take().map(|f| {
            f.present();
            queue.push(OnRequestWindowRedraw);
        });
    }
}

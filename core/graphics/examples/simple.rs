use rgine_graphics::{color::Color3, GraphicsModule, SubmitRenderEvent};
use rgine_modules::{
    events::{EventQueue, Listener},
    AnyResult, Dependency, Engine, Module,
};
use rgine_platform::window::{WindowPlatformConfig, WindowPlatformEngineExt};
use wgpu::*;

fn main() {
    Engine::new::<Example>().run_windowed(WindowPlatformConfig::default());
}

pub struct Example {
    graphics: Dependency<GraphicsModule>,
}
impl Module for Example {
    type ListeningTo = (SubmitRenderEvent,);
    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        Ok(Self {
            graphics: ctx.dependency()?,
        })
    }
}
impl Listener<SubmitRenderEvent> for Example {
    fn on_event(&mut self, _: &mut SubmitRenderEvent, _: &mut EventQueue) {
        let g = self.graphics.read_state();
        let ctx = g.ctx.as_ref().unwrap();
        let frame = g.current_frame.as_ref().unwrap();

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        // Simply puts a blue background!
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Sprite Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color3::rgb(0.25, 0.25, 1.).into()),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        ctx.queue.submit(std::iter::once(encoder.finish()));
    }
}

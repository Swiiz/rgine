use rgine_graphics::{color::Color3, GraphicsModule, OnRender};
use rgine_modules::{
    events::{EventQueue, Listener},
    AnyResult, Dependency, Engine, Module,
};
use rgine_platform::window::{WindowPlatformConfig, WindowPlatformEngineExt};
use wgpu::RenderPassDescriptor;

fn main() {
    let mut engine = Engine::new();

    engine.dependency::<Example>().unwrap();

    engine
        .run_windowed(WindowPlatformConfig::default())
        .unwrap();
}

pub struct Example {
    graphics: Dependency<GraphicsModule>,
}
impl Module for Example {
    type ListeningTo = (OnRender,);
    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        Ok(Self {
            graphics: ctx.dependency()?,
        })
    }
}
impl Listener<OnRender> for Example {
    fn on_event(&mut self, event: &mut OnRender, _: &mut EventQueue) {
        let g = self.graphics.read_state();
        let ctx = g.ctx.as_ref().unwrap();

        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Simply puts a blue background!
        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Sprite Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &event.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color3::rgb(0.25, 0.25, 1.).into()),
                        store: wgpu::StoreOp::Store,
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

use cgmath::{Array, Matrix3, Vector2};
use renderer::SpriteRenderer;
use rgine_graphics::{
    GraphicsModule, PreRenderSubmitEvent, SubmitRenderEvent, SurfaceResizeEvent, WindowReadyEvent,
};
use rgine_modules::{
    events::{EventQueue, Listener},
    AnyResult, Dependency, Engine, Module,
};

use texture::{DrawParams, Sprite, SpriteSheetsRegistry};

mod renderer;
pub mod texture;

pub trait AssetLoader: Module {
    fn sprite_registry(&self) -> SpriteSheetsRegistry;
}

pub struct Render2DEvent;
pub struct DrawSpriteEvent {
    sprite: Sprite,
    params: DrawParams,
}
pub struct RefreshRenderer2DEvent;

pub struct Renderer2DModule<L: AssetLoader> {
    graphics: Dependency<GraphicsModule>,
    asset_loader: Dependency<L>,

    renderer: Option<SpriteRenderer>,
}

impl<L: AssetLoader> Module for Renderer2DModule<L> {
    type ListeningTo = (
        WindowReadyEvent,
        RefreshRenderer2DEvent,
        PreRenderSubmitEvent,
        SubmitRenderEvent,
        SurfaceResizeEvent,
        DrawSpriteEvent,
    );
    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        let graphics = ctx.dependency::<GraphicsModule>()?;
        let asset_loader = ctx.dependency::<L>()?;

        Ok(Self {
            graphics,
            asset_loader,
            renderer: None,
        })
    }
}

impl<L: AssetLoader> Listener<WindowReadyEvent> for Renderer2DModule<L> {
    fn on_event(&mut self, _: &mut WindowReadyEvent, queue: &mut EventQueue) {
        queue.push(RefreshRenderer2DEvent);
    }
}

impl<L: AssetLoader> Listener<RefreshRenderer2DEvent> for Renderer2DModule<L> {
    fn on_event(&mut self, _: &mut RefreshRenderer2DEvent, _: &mut EventQueue) {
        let g = self.graphics.read_state();
        self.renderer.replace(SpriteRenderer::new(
            g.ctx.as_ref().unwrap(),
            g.window_size().unwrap(),
            self.asset_loader.read_state().sprite_registry(),
        ));
    }
}

impl<L: AssetLoader> Listener<PreRenderSubmitEvent> for Renderer2DModule<L> {
    fn on_event(&mut self, _: &mut PreRenderSubmitEvent, queue: &mut EventQueue) {
        queue.push(Render2DEvent);
    }
}

impl<L: AssetLoader> Listener<SubmitRenderEvent> for Renderer2DModule<L> {
    fn on_event(&mut self, _: &mut SubmitRenderEvent, _: &mut EventQueue) {
        if let Some(renderer) = &mut self.renderer {
            let g = self.graphics.read_state();
            let ctx = g.ctx.as_ref().unwrap();
            let frame = g.current_frame.as_ref().unwrap();
            renderer.submit(ctx, frame);
        }
    }
}

impl<L: AssetLoader> Listener<SurfaceResizeEvent> for Renderer2DModule<L> {
    fn on_event(&mut self, _: &mut SurfaceResizeEvent, _: &mut EventQueue) {
        if let Some(renderer) = &mut self.renderer {
            let g = self.graphics.read_state();
            let ctx = g.ctx.as_ref().unwrap();
            renderer.resize(ctx, g.window_size().unwrap())
        }
    }
}

impl<L: AssetLoader> Listener<DrawSpriteEvent> for Renderer2DModule<L> {
    fn on_event(&mut self, event: &mut DrawSpriteEvent, _: &mut EventQueue) {
        self.renderer
            .as_mut()
            .unwrap()
            .draw(event.sprite.clone(), event.params.clone());
    }
}

pub struct Draw2d<'a>(pub &'a mut EventQueue);
impl<'a> Draw2d<'a> {
    pub fn sprite(&mut self, sprite: Sprite, params: DrawParams) {
        self.0.push(DrawSpriteEvent { sprite, params })
    }
    pub fn sprite_centered(&mut self, sprite: Sprite, mut params: DrawParams) {
        params.transform = Matrix3::from_translation(Vector2::from_value(-0.5)) * params.transform;
        self.0.push(DrawSpriteEvent { sprite, params })
    }
}

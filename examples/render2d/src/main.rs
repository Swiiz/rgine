use rgine::{
    graphics::color::Color3,
    maths::{Array, Matrix3, SquareMatrix, Vector2, Zero},
    maths_extra::One,
    modules::{
        events::{EventQueue, Listener},
        AnyResult, Dependency, Engine, Module,
    },
    platform::window::{WindowPlatformConfig, WindowPlatformEngineExt},
    renderer_2d::{
        texture::{DrawParams, Sprite, SpriteSheetData, SpriteSheetHandle, SpriteSheetsRegistry},
        AssetLoader, Draw2d, Render2DEvent, Renderer2DModule,
    },
};

fn main() -> AnyResult<()> {
    let mut engine = Engine::new();

    engine.dependency::<Example>().unwrap();

    engine.run_windowed(WindowPlatformConfig::default())
}

type RendererModule = Renderer2DModule<AssetsModule>;

struct Example {
    assets: Dependency<AssetsModule>,
}
impl Module for Example {
    type ListeningTo = (Render2DEvent,);
    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        ctx.dependency::<RendererModule>()?;

        Ok(Self {
            assets: ctx.dependency()?,
        })
    }
}

impl Listener<Render2DEvent> for Example {
    fn on_event(&mut self, _: &mut Render2DEvent, queue: &mut EventQueue) {
        let mut draw = Draw2d(queue);
        draw.sprite_centered(
            Sprite {
                sheet: self.assets.read_state().characters_sheet,
                position: Vector2::zero(),
                size: Vector2::one(),
            },
            DrawParams {
                depth: 0.,
                tint: Color3::WHITE,
                transform: Matrix3::identity(),
            },
        );
    }
}

struct AssetsModule {
    pub characters_sheet: SpriteSheetHandle,
    sprite_registry: SpriteSheetsRegistry,
}
impl Module for AssetsModule {
    type ListeningTo = ();
    fn new(_: &mut Engine) -> AnyResult<Self> {
        let mut sprite_registry = SpriteSheetsRegistry::new();

        let characters_sprite = sprite_registry.register(SpriteSheetData {
            path: "./examples/render2d/assets/characters.png".to_string(),
            sprite_px_size: Vector2::from_value(16),
        });

        Ok(AssetsModule {
            characters_sheet: characters_sprite,
            sprite_registry,
        })
    }
}
impl AssetLoader for AssetsModule {
    fn sprite_registry(&self) -> SpriteSheetsRegistry {
        self.sprite_registry.clone()
    }
}

use std::{f32::consts::PI, time::Instant};

use rgine::prelude::*;

fn main() {
    Engine::new::<Example>().run_windowed(WindowPlatformConfig::default());
}

struct Example {
    time: Instant,
    characters_sheet: Option<SpriteSheetHandle>,
}
impl Module for Example {
    type ListeningTo = (Render2DEvent, StartEvent);
    fn new(ctx: &mut Engine) -> AnyResult<Self> {
        ctx.dependency::<Renderer2DModule>()?;

        Ok(Self {
            time: Instant::now(),
            characters_sheet: None,
        })
    }
}

impl Listener<StartEvent> for Example {
    fn on_event(&mut self, _: &mut StartEvent, queue: &mut EventQueue) {
        let mut sprite_registry = SpriteSheetsRegistry::new();

        self.characters_sheet = Some(sprite_registry.register(SpriteSheetData {
            path: "./examples/render2d/assets/characters.png".to_string(),
            sprite_px_size: Vector2::from_value(16),
        }));

        queue.load_asset(sprite_registry);
    }
}

impl Listener<Render2DEvent> for Example {
    fn on_event(&mut self, _: &mut Render2DEvent, queue: &mut EventQueue) {
        let rotation = Rad(self.time.elapsed().subsec_millis() as f32 * PI / 500.);

        let mut draw = Draw2d(queue);
        draw.sprite_centered(
            Sprite {
                sheet: self.characters_sheet.unwrap(),
                position: Vector2::zero(),
                size: Vector2::one(),
            },
            DrawParams {
                depth: 0.,
                tint: Color3::WHITE,
                transform: Matrix3::from_angle_z(rotation),
            },
        );
    }
}

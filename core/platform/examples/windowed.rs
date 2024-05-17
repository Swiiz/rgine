use rgine_modules::Engine;
use rgine_platform::window::WindowPlatformEngineExt;

fn main() {
    let mut engine = Engine::new();
    engine.run_windowed().unwrap();
}

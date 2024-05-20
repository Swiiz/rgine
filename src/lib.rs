pub use rgine_graphics as graphics;
pub use rgine_modules as modules;
pub use rgine_platform as platform;

#[cfg(feature = "2d")]
pub use rgine_renderer_2d as renderer_2d;

pub use cgmath as maths;

#[rustfmt::skip] pub mod maths_extra {
  use super::maths;
  pub trait One { fn one() -> Self; } use maths::{num_traits::NumCast, Array, BaseNum}; 
  impl<S: BaseNum> One for maths::Vector2<S> { fn one() -> Self { Self::from_value(<S as NumCast>::from(1).unwrap()) }}
  impl<S: BaseNum> One for maths::Vector3<S> { fn one() -> Self { Self::from_value(<S as NumCast>::from(1).unwrap()) }}
}

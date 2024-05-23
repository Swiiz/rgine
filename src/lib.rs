pub use rgine_assets as assets;
pub use rgine_graphics as graphics;
pub use rgine_modules as modules;
pub use rgine_platform as platform;

pub use rgine_logger::*;

#[cfg(feature = "2d")]
pub use rgine_renderer_2d as renderer_2d;

pub mod prelude {
    pub use crate::{assets::AssetsEventQueueExt, maths::*, modules::prelude::*};

    #[cfg(feature = "graphics")]
    pub use crate::{
        graphics::color::Color3,
        platform::window::{WindowPlatformConfig, WindowPlatformEngineExt},
    };

    #[cfg(feature = "2d")]
    pub use crate::renderer_2d::prelude::*;
}

pub mod maths {
    pub use cgmath::*;

    pub trait One {
        fn one() -> Self;
    }

    impl<S: BaseNum> One for Vector2<S> {
        fn one() -> Self {
            Self::from_value(<S as num_traits::NumCast>::from(1).unwrap())
        }
    }
    impl<S: BaseNum> One for Vector3<S> {
        fn one() -> Self {
            Self::from_value(<S as num_traits::NumCast>::from(1).unwrap())
        }
    }
}

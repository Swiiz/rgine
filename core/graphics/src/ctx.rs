use rgine_platform::window::Window;
use wgpu::*;

//TODO: add real logging
use std::sync::Arc;

pub struct GraphicsCtx {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_texture_format: TextureFormat,
    pub surface_capabilities: SurfaceCapabilities,
}

pub struct Frame {
    pub view: TextureView,
    pub surface_texture: SurfaceTexture,
}

impl GraphicsCtx {
    pub(crate) fn new(window: Arc<Window>) -> Self {
        let window_size = window.inner_size().into();
        let instance = Instance::new(InstanceDescriptor {
            backends: util::backend_bits_from_env().unwrap_or(Backends::all()),
            ..Default::default()
        });
        let surface = instance
            .create_surface(window)
            .unwrap_or_else(|e| panic!("Could not create graphics surface: {e}"));
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
            },
            None,
        ))
        .unwrap_or_else(|e| panic!("Could not acquire graphics device: {e}"));

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_texture_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);

        let mut _self = Self {
            device,
            queue,
            surface,
            surface_capabilities,
            surface_texture_format,
        };

        _self.resize(window_size);

        _self
    }

    pub(crate) fn resize(&mut self, window_size: (u32, u32)) {
        if window_size.0 > 0 && window_size.1 > 0 {
            self.surface.configure(
                &self.device,
                &SurfaceConfiguration {
                    usage: TextureUsages::RENDER_ATTACHMENT,
                    format: self.surface_texture_format,
                    width: window_size.0,
                    height: window_size.1,
                    present_mode: self.surface_capabilities.present_modes[0],
                    alpha_mode: self.surface_capabilities.alpha_modes[0],
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                },
            );
        }
    }

    pub(crate) fn next_frame(&self) -> Option<Frame> {
        let surface_texture = self
            .surface
            .get_current_texture()
            .map_err(|e| match e {
                SurfaceError::OutOfMemory => {
                    panic!("The system is out of memory for rendering!")
                }
                _ => format!("An error occured during surface texture acquisition: {e}"),
            })
            .ok()?;

        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        Some(Frame {
            surface_texture,
            view,
        })
    }
}

impl Frame {
    pub(crate) fn present(self) {
        self.surface_texture.present();
    }
}

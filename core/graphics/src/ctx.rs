use rgine_platform::window::Window;
use wgpu::*;

//TODO: add real logging
use std::{println as warn, sync::Arc};

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
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: util::backend_bits_from_env().unwrap_or(Backends::all()),
            ..Default::default()
        });
        let surface = instance
            .create_surface(window)
            .unwrap_or_else(|e| panic!("Could not create graphics surface: {e}"));
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
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
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
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

    pub(crate) fn next_frame(&self) -> Option<(SurfaceTexture, TextureView)> {
        let surface_texture = self.surface.get_current_texture().map_err(|e| match e {
            wgpu::SurfaceError::OutOfMemory => {
                panic!("The system is out of memory for rendering!")
            }
            _ => format!("An error occured during surface texture acquisition: {e}"),
        });

        if surface_texture.is_err() {
            warn!("{}", surface_texture.err().unwrap());
            return None;
        }
        let surface_texture = surface_texture.unwrap();

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Some((surface_texture, view))
    }
}

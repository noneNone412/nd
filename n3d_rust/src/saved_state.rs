use std::sync::Arc;
use std::{ffi::c_void, ptr::NonNull};
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::HtmlCanvasElement;
use wgpu::SurfaceTarget;
use wgpu::{Device, Features, Instance, Queue, Surface};
#[cfg(target_arch = "wasm32")]
pub struct SavedState {
    instance: Arc<wgpu::Instance>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

#[cfg(target_arch = "wasm32")]
impl SavedState {
    pub fn get_instance(&self) -> Arc<wgpu::Instance> {
        self.instance.clone()
    }
    pub fn get_adapter(&self) -> wgpu::Adapter {
        self.adapter.clone()
    }
    pub fn get_device(&self) -> wgpu::Device {
        self.device.clone()
    }
    pub fn get_queue(&self) -> wgpu::Queue {
        self.queue.clone()
    }
    pub fn get_surface(&self) -> &wgpu::Surface<'static> {
        &self.surface
    }
    pub fn get_surface_format(&self) -> wgpu::TextureFormat {
        self.surface_format.clone()
    }
}
#[cfg(target_arch = "wasm32")]
impl SavedState {
    pub async fn new() -> Self {
        let instance = Arc::new(Self::create_instance().await);
        let (surface, canvas) = Self::create_surface(&instance, String::from("canvas")).await;
        let adapter = Self::create_adapter(&instance, &surface).await;
        let (device, queue) = Self::create_device_and_queue(&adapter).await;
        let surface_format = Self::config_surface(&device, &adapter, &surface, &canvas).await;
        Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_format,
        }
    }

    async fn create_instance() -> wgpu::Instance {
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            flags: wgpu::InstanceFlags::empty(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
        };
        console::log_1(&"instance created!".into());
        return wgpu::Instance::new(&instance_descriptor);
    }

    async fn create_surface(
        instance: &Arc<wgpu::Instance>,
        canvas_id: String,
    ) -> (wgpu::Surface<'static>, HtmlCanvasElement) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(canvas_id.as_str()).unwrap();
        let render_canvas = canvas.dyn_into::<HtmlCanvasElement>();
        if render_canvas
            .as_ref()
            .unwrap()
            .get_context("webgpu")
            .is_err()
        {
            // Canvas might already have a context
            console::log_1(&"canvas already has context!".into());
        } else {
            console::log_1(&"canvas has no context!".into());
        }
        match &render_canvas {
            Ok(r) => {
                console::log_1(&"canvas created!".into());
            }
            Err(e) => {
                console::log_1(&"canvas creation failed!".into());
            }
        }
        if render_canvas
            .as_ref()
            .unwrap()
            .get_context("webgpu")
            .is_err()
        {
            // Canvas might already have a context
            console::log_1(&"canvas already has context!".into());
        } else {
            console::log_1(&"canvas has no context!".into());
        }
        // Canvas  not found error to be ignored
        let surface =
            instance.create_surface(SurfaceTarget::Canvas(render_canvas.clone().unwrap()));
        match &surface {
            Ok(s) => {
                console::log_1(&"surface created!".into());
            }
            Err(e) => {
                console::log_1(&format!("surface creation failed: {:?}", e).into());
            }
        }
        (surface.unwrap(), render_canvas.unwrap())
    }

    //#[cfg(target_arch = "wasm32")]
    async fn create_adapter<'a>(
        instance: &Arc<wgpu::Instance>,
        surface: &'a wgpu::Surface<'a>,
    ) -> wgpu::Adapter {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(surface),
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
            })
            .await;
        match &adapter {
            Ok(a) => {
                console::log_1(&"adapter created!".into());
            }
            Err(e) => {
                console::log_1(&format!("adapter creation failed: {:?}", e).into());
            }
        }
        adapter.unwrap()
    }

    async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        let features = adapter.features();
        println!("Supported adapter features:");
        for feature in wgpu::Features::all().iter() {
            if features.contains(feature) {
                console::log_1(&format!(" - {:?}", feature).into());
                //println!("  - {:?}", feature);
            }
        }

        let r_device = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: adapter.features(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,
                memory_hints: wgpu::MemoryHints::Performance, // or MemoryUsage, or Manual { ... }
                trace: wgpu::Trace::Off,
            })
            .await;
        match &r_device {
            Ok(d) => {
                console::log_1(&"device and queue created!".into());
            }
            Err(e) => {
                console::log_1(&format!("device and queue creation failed: {:?}", e).into());
            }
        }
        let d = r_device.unwrap();
        (d.0, d.1)
    }

    async fn config_surface<'a>(
        device: &wgpu::Device,
        adapter: &wgpu::Adapter,
        surface: &'a wgpu::Surface<'a>,
        canvas: &HtmlCanvasElement,
    ) -> wgpu::TextureFormat {
        let surface_caps = surface.get_capabilities(adapter);
        console::log_1(&"Supported texture formats:".into());
        for format in &surface_caps.formats {
            console::log_1(&format!("{:?}", format).into());
        }
        console::log_1(&"Supported present_modes:".into());
        for format in &surface_caps.present_modes {
            console::log_1(&format!("{:?}", format).into());
        }
        console::log_1(&"Supported CompositeAlphaMode:".into());
        for format in &surface_caps.alpha_modes {
            console::log_1(&format!("{:?}", format).into());
        }
        console::log_1(&"Supported TextureUsages:".into());
        for format in surface_caps.usages {
            console::log_1(&format!("{:?}", format).into());
        }
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format.clone(),
                width: canvas.width(),
                height: canvas.height(),
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![surface_format],
                desired_maximum_frame_latency: 2,
            },
        );
        console::log_1(&"surface configured!".into());
        surface_format.clone()
    }

    pub async fn create_shader_module(
        &self,
    ) -> (wgpu::ShaderModule, wgpu::ShaderModule, wgpu::ShaderModule) {
        let vert = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Vertex Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("vertex_shader.wgsl").into()),
            });
        console::log_1(&"vert shader module loaded".into());
        let frag = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Unlit Fragment Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("fragment_shader.wgsl").into()),
            });
        console::log_1(&"fragment shader module loaded".into());
        let frag_pbr = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("PBR Fragment Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("pbr_fragment_shader.wgsl").into()),
            });
        console::log_1(&"PBR fragment shader module loaded".into());
        console::log_1(&"shader modules configured!".into());
        (vert, frag, frag_pbr)
    }
}

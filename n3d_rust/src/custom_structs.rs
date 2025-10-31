use bytemuck::{Pod, Zeroable};
pub mod CustomStructs {
    #[derive(Debug, Clone)]
    pub struct Model {
        pub vertex_buffer: wgpu::Buffer,
        pub indice_buffer: wgpu::Buffer,
        pub indice_count: u32,
    }
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Vertex {
        pub position: [f32; 3], // @location(0)
        pub normal: [f32; 3],   // @location(1)
        pub tangent: [f32; 4],  // @location(2)
        pub uv: [f32; 2],       // @location(3)
        pub color: [f32; 4],    // @location(4)
        pub joints: [u32; 4],   // @location(5)
        pub weights: [f32; 4],  // @location(6)
    }
    #[repr(C)]
    #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct CameraUniform {
        pub view_proj: [[f32; 4]; 4],
        pub model: [[f32; 4]; 4],
        pub camera_pos: [f32; 3],
        pub _pad: f32, // padding for 16-byte alignment
    }
    #[repr(C)]
    #[derive(Copy, Clone, bytemuck:: Pod, bytemuck::Zeroable)]
    pub struct LightUniform {
        pub direction: [f32; 3],
        pub _pad1: f32,
        pub color: [f32; 3],
        pub _pad2: f32,
    }
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Material {
        pub base_color_factor: [f32; 4],
        pub alpha_cutoff: f32,
        pub alpha_mode: u32,
        pub double_sided: u32,
        // padding to 16-byte align struct
        pub _padding: [u32; 1],
    }
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct MaterialPBR {
        pub base_color_factor: [f32; 4], // default [1,1,1,1]
        pub emissive_factor: [f32; 3],   // default [0,0,0]
        pub metallic_factor: f32,        // default 1.0
        pub roughness_factor: f32,       // default 1.0

        // texture indices into your Texture struct arrays
        pub base_color_texture: u32,
        pub metallic_roughness_texture: u32,
        pub normal_texture: u32,
        pub occlusion_texture: u32,
        pub emissive_texture: u32,

        pub alpha_cutoff: f32,
        pub alpha_mode: u32,
        pub double_sided: u32,
        pub _padding: [u32; 1],
    }

    pub struct Texture {
        pub textures: Vec<wgpu::Texture>,
        pub views: Vec<wgpu::TextureView>,
        pub samplers: Vec<wgpu::Sampler>,
    }
}

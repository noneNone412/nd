use super::custom_structs::CustomStructs;
use glam::{Mat4, Vec3};
use gltf::buffer::Data;
use gltf::image::Format;
use gltf::mesh::util::indices;
use gltf::Gltf;
use gltf::{import_slice, Document};
use gltf_json::material;
use std::num::NonZeroU32;
use std::primitive;
use wasm_bindgen::JsValue;
use web_sys::console;
use wgpu::util::DeviceExt;
use wgpu::BindGroup;

pub struct ModelExec {}

impl ModelExec {
    pub async fn new() -> Self {
        ModelExec {}
    }
    pub async fn parse_gltf_n_glb(&self, gltf_bytes: &[u8]) -> gltf::Gltf {
        let model = Gltf::from_slice(gltf_bytes).map_err(|e| e.to_string());
        let (document, buffers, images) =
            gltf::import_slice(gltf_bytes).expect("Failed to parse GLB");
        match &model {
            Ok(m) => {
                console::log_1(&"model parsed faltu!".into());
            }
            Err(e) => {
                console::log_1(&format!("model faltu parsing failed: {:?}", e).into());
            }
        }
        model.unwrap()
    }
    fn is_Model_PBR(&self, document: &gltf::Document) -> bool {
        for material in document.materials() {
            let pbr = material.pbr_metallic_roughness();
            let has_base_color_texture = pbr.base_color_texture().is_some();
            let has_metallic_texture = pbr.metallic_roughness_texture().is_some();
            let metallic_non_default = pbr.metallic_factor() != 1.0;
            let roughness_non_default = pbr.roughness_factor() != 1.0;
            if has_base_color_texture
                || has_metallic_texture
                || metallic_non_default
                || roughness_non_default
            {
                return true;
            }
        }
        return false;
    }
    pub async fn build_model(
        &self,
        gltf_bytes: &[u8],
        device: wgpu::Device,
        queue: wgpu::Queue,
    ) -> (
        bool,
        CustomStructs::Model,
        Vec<CustomStructs::Material>,
        Vec<CustomStructs::MaterialPBR>,
        CustomStructs::Texture,
    ) {
        // --- Parse glb ---
        let parsed_glb: Result<(Document, Vec<Data>, Vec<gltf::image::Data>), gltf::Error> =
            gltf::import_slice(gltf_bytes);

        let (document, buffers, images) = match parsed_glb {
            Ok((doc, buf, img)) => {
                console::log_1(&"parsed glb file".into());
                (doc, buf, img)
            }
            Err(e) => {
                console::log_1(&format!("error parsing glb: {:?}", e).into());
                panic!("Failed to parse GLB file");
            }
        };
        let isPbr = self.is_Model_PBR(&document);
        console::log_1(&format!("isPbr: {:?}", isPbr).into());

        // --- Geometry ---
        let mut vertices: Vec<CustomStructs::Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut meshC = 0;
        let mut primitiveC = 0;
        for mesh in document.meshes() {
            console::log_1(&format!("mesh is : {:?}", meshC).into());
            meshC += 1;
            for primitive in mesh.primitives() {
                console::log_1(&format!("primitive is : {:?}", primitiveC).into());
                primitiveC += 1;
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                console::log_1(&format!("reader is : {:?}", "?").into());
                // positions
                let positions: Vec<[f32; 3]> = reader
                    .read_positions()
                    .map(|iter| iter.collect())
                    .unwrap_or_default();
                console::log_1(&format!("position is : {:?}", "?").into());
                // normals
                let normals: Vec<[f32; 3]> = reader
                    .read_normals()
                    .map(|iter| iter.collect())
                    .unwrap_or_default();
                // tangents
                let tangents: Vec<[f32; 4]> = reader
                    .read_tangents()
                    .map(|iter| iter.collect())
                    .unwrap_or_default();

                console::log_1(&format!("tangent is : {:?}", "?").into());
                let uvs: Vec<[f32; 2]> = reader
                    .read_tex_coords(0)
                    .map(|tc| tc.into_f32().collect())
                    .unwrap_or_default();
                console::log_1(&format!("uv is : {:?}", "?").into());
                let colors: Vec<[f32; 4]> = reader
                    .read_colors(0)
                    .map(|c| c.into_rgba_f32().collect())
                    .unwrap_or_default();
                console::log_1(&format!("color is : {:?}", "?").into());
                let joints: Vec<[u32; 4]> = reader
                    .read_joints(0)
                    .map(|j| {
                        j.into_u16()
                            .map(|a| [a[0] as u32, a[1] as u32, a[2] as u32, a[3] as u32])
                            .collect()
                    })
                    .unwrap_or_default();
                console::log_1(&format!("joint is : {:?}", "?").into());
                let weights: Vec<[f32; 4]> = reader
                    .read_weights(0)
                    .map(|w| w.into_f32().collect())
                    .unwrap_or_default();
                console::log_1(&format!("weight is : {:?}", "?").into());
                // indices
                let primitive_indices: Vec<u32> = if let Some(read_indices) = reader.read_indices()
                {
                    match read_indices {
                        gltf::mesh::util::ReadIndices::U8(iter) => iter.map(|i| i as u32).collect(),
                        gltf::mesh::util::ReadIndices::U16(iter) => {
                            iter.map(|i| i as u32).collect()
                        }
                        gltf::mesh::util::ReadIndices::U32(iter) => iter.collect(),
                    }
                } else {
                    (0..positions.len() as u32).collect()
                };
                let index_offset = vertices.len() as u32;
                indices.extend(primitive_indices.iter().map(|i| i + index_offset));
                console::log_1(
                    &format!(
                        "indices is {:?} and positions.len():  {:?}",
                        indices.len(),
                        positions.len()
                    )
                    .into(),
                );
                console::log_1(&format!("positions length : {:?}", positions.len()).into());
                console::log_1(&format!("normal length : {:?}", normals.len()).into());
                console::log_1(&format!("tangent length : {:?}", tangents.len()).into());
                console::log_1(&format!("uv length : {:?}", uvs.len()).into());
                console::log_1(&format!("color length : {:?}", colors.len()).into());
                console::log_1(&format!("joints length : {:?}", joints.len()).into());
                console::log_1(&format!("weights length : {:?}", weights.len()).into());
                for i in 0..positions.len() {
                    vertices.push(CustomStructs::Vertex {
                        position: positions[i],
                        normal: normals.get(i).copied().unwrap_or([0.0, 0.0, 0.0]),
                        tangent: tangents.get(i).copied().unwrap_or([0.0, 0.0, 0.0, 1.0]),
                        uv: uvs.get(i).copied().unwrap_or([0.0, 0.0]),
                        color: colors.get(i).copied().unwrap_or([1.0, 1.0, 1.0, 1.0]),
                        joints: joints.get(i).copied().unwrap_or([0, 0, 0, 0]),
                        weights: weights.get(i).copied().unwrap_or([1.0, 0.0, 0.0, 0.0]),
                    });
                }
            }
        }
        // --- Upload to GPU buffers ---
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let indice_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let model: CustomStructs::Model = CustomStructs::Model {
            vertex_buffer,
            indice_buffer,
            indice_count: indices.len() as u32,
        };
        // --- Materials ---
        let mut materials: Vec<CustomStructs::Material> = Vec::new();
        let mut materialPbr: Vec<CustomStructs::MaterialPBR> = Vec::new();
        if isPbr {
            console::log_1(&"r1".into());
            for mat in document.materials() {
                let pbr = mat.pbr_metallic_roughness();
                let alpha_mode = match mat.alpha_mode() {
                    gltf::material::AlphaMode::Opaque => 0,
                    gltf::material::AlphaMode::Mask => 1,
                    gltf::material::AlphaMode::Blend => 2,
                };
                console::log_1(&"r2".into());
                // Lookup texture indices (if available)
                let base_color_tex = pbr.base_color_texture().map(|t| t.texture().index() as u32);
                let metallic_roughness_tex = pbr
                    .metallic_roughness_texture()
                    .map(|t| t.texture().index() as u32);
                let normal_tex = mat.normal_texture().map(|t| t.texture().index() as u32);
                let occlusion_tex = mat.occlusion_texture().map(|t| t.texture().index() as u32);
                let emissive_tex = mat.emissive_texture().map(|t| t.texture().index() as u32);

                // Fill struct
                let material: CustomStructs::MaterialPBR = CustomStructs::MaterialPBR {
                    base_color_factor: pbr.base_color_factor(),
                    metallic_factor: pbr.metallic_factor(),
                    roughness_factor: pbr.roughness_factor(),
                    emissive_factor: mat.emissive_factor(),

                    base_color_texture: base_color_tex.unwrap_or(0),
                    metallic_roughness_texture: metallic_roughness_tex.unwrap_or(0),
                    normal_texture: normal_tex.unwrap_or(0),
                    occlusion_texture: occlusion_tex.unwrap_or(0),
                    emissive_texture: emissive_tex.unwrap_or(0),

                    alpha_cutoff: mat.alpha_cutoff().unwrap_or(0.5),
                    alpha_mode,
                    double_sided: if mat.double_sided() { 1 } else { 0 },
                    _padding: [0],
                };

                console::log_1(&"r3".into());
                materialPbr.push(material);
            }
        } else {
            for mat in document.materials() {
                let base_color_factor = [1.0, 1.0, 1.0, 1.0]; // pure white

                let alpha_mode = match mat.alpha_mode() {
                    gltf::material::AlphaMode::Opaque => 0,
                    gltf::material::AlphaMode::Mask => 1,
                    gltf::material::AlphaMode::Blend => 2,
                };

                materials.push(CustomStructs::Material {
                    base_color_factor,
                    alpha_cutoff: 0.0, // ignore mask cutoff
                    alpha_mode,
                    double_sided: 0, // default single-sided
                    _padding: [0],
                });

                // Base Color
                if let Some(tex) = mat.normal_texture() {
                    console::log_1(
                        &format!("NormalTexture: index {}", tex.texture().index()).into(),
                    );
                }

                // Metallic-Roughness
                let pbr = mat.pbr_metallic_roughness();

                // Base Color
                if let Some(tex) = pbr.base_color_texture() {
                    console::log_1(
                        &format!("BaseColorTexture: index {}", tex.texture().index()).into(),
                    );
                }

                if let Some(tex) = pbr.metallic_roughness_texture() {
                    console::log_1(
                        &format!("MetallicRoughnessTexture: index {}", tex.texture().index())
                            .into(),
                    );
                }

                // Normal map
                if let Some(tex) = mat.normal_texture() {
                    console::log_1(
                        &format!("NormalTexture: index {}", tex.texture().index()).into(),
                    );
                }

                // Occlusion
                if let Some(tex) = mat.occlusion_texture() {
                    console::log_1(
                        &format!("OcclusionTexture: index {}", tex.texture().index()).into(),
                    );
                }

                // Emissive
                if let Some(tex) = mat.emissive_texture() {
                    console::log_1(
                        &format!("EmissiveTexture: index {}", tex.texture().index()).into(),
                    );
                }

                console::log_1(&"Factors:".into());
                console::log_1(&format!("  BaseColorFactor: {:?}", pbr.base_color_factor()).into());
                console::log_1(&format!("  MetallicFactor: {}", pbr.metallic_factor()).into());
                console::log_1(&format!("  RoughnessFactor: {}", pbr.roughness_factor()).into());
                console::log_1(&format!("  EmissiveFactor: {:?}", mat.emissive_factor()).into());
            }
        }

        const MAX_TEXTURES: usize = 16;
        // --- Textures ---
        let mut textures: CustomStructs::Texture = CustomStructs::Texture {
            textures: Vec::new(),
            views: Vec::new(),
            samplers: Vec::new(),
        };

        for (i, image) in images.iter().enumerate() {
            if i >= MAX_TEXTURES {
                break; // clamp to fragment shader array size
            }
            console::log_1(&format!("image format: {:?}", image.format).into());
            let mut rgba_pixels = Vec::with_capacity((image.width * image.height * 4) as usize);
            match image.format {
                Format::R8G8B8 => {
                    for chunk in image.pixels.chunks(3) {
                        rgba_pixels.push(chunk[0]);
                        rgba_pixels.push(chunk[1]);
                        rgba_pixels.push(chunk[2]);
                        rgba_pixels.push(255); // opaque alpha
                    }
                }
                Format::R8G8B8A8 => {
                    for chunk in image.pixels.chunks(4) {
                        rgba_pixels.extend_from_slice(chunk);
                    }
                }
                _ => panic!("Unsupported image format: {:?}", image.format),
            }

            console::log_1(&format!("rgba_pixels: {:?}", rgba_pixels.len()).into());

            // Compute padded bytes_per_row for WebGPU (must be multiple of 256)
            let bytes_per_row = (((4 * image.width as u32 + 255) / 256) * 256) as usize;

            // Create padded buffer
            let mut padded_data = vec![0u8; bytes_per_row * image.height as usize];
            for y in 0..image.height {
                let src_start = (y as usize * image.width as usize * 4);
                let src_end = src_start + (image.width as usize * 4);
                let dst_start = y as usize * bytes_per_row;
                let dst_end = dst_start + (image.width as usize * 4);

                padded_data[dst_start..dst_end].copy_from_slice(&rgba_pixels[src_start..src_end]);
            }

            let tex_size = wgpu::Extent3d {
                width: image.width as u32,
                height: image.height as u32,
                depth_or_array_layers: 1,
            };

            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some(&format!("Texture {}", i)),
                size: tex_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &padded_data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row as u32),
                    rows_per_image: Some(image.height as u32),
                },
                tex_size,
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some(&format!("Sampler {}", i)),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });

            textures.textures.push(texture);
            textures.views.push(view);
            textures.samplers.push(sampler);
        }
        // Fill remaining texture slots with 1x1 white dummy textures
        while textures.textures.len() < MAX_TEXTURES {
            let tex_size = wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            };
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Dummy Texture"),
                size: tex_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

            let white_pixel = [255u8, 255, 255, 255];
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &white_pixel,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                tex_size,
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

            textures.textures.push(texture);
            textures.views.push(view);
            textures.samplers.push(sampler);
        }
        console::log_1(&"model_build run successful".into());
        (isPbr, model, materials, materialPbr, textures)
    }

    pub fn extract_indices(gltf: &Gltf, buffers: &Vec<Data>) -> Vec<u32> {
        let mut indices_out = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                if let Some(indices) = reader.read_indices() {
                    match indices {
                        gltf::mesh::util::ReadIndices::U8(iter) => {
                            indices_out.extend(iter.map(|i| i as u32));
                        }
                        gltf::mesh::util::ReadIndices::U16(iter) => {
                            indices_out.extend(iter.map(|i| i as u32));
                        }
                        gltf::mesh::util::ReadIndices::U32(iter) => {
                            indices_out.extend(iter);
                        }
                    }
                } else {
                    // No indices → generate a simple [0..N] list
                    let vertex_count = reader.read_positions().unwrap().count() as u32;
                    indices_out.extend(0..vertex_count);
                }
            }
        }

        indices_out
    }

    pub async fn bindGroupLayoutPBR_all(
        &self,
        device: &wgpu::Device,
    ) -> (
        wgpu::BindGroupLayout,
        wgpu::BindGroupLayout,
        wgpu::BindGroupLayout,
    ) {
        // Group 0: Camera
        let camera_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Group 1: Material uniforms + textures (5 textures + 5 samplers)
        let material_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Material BGL"),
            entries: &[
                // Uniform buffer (Material factors)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // BaseColor
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // MetallicRoughness
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Normal
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Occlusion
                wgpu::BindGroupLayoutEntry {
                    binding: 7,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 8,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Emissive
                wgpu::BindGroupLayoutEntry {
                    binding: 9,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 10,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Group 2: Directional light (optional, can be simplified for now)
        let light_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Light BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        (camera_bgl, material_bgl, light_bgl)
    }

    // Camera buffer
    fn buffer_cameraUniform_pbr(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let rotation = Mat4::from_rotation_y(std::f32::consts::PI / 4.0);

        let uniforms = CustomStructs::CameraUniform {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
            model: glam::Mat4::IDENTITY.to_cols_array_2d(),
            camera_pos: [1.0, 1.0, 0.0], // initial camera position
            _pad: 0.0,                   // padding
        };
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    // Material buffer
    fn buffer_material_pbr(
        &self,
        device: &wgpu::Device,
        materials: Vec<CustomStructs::MaterialPBR>,
    ) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::cast_slice(&materials),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    // Light buffer (simple directional light)
    fn buffer_light_pbr(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let light = CustomStructs::LightUniform {
            direction: [0.0, -1.0, 0.0],
            _pad1: 0.0,
            color: [0.0, 0.0, 0.0],
            _pad2: 0.0,
        };
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light Buffer"),
            contents: bytemuck::cast_slice(&[light]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub async fn create_depth_texture(
        &self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub async fn buildPipelinePBR(
        &self,
        device: &wgpu::Device,
        render_format: wgpu::TextureFormat,
        camera_bgl: &wgpu::BindGroupLayout,
        material_bgl: &wgpu::BindGroupLayout,
        light_bgl: &wgpu::BindGroupLayout,
        vertShader: &wgpu::ShaderModule,
        fragPbrShader: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("PBR Pipeline Layout"),
            bind_group_layouts: &[camera_bgl, material_bgl, light_bgl],
            push_constant_ranges: &[],
        });
        // vertex layout
        let vertex_layouts: &[wgpu::VertexBufferLayout] = &[wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<CustomStructs::Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex, // one vertex per step
            attributes: &[
                // POSITION @location(0)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // NORMAL @location(1)
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // TANGENT @location(2)
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // TEXCOORD_0 @location(3)
                wgpu::VertexAttribute {
                    offset: 40,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // COLOR_0 @location(4)
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // JOINTS_0 @location(5) (u32x4)
                wgpu::VertexAttribute {
                    offset: 64,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Uint32x4,
                },
                // WEIGHTS_0 @location(6)
                wgpu::VertexAttribute {
                    offset: 80,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }];
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("PBR Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: vertShader,
                entry_point: Some("vs_main"),
                buffers: vertex_layouts,
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants: &[],
                    zero_initialize_workgroup_memory: true,
                },
            },
            fragment: Some(wgpu::FragmentState {
                module: fragPbrShader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: render_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants: &[],
                    zero_initialize_workgroup_memory: true,
                },
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

    pub async fn bindGroupPBR_all(
        &self,
        device: &wgpu::Device,
        materials: Vec<CustomStructs::MaterialPBR>,
        textures: &CustomStructs::Texture,
        camera_bgl: wgpu::BindGroupLayout,
        material_bgl: wgpu::BindGroupLayout,
        light_bgl: wgpu::BindGroupLayout,
    ) -> (wgpu::BindGroup, wgpu::BindGroup, wgpu::BindGroup) {
        //let (camera_bgl, material_bgl, light_bgl) = self.bindGroupLayoutPBR_all(device);

        // === Camera bind group ===
        let camera_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera BG"),
            layout: &camera_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.buffer_cameraUniform_pbr(device).as_entire_binding(),
            }],
        });

        // === Material uniform buffer ===
        let material_buffer = self.buffer_material_pbr(device, materials);

        // === Default 1x1 texture + sampler ===
        let default_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("DefaultTexture"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let default_view = default_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let default_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        // Fill missing textures with default
        let base_color_tex = textures.views.get(0).unwrap_or(&default_view);
        let base_color_smp = textures.samplers.get(0).unwrap_or(&default_sampler);
        let metallic_tex = textures.views.get(1).unwrap_or(&default_view);
        let metallic_smp = textures.samplers.get(1).unwrap_or(&default_sampler);
        let normal_tex = textures.views.get(2).unwrap_or(&default_view);
        let normal_smp = textures.samplers.get(2).unwrap_or(&default_sampler);
        let occlusion_tex = textures.views.get(3).unwrap_or(&default_view);
        let occlusion_smp = textures.samplers.get(3).unwrap_or(&default_sampler);
        let emissive_tex = textures.views.get(4).unwrap_or(&default_view);
        let emissive_smp = textures.samplers.get(4).unwrap_or(&default_sampler);

        // === Material + textures bind group ===
        let material_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material + Textures BG"),
            layout: &material_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: material_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(base_color_tex),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(base_color_smp),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(metallic_tex),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(metallic_smp),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::TextureView(normal_tex),
                },
                wgpu::BindGroupEntry {
                    binding: 6,
                    resource: wgpu::BindingResource::Sampler(normal_smp),
                },
                wgpu::BindGroupEntry {
                    binding: 7,
                    resource: wgpu::BindingResource::TextureView(occlusion_tex),
                },
                wgpu::BindGroupEntry {
                    binding: 8,
                    resource: wgpu::BindingResource::Sampler(occlusion_smp),
                },
                wgpu::BindGroupEntry {
                    binding: 9,
                    resource: wgpu::BindingResource::TextureView(emissive_tex),
                },
                wgpu::BindGroupEntry {
                    binding: 10,
                    resource: wgpu::BindingResource::Sampler(emissive_smp),
                },
            ],
        });

        // === Light bind group ===
        let light_buffer = self.buffer_light_pbr(device); // create your light uniform buffer
        let light_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Light BG"),
            layout: &light_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
        });

        (camera_bg, material_bg, light_bg)
    }

    pub fn print_gltf(&self, model: &gltf::Gltf) {
        console::log_1(&"========== glTF Contents ==========".into());
        console::log_1(&format!("Scenes: {:?}", model.scenes().count()).into());
        /*
        for (i, scene) in model.scenes().enumerate() {
            console::log_1(&format!("Scene {}: {:?}", i, scene.name()).into());
            for node in scene.nodes() {
                let pad = " ".repeat(2 * 2);
                console::log_1(&format!("{}Node {}: {:?}", pad, node.index(), node.name()).into());

                if let Some(mesh) = node.mesh() {
                    console::log_1(&format!("{}  Mesh: {:?}", pad, mesh.name()).into());
                }
                if let Some(camera) = node.camera() {
                    console::log_1(&format!("{}  Camera: {:?}", pad, camera.name()).into());
                }
                if let Some(skin) = node.skin() {
                    console::log_1(&format!("{}  Skin: {:?}", pad, skin.name()).into());
                }
            }
        }If the texture is embedd
        */
        // Materials
        console::log_1(&format!("Materials: {}", model.materials().count()).into());
        /* for (i, mat) in model.materials().enumerate() {
            console::log_1(&format!("  Material {}: {:?}", i, mat.name()).into());
        } */

        // Textures
        console::log_1(&format!("Textures: {}", model.textures().count()).into());
        /* for (i, tex) in model.textures().enumerate() {
            console::log_1(&format!("  Texture {}: source = {:?}", i, tex.source().name()).into());
        } */

        // Images
        console::log_1(&format!("Images: {}", model.images().count()).into());
        /* for (i, img) in model.images().enumerate() {
            console::log_1(&format!("  Image {}: {:?}", i, img.name()).into());
        } */

        // Samplers
        console::log_1(&format!("Samplers: {}", model.samplers().count()).into());
        /* for (i, samp) in model.samplers().enumerate() {
            console::log_1(
                &format!(
                    "  Sampler {}: mag={:?}, min={:?}, wrap_s={:?}, wrap_t={:?}",
                    i,
                    samp.mag_filter(),
                    samp.min_filter(),
                    samp.wrap_s(),
                    samp.wrap_t()
                )
                .into(),
            );
        } */

        // Buffers
        console::log_1(&format!("Buffers: {}", model.buffers().count()).into());
        /* for (i, buf) in model.buffers().enumerate() {
            console::log_1(&format!("  Buffer {}: byte_length={}", i, buf.length()).into());
        } */

        // Accessors
        console::log_1(&format!("Accessors: {}", model.accessors().count()).into());
        /* for (i, accessor) in model.accessors().enumerate() {
            console::log_1(
                &format!(
                    "  Accessor {}: type={:?}, count={}, component_type={:?}, view={:?}",
                    i,
                    accessor.dimensions(),
                    accessor.count(),
                    accessor.data_type(),
                    accessor.view().map(|v| v.index())
                )
                .into(),
            );
        } */

        // Animations
        console::log_1(&format!("Animations: {}", model.animations().count()).into());
        /* for (i, anim) in model.animations().enumerate() {
            console::log_1(&format!("  Animation {}: {:?}", i, anim.name()).into());
            for channel in anim.channels() {
                let target = channel.target();
                console::log_1(&format!("    Channel: node={:?}", target.node().index()).into());
            }
        } */

        // Skins
        console::log_1(&format!("Skins: {}", model.skins().count()).into());
        /* for (i, skin) in model.skins().enumerate() {
            console::log_1(
                &format!(
                    "  Skin {}: {:?}, joints={}",
                    i,
                    skin.name(),
                    skin.joints().count()
                )
                .into(),
            );
        } */
        console::log_1(&"===================================".into());
    }
}

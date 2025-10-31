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

pub struct ModelExec2 {}

impl ModelExec2 {
    pub async fn new() -> Self {
        ModelExec2 {}
    }

    pub async fn build_model(&self) {}
}

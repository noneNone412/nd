#![allow(warnings)]
mod camera_math;
mod config_pipeline;
mod custom_structs;
mod model_exec;
mod render_loop;
mod saved_state;
use glam::Vec3;
use gltf::Gltf;
mod model_exec2;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
struct Renderer {
    saved_gpu: saved_state::SavedState,
    model_manager: model_exec::ModelExec,
    pipeline: config_pipeline::ConfigPipeline,
    render_manager: render_loop::RenderLoop,
    camera: camera_math::Camera,
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub async fn new(height: u32, width: u32) -> js_sys::Promise {
        // Convert the Rust Future into a JS Promise
        future_to_promise(async move {
            let saved_gpu = saved_state::SavedState::new().await;
            let model_manager = model_exec::ModelExec::new().await;
            let pipeline = config_pipeline::ConfigPipeline::new().await;
            let render_manager = render_loop::RenderLoop::new().await;
            let camera = camera_math::Camera::new(
                Vec3::new(0.0, 0.0, 5.0),     // eye
                Vec3::ZERO,                   // target
                width as f32 / height as f32, // aspect
            )
            .await;
            // Wrap Manager in JsValue
            let render = Renderer {
                saved_gpu,
                model_manager,
                pipeline,
                render_manager,
                camera,
            };
            // Return Ok(JsValue) as expected by future_to_promise
            Ok(JsValue::from(render))
        })
    }
    pub async fn render(&self, bytes: &[u8]) -> JsValue {
        use gltf::texture;

        use crate::custom_structs::CustomStructs::MaterialPBR;

        let model = self.model_manager.parse_gltf_n_glb(bytes).await;
        self.model_manager.print_gltf(&model);
        let (vertShader, fragShader, pbrFragShader) = self.saved_gpu.create_shader_module().await;
        console::log_1(&"shaders created".into());
        let (isPbr, model, material, material_pbr, texture) = self
            .model_manager
            .build_model(
                bytes,
                self.saved_gpu.get_device(),
                self.saved_gpu.get_queue(),
            )
            .await;
        let (camera_bgl, material_bgl, light_bgl) = self
            .model_manager
            .bindGroupLayoutPBR_all(&self.saved_gpu.get_device())
            .await;
        let (camera_bg, material_bg, light_bg) = self
            .model_manager
            .bindGroupPBR_all(
                &self.saved_gpu.get_device(),
                material_pbr,
                &texture,
                camera_bgl.clone(),
                material_bgl.clone(),
                light_bgl.clone(),
            )
            .await;
        let model_pipeline = self
            .model_manager
            .buildPipelinePBR(
                &self.saved_gpu.get_device(),
                self.saved_gpu.get_surface_format(),
                &camera_bgl,
                &material_bgl,
                &light_bgl,
                &vertShader,
                &pbrFragShader,
            )
            .await;
        let depth_texture = self
            .model_manager
            .create_depth_texture(
                &self.saved_gpu.get_device(),
                self.saved_gpu
                    .get_surface()
                    .get_current_texture()
                    .unwrap()
                    .texture
                    .size()
                    .width,
                self.saved_gpu
                    .get_surface()
                    .get_current_texture()
                    .unwrap()
                    .texture
                    .size()
                    .height,
            )
            .await;
        self.render_manager
            .render2(
                self.saved_gpu.get_device(),
                self.saved_gpu.get_queue(),
                model_pipeline,
                model,
                camera_bg,
                material_bg,
                light_bg,
                self.saved_gpu.get_surface(),
                &depth_texture,
            )
            .await;
        console::log_1(&"render function exited successfully".into());
        JsValue::NULL
    }
    pub async fn reconfigure_surface(&self) -> JsValue {
        console::log_1(&format!("reconfigure_surface caled: {:?}", 0).into());
        JsValue::NULL
    }
    pub async fn rotate() {}
    pub async fn zoom() -> JsValue {
        JsValue::NULL
    }
}

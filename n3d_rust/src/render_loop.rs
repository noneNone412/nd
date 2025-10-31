use crate::custom_structs::CustomStructs;
use wasm_bindgen::JsValue;
use web_sys::console;
use winit::window::CustomCursor;

pub struct RenderLoop {}

impl RenderLoop {
    pub async fn new() -> RenderLoop {
        Self {}
    }
    pub async fn render2(
        &self,
        device: wgpu::Device,
        queue: wgpu::Queue,
        pipeline: wgpu::RenderPipeline,
        model: CustomStructs::Model,
        camera_bind_group: wgpu::BindGroup,
        material_bind_group: wgpu::BindGroup,
        texture_bind_group: wgpu::BindGroup,
        surface: &wgpu::Surface<'static>,
        depth_texture: &wgpu::TextureView,
    ) -> JsValue {
        console::log_1(&"renderloop starts".into());

        //  Acquire next frame from swapchain
        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                console::log_1(&format!("could not acquire next frame: {:?}", e).into());
                return JsValue::NULL;
            }
        };

        let frame_view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        console::log_1(&"frame view created".into());

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Begin render pass with depth attachment
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store, // keep results for presenting
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), // far plane
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            //  Set pipeline and bind groups
            render_pass.set_pipeline(&pipeline);
            render_pass.set_bind_group(0, Some(&camera_bind_group), &[]);
            render_pass.set_bind_group(1, Some(&material_bind_group), &[]);
            render_pass.set_bind_group(2, Some(&texture_bind_group), &[]);

            // Set vertex/index buffers
            render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
            render_pass.set_index_buffer(model.indice_buffer.slice(..), wgpu::IndexFormat::Uint32);

            //  Draw
            render_pass.draw_indexed(0..model.indice_count, 0, 0..1);
        } // render_pass ends here

        // Submit commands
        queue.submit(Some(encoder.finish()));
        frame.present();

        JsValue::NULL
    }

    pub async fn rotate() {
       // queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
    pub async fn render(
        &self,
        device: wgpu::Device,
        queue: wgpu::Queue,
        pipeline: wgpu::RenderPipeline,
        model: CustomStructs::Model,
        camera_bind_group: wgpu::BindGroup,
        material_bind_group: wgpu::BindGroup,
        texture_bind_group: wgpu::BindGroup,
        surface: &wgpu::Surface<'static>,
        depth_texture: &wgpu::TextureView,
    ) -> JsValue {
        console::log_1(&"renderloop starts".into());
        // 1. Acquire next texture from the swapchain
        let frame = surface.get_current_texture();
        match &frame {
            Ok(f) => {
                console::log_1(&"Acquired next frame".into());
            }
            Err(e) => {
                console::log_1(&format!("could not acquire next frame: {:?}", e).into());
            }
        }
        let view = frame
            .as_ref()
            .unwrap()
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        console::log_1(&"view created".into());

        // 2. Create a command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        console::log_1(&"encoder created".into());

        console::log_1(&"render pass begins".into());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store, // keep results for presenting
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&pipeline);
            render_pass.set_bind_group(0, &camera_bind_group, &[]);
            render_pass.set_bind_group(1, &material_bind_group, &[]);
            render_pass.set_bind_group(2, &texture_bind_group, &[]);
            render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
            render_pass.set_index_buffer(model.indice_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..model.indice_count, 0, 0..1);

            console::log_1(&"render pass draws".into());
        } // <-- render_pass ends *here* when dropped
        console::log_1(&"render pass ends".into());

        queue.submit(Some(encoder.finish()));
        frame.unwrap().present();
        JsValue::NULL
    }
}

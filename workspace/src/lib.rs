mod vertex;
mod camera;
mod utils;
// mod advance;
mod component;

use gloo::utils::document;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use yew::platform::spawn_local;
use utils::request_animation_frame_extends::wait_request_animation_frame;
use crate::vertex::Vertex;
use crate::utils::init::INITIAL_WEBGPU_WITH_PROP_CANVAS;
use crate::component::all::Roots;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

async fn run() {
    let canvas = document().query_selector("canvas")
        .unwrap().unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
    let (
        config,
        device,
        queue,
        surface,
    ) = INITIAL_WEBGPU_WITH_PROP_CANVAS(&canvas).await;
    let vertex_buffer = Vertex::buffer(
        &device,
        "Vertex Buffer"
    );
    let shader = device.create_shader_module(
        wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader/shader.wgsl").into()),
        }
    );
    let render_pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        }
    );
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
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
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });
    loop {
        log::info!("render!");

        let output = match surface.get_current_texture() {
            Ok(output) => output,
            Err(e) => {
                log::error!("{e:?}");
                return;
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..3, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        wait_request_animation_frame().await;
    }
}

#[wasm_bindgen(start)]
fn main() {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("wgpu_to_canvas"));
    yew::Renderer::<Roots>::new().render();
    spawn_local(run());
}


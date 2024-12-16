use wgpu::{BufferAddress, Device};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct CubeVertex {
    position : [f32; 4],
    color : [f32; 4],
    uv : [f32; 2],
}

impl CubeVertex {
    pub const VERTEX_SIZE: usize = std::mem::size_of::<Self>();
    pub const VERTEX_COUNT: usize = Self::VERTICES.len();

    pub fn vertex_buffer(device: &wgpu::Device, cube_vertices: &[CubeVertex]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(cube_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn pipe_line(device: &wgpu::Device,config: &wgpu::SurfaceConfiguration ) -> wgpu::RenderPipeline{
        let attributes = wgpu::vertex_attr_array![
            0 => Float32x4,
            1 => Float32x4,
            2 => Float32x2,
        ];
        device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts: &[],
                        push_constant_ranges: &[],
                    }
                )),
                vertex: wgpu::VertexState {
                    module: &device.create_shader_module(
                        wgpu::ShaderModuleDescriptor {
                            label: Some("Vertex Shader"),
                            source: wgpu::ShaderSource::Wgsl(
                                include_str!("../../shader/shader.wgsl").into()
                            )
                        }
                    ),
                    entry_point: Some("vs_main"),
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: Self::VERTEX_SIZE as BufferAddress,
                            attributes: attributes.as_slice(),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: Default::default(),
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: Default::default(),
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24Plus,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &device.create_shader_module(
                        wgpu::ShaderModuleDescriptor {
                            label: Some("Vertex Shader"),
                            source: wgpu::ShaderSource::Wgsl(
                                include_str!("../../shader/shader.wgsl").into()
                            )
                        }
                    ),
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })
                    ],
                }),
                multiview: None,
                cache: None,
            }
        )
    }
    pub const VERTICES: &'static [CubeVertex] = &[
        CubeVertex { position: [1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0, 1.0], uv: [1.0, 1.0] },
        CubeVertex { position: [-1.0, -1.0, -1.0, 1.0], color: [0.0, 0.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0, 1.0], uv: [0.0, 0.0] },
        CubeVertex { position: [1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [-1.0, -1.0, -1.0, 1.0], color: [0.0, 0.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [1.0, 1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 1.0, 1.0], uv: [1.0, 1.0] },
        CubeVertex { position: [1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [1.0, 1.0, -1.0, 1.0], color: [1.0, 1.0, 0.0, 1.0], uv: [0.0, 0.0] },
        CubeVertex { position: [1.0, 1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [1.0, 1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], uv: [1.0, 1.0] },
        CubeVertex { position: [1.0, 1.0, -1.0, 1.0], color: [1.0, 1.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0, 1.0], uv: [0.0, 0.0] },
        CubeVertex { position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [1.0, 1.0, -1.0, 1.0], color: [1.0, 1.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 1.0, 1.0], uv: [1.0, 1.0] },
        CubeVertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [-1.0, -1.0, -1.0, 1.0], color: [0.0, 0.0, 0.0, 1.0], uv: [0.0, 0.0] },
        CubeVertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [1.0, 1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 1.0, 1.0], uv: [1.0, 1.0] },
        CubeVertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 0.0, 1.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 1.0, 1.0], uv: [0.0, 0.0] },
        CubeVertex { position: [1.0, 1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [-1.0, -1.0, -1.0, 1.0], color: [0.0, 0.0, 0.0, 1.0], uv: [1.0, 1.0] },
        CubeVertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0, 1.0], uv: [1.0, 0.0] },
        CubeVertex { position: [1.0, 1.0, -1.0, 1.0], color: [1.0, 1.0, 0.0, 1.0], uv: [0.0, 0.0] },
        CubeVertex { position: [1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0, 1.0], uv: [0.0, 1.0] },
        CubeVertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0, 1.0], uv: [1.0, 0.0] },
    ];

}

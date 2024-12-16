use wgpu::{BindGroupLayout, SurfaceConfiguration};

#[repr(C)]
#[derive(Copy, Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct Uniform {
    pub(crate) transform: [[f32;4];4]
}

impl Uniform {
    pub const UNIFORM_BUFFER_SIZE: usize = std::mem::size_of::<Self>();
    pub fn buffer(device: &wgpu::Device,label: &str)-> wgpu::Buffer {
        let desc = wgpu::BufferDescriptor {
            label: Some(label),
            size: Self::UNIFORM_BUFFER_SIZE as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        };
        return device.create_buffer(&desc);
    }

    pub fn bind_group(device: &wgpu::Device,label: &str,uniform: &wgpu::Buffer) ->
    (wgpu::BindGroupLayout,wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(
          &wgpu::BindGroupLayoutDescriptor {
              label: None,
              entries: &[
                  wgpu::BindGroupLayoutEntry {
                      binding: 0,
                      visibility: wgpu::ShaderStages::VERTEX,
                      ty: wgpu::BindingType::Buffer {
                          ty: wgpu::BufferBindingType::Uniform,
                          has_dynamic_offset: false,
                          min_binding_size: None
                      },
                      count: None
                  }
              ]
          }
        );
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform.as_entire_binding()
                }]
            }
        );
        return (layout,bind_group);
    }
    pub fn pipeline_layout(device: &wgpu::Device, bind_group_layout: BindGroupLayout) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cube Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        })
    }

    pub fn depth_texture(device: &wgpu::Device,surface_configuration: &wgpu::SurfaceConfiguration,label: &str) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: surface_configuration.width,
                height: surface_configuration.height,
                depth_or_array_layers: 1
            },
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            ..Default::default()
        })
    }
}

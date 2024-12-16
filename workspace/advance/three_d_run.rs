use crate::advance::cube::CubeVertex;
use crate::advance::uniform::Uniform;
use crate::utils::init::INITIAL_WEBGPU;

async fn run() {
    let (
    surface_configuration,
        device,
        queue,
        surface
    )  = INITIAL_WEBGPU().await;

    let vertex_buffer = CubeVertex::vertex_buffer(
        &device,
        CubeVertex::VERTICES
    );

    let pipeline = CubeVertex::pipe_line(
        &device,
        &surface_configuration
    );

    let depth_texture = Uniform::depth_texture(
        &device,
        &surface_configuration,
        "Depth Texture"
    );

    let uniform_buffer = Uniform::buffer(
        &device,
        "Uniform Buffer"
    );

    let (bind_group_layout,bind_group) = Uniform::bind_group(
        &device,
        "Uniform Bind Group",
        &uniform_buffer
    );

   
}
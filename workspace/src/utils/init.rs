use gloo::utils::{document, document_element};
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use wgpu::{Device, Queue, Surface};

pub async fn INITIAL_WEBGPU_WITH_PROP_CANVAS<'window>(canvas: &web_sys::HtmlCanvasElement) -> (wgpu::SurfaceConfiguration, Device, Queue, Surface<'window>) {
    let dyn_canvas = canvas.clone().dyn_into::<HtmlCanvasElement>().unwrap();
    let width = dyn_canvas.width();
    let height = dyn_canvas.height();

    let instance = wgpu::Instance::new(
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..wgpu::InstanceDescriptor::default()
        }
    );

    let surface_target = wgpu::SurfaceTarget::Canvas(dyn_canvas);
    let surface = instance.create_surface(surface_target).unwrap();
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }
    ).await.unwrap();
    // let surface_caps = surface.get_capabilities(&adapter);

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_limits: wgpu::Limits::default(),
            required_features: wgpu::Features::empty(),
            label: None,
            ..wgpu::DeviceDescriptor::default()
        },
        None
    ).await.unwrap();

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    return (
        config,
        device,
        queue,
        surface
    )
}

pub async fn INITIAL_WEBGPU<'window>() ->
                                       (wgpu::SurfaceConfiguration, Device, Queue, Surface<'window>) {
    let canvas = document().create_element("canvas").unwrap();
    document_element().append_child(&canvas).unwrap();

    let dyn_canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    let width = dyn_canvas.width();
    let height = dyn_canvas.height();

    let instance = wgpu::Instance::new(
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..wgpu::InstanceDescriptor::default()
        }
    );

    let surface_target = wgpu::SurfaceTarget::Canvas(dyn_canvas);
    let surface = instance.create_surface(surface_target).unwrap();
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false
        }
    ).await.unwrap();
    // let surface_caps = surface.get_capabilities(&adapter);

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_limits: wgpu::Limits::default(),
            required_features: wgpu::Features::empty(),
            label: None,
            ..wgpu::DeviceDescriptor::default()
        },
        None
    ).await.unwrap();

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    return (
        config,
        device,
        queue,
        surface
    )
}

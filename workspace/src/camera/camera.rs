use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};
use wgpu::util::DeviceExt;

pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    fovy: f32,
    aspect: f32,
    znear: f32,
    zfar: f32,
}

#[repr(C)]
#[derive(Copy,Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32;4];4],
}

struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_back_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}


impl Camera {
    pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,0.5,0.5,
        0.0,0.0,0.0,1.0,
    );
    // 初期のカメラ生成
    pub fn new(width: u32,height: u32) -> Self {
        Self {
            eye: cgmath::Point3::new(0.0,1.0,2.0),
            target: cgmath::Point3::new(0.0,0.0,0.0),
            up: cgmath::Vector3::unit_y(),
            fovy: 45.0,
            aspect: width as f32 / height as f32,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye,self.target,self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy),self.aspect,self.znear,self.zfar);
        return Camera::OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self,camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }

    pub fn buffer(device: &wgpu::Device,label: &str,camera_uniform: &CameraUniform) -> wgpu::Buffer {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(&[camera_uniform.clone()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        )
    }
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_back_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn key_events(&'static mut self){
        use gloo::utils::window;
        use gloo::events::EventListener;

        let window = window();
        let _ = EventListener::new(
            window.deref(),
            "keydown",
             |event: &Event| {
                event.prevent_default();
                let keydown_event = event.dyn_ref::<KeyboardEvent>().unwrap();
                match keydown_event.key().as_str() {
                    "ArrowUp" => {
                        // Process for Arrow Up key
                        self.is_forward_pressed = true;
                    }
                    "ArrowDown" => {
                        // Process for Arrow Down key
                        self.is_back_pressed = true;
                    }
                    "ArrowLeft" => {
                        // Process for Arrow Left key
                        self.is_left_pressed = true;
                    }
                    "ArrowRight" => {
                        // Process for Arrow Right key
                        self.is_right_pressed = true;
                    }
                    _ => {}
                }
            }
        );
    }

    pub fn update_camera(&self,camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_back_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }

    }
}
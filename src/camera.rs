use std::ops::Mul;
//use wgpu::naga::SubgroupOperation::Mul;
use winit::keyboard::NativeKey::MacOS;
use crate::constants::OPENGL_TO_WGPU_MATRIX;
use crate::math::{perspective, Matrix4, Point3, Vector3};

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}


impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye.clone(), self.target.clone(), self.up.clone());
        let projection = perspective(self.fovy, self.aspect, self.znear, self.zfar);

       let output = view.clone() * projection.clone();
        let outpute = view * projection;

        output
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Matrix4::new_identity().into()
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

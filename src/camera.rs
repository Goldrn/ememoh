use std::ops::Mul;
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

        OPENGL_TO_WGPU_MATRIX * projection * view
    }
}

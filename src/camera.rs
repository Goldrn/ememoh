use wgpu::rwh::HandleError;
use winit::dpi::Position;
use crate::constants::OPENGL_TO_WGPU_MATRIX;
use crate::math::{perspective, Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub up: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
}


impl Camera {

    pub fn new<U: Into<Vector3<f32>>, V: Into<Point3<f32>>, Y: Into<f32>, P: Into<f32>> (up: U, position: V, yaw: Y, pitch: P) -> Self {
        Self {position: position.into(), up: up.into(), yaw: yaw.into(), pitch: pitch.into()}
    }
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let view = Matrix4::look_to_rh(self.position.clone(), Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(), self.up.clone());

        view
    }
}

pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<f32>> (width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {aspect: width as f32 / height as f32, fovy: fovy.into(), znear, zfar}
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        perspective(self.fovy, self.aspect, self.znear, self.zfar)
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

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_proj = (camera.build_view_projection_matrix() * projection.calc_matrix()).into();
    }
}

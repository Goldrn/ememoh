use crate::math::{Matrix4, Vector3};
use std::f32::consts::FRAC_2_PI;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub const NUM_INSTANCES_PER_ROW: u32 = 50;
pub const INSTANCE_DISPLACEMENT: Vector3<f32> = Vector3::new(NUM_INSTANCES_PER_ROW as f32 * 0.5, 0.0,NUM_INSTANCES_PER_ROW as f32 * 0.5);

pub const SAFE_FRAC_PI_2: f32 = FRAC_2_PI - 0.0001;
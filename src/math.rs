pub struct Point3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

pub struct Vector3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl Vector3<f32> {
    pub fn new(x: f32, y:f32, z: f32) -> Vector3<f32> {
        Vector3{x, y, z }
    }
    pub fn cross(&self, v: Vector3<f32>) -> Vector3<f32> {
        Vector3::new((self.y*v.z) - (self.z*v.y), (self.z*v.x) - (self.x*v.z), (self.x*v.y) - (self.y*v.x))
    }

    pub fn dot(&self, v: Vector3<f32>) -> f32 {
        (self.x*v.x) + (self.y*v.y) + (self.z*v.z)
    }
}
pub struct Vector4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}
pub struct Matrix4<S> {
    pub x: Vector4<S>,
    pub y: Vector4<S>,
    pub z: Vector4<S>,
    pub w: Vector4<S>,
}

impl Matrix4<f32> {
    pub fn look_at_rh(&self) -> Matrix4<f32> {

    }
}


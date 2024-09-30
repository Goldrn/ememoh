use std::arch::x86_64::_mm256_sqrt_ps;
use std::ops::Mul;

pub struct Point3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl Point3<f32> {
    pub fn new(x: f32, y: f32, z: f32) -> Point3<f32> {
        Point3 { x, y, z }
    }

    pub fn clone(&self) -> Point3<f32> {
        Point3{x:self.x, y: self.y, z: self.z}
    }

    pub fn dot_with_vector(&self, vector: Vector3<f32>) -> f32 {
        vector.x * self.x + vector.y * self.y + vector.z * self.z
    }
}

pub struct Vector3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl Vector3<f32> {
    pub fn new(x: f32, y:f32, z: f32) -> Vector3<f32> {
        Vector3{x, y, z}
    }
    pub fn cross(&self, v: Vector3<f32>) -> Vector3<f32> {
        Vector3::new((self.y*v.z) - (self.z*v.y), (self.z*v.x) - (self.x*v.z), (self.x*v.y) - (self.y*v.x))
    }

    pub fn dot(&self, v: Vector3<f32>) -> f32 {
        (self.x*v.x) + (self.y*v.y) + (self.z*v.z)
    }

    pub fn normalize(&self) -> Vector3<f32> {
        let mag : f32 = f32::sqrt((self.x*self.x) + (self.y*self.y) + (self.z*self.z));
        Vector3::new(self.x/mag, self.y/mag, self.z/mag)
    }

    pub fn clone(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}
pub struct Vector4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

impl Vector4<f32> {
    pub const fn new(x: f32, y: f32, z: f32, w:f32) -> Vector4<f32> {
        Vector4{x,y,z,w}
    }
    pub fn dot(&self, v: Vector4<f32>) -> f32 {
        (self.x*v.x) + (self.y*v.y) + (self.z*v.z) + (self.w*v.w)
    }
}
pub struct Matrix4<S> {
    pub x: Vector4<S>,
    pub y: Vector4<S>,
    pub z: Vector4<S>,
    pub w: Vector4<S>,
}

impl Matrix4<f32> {

    pub const fn new(x1: f32, x2: f32, x3: f32, x4: f32, y1: f32, y2: f32, y3: f32, y4: f32, z1: f32, z2: f32, z3: f32, z4: f32, w1: f32, w2: f32, w3: f32, w4: f32,) -> Matrix4<f32> {
        Matrix4{x: Vector4::new(x1, x2, x3, x4), y: Vector4::new(y1, y2, y3, y4), z: Vector4::new(z1, z2, z3, z4), w: Vector4::new(w1, w2, w3, w4)}
    }
    pub fn look_to_rh(eye: Point3<f32>, dir: Vector3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
        let dir_normal = dir.normalize();
        let perpenduicular = dir_normal.cross(up).normalize();
        //because up is not nessesarily perpendicular to dir
        let perp_up = perpenduicular.cross(dir_normal.clone());

        Matrix4::new(
            perpenduicular.x.clone(), perp_up.x.clone(), -dir_normal.x.clone(), 0.0,
            perpenduicular.y.clone(), perp_up.y.clone(), -dir_normal.y.clone(), 0.0,
            perpenduicular.z.clone(), perp_up.z.clone(), -dir_normal.z.clone(), 0.0,
            -eye.dot_with_vector(perpenduicular), -eye.dot_with_vector(perp_up), -eye.dot_with_vector(dir_normal), 1.0,
        )
    }

    pub fn look_at_rh(eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
        Matrix4::look_to_rh(eye.clone(), Vector3::new(center.x - eye.x, center.y - eye.y, center.z - eye.z), up)
    }
}
impl Mul for Matrix4<f32> {
    type Output = Matrix4<f32>;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4::new(
            self.x.dot(Vector4::new(rhs.x.x, rhs.y.x, rhs.z.x, rhs.w.x)), self.x.dot(Vector4::new(rhs.x.y, rhs.y.y, rhs.z.y, rhs.w.y)), self.x.dot(Vector4::new(rhs.x.z, rhs.y.z, rhs.z.z, rhs.w.z)), self.x.dot(Vector4::new(rhs.x.w, rhs.y.w, rhs.z.w, rhs.w.w)),
            self.y.dot(Vector4::new(rhs.x.x, rhs.y.x, rhs.z.x, rhs.w.x)), self.y.dot(Vector4::new(rhs.x.y, rhs.y.y, rhs.z.y, rhs.w.y)), self.y.dot(Vector4::new(rhs.x.z, rhs.y.z, rhs.z.z, rhs.w.z)), self.y.dot(Vector4::new(rhs.x.w, rhs.y.w, rhs.z.w, rhs.w.w)),
            self.z.dot(Vector4::new(rhs.x.x, rhs.y.x, rhs.z.x, rhs.w.x)), self.z.dot(Vector4::new(rhs.x.y, rhs.y.y, rhs.z.y, rhs.w.y)), self.z.dot(Vector4::new(rhs.x.z, rhs.y.z, rhs.z.z, rhs.w.z)), self.z.dot(Vector4::new(rhs.x.w, rhs.y.w, rhs.z.w, rhs.w.w)),
            self.w.dot(Vector4::new(rhs.x.x, rhs.y.x, rhs.z.x, rhs.w.x)), self.w.dot(Vector4::new(rhs.x.y, rhs.y.y, rhs.z.y, rhs.w.y)), self.w.dot(Vector4::new(rhs.x.z, rhs.y.z, rhs.z.z, rhs.w.z)), self.w.dot(Vector4::new(rhs.x.w, rhs.y.w, rhs.z.w, rhs.w.w)),
        )
    }
}

pub struct PerspectiveFov<S> {
    pub fovy: S,
    pub aspect: S,
    pub near: S,
    pub far: S,
}

impl From<PerspectiveFov<f32>> for Matrix4<f32> {
    fn from(persp: PerspectiveFov<f32>) -> Matrix4<f32> {
        let f = (persp.fovy / 2.0).to_radians().tan().recip();

        let c1r1 = f / persp.aspect;
        let c1r2 = 0.0;
        let c1r3 = 0.0;
        let c1r4 = 0.0;

        let c2r1 = 0.0;
        let c2r2 = f;
        let c2r3 = 0.0;
        let c2r4 = 0.0;

        let c3r1 = 0.0;
        let c3r2 = 0.0;
        let c3r3 = (persp.far + persp.near)/(persp.near - persp.far);
        let c3r4= -1.0;

        let c4r1 = 0.0;
        let c4r2 = 0.0;
        let c4r3 = (2.0 * persp.far * persp.near) / (persp.near - persp.far);
        let c4r4= 0.0;

        Matrix4::new(
            c1r1, c1r2, c1r3, c1r4,
            c2r1, c2r3, c2r4, c2r1,
            c3r1, c3r2, c3r3, c3r4,
            c4r1, c4r2, c4r3, c4r4,
        )
    }
}
pub fn perspective<S> (fovy: S, aspect: S, near: S, far: S) -> Matrix4<S> where Matrix4<S>: From<PerspectiveFov<S>> {
    PerspectiveFov {
        fovy,
        aspect,
        near,
        far,
    }.into()
}

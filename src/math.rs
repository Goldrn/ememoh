use std::ops::{Add, AddAssign, Mul, Sub};

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
    pub fn cross_with_vector(&self, v: Vector3<f32>) -> Vector3<f32> {
        Vector3::new((self.y*v.z) - (self.z*v.y), (self.z*v.x) - (self.x*v.z), (self.x*v.y) - (self.y*v.x))
    }
    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x*self.x) + (self.y*self.y) + (self.z*self.z))
    }

    pub fn normalize(&self) -> Point3<f32>{
        let mag : f32 = f32::sqrt((self.x*self.x) + (self.y*self.y) + (self.z*self.z));
        Point3::new(self.x/mag, self.y/mag, self.z/mag)
    }
}
impl AddAssign<Vector3<f32>> for Point3<f32> {
    fn add_assign(&mut self, rhs: Vector3<f32>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Add<Vector3<f32>> for Point3<f32> {
    type Output = Point3<f32>;

    fn add(self, rhs: Vector3<f32>) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point3<f32> {
    type Output = Point3<f32>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector3<f32>> for Point3<f32> {
    type Output = Point3<f32>;

    fn sub(self, rhs: Vector3<f32>) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for Point3<f32> {
    type Output = Point3<f32>;

    fn mul(self, rhs: Self) -> Self::Output {
        Point3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Point3<f32> {
    type Output = Point3<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

pub struct Vector3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

impl Vector3<f32> {
    pub const fn new(x: f32, y:f32, z: f32) -> Vector3<f32> {
        Vector3{x, y, z}
    }
    pub fn cross(&self, v: Vector3<f32>) -> Vector3<f32> {
        Vector3::new((self.y*v.z) - (self.z*v.y), (self.z*v.x) - (self.x*v.z), (self.x*v.y) - (self.y*v.x))
    }

    pub fn dot(&self, v: Vector3<f32>) -> f32 {
        (self.x*v.x) + (self.y*v.y) + (self.z*v.z)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x*self.x) + (self.y*self.y) + (self.z*self.z))
    }
    pub fn normalize(&self) -> Vector3<f32> {
        let mag : f32 = f32::sqrt((self.x*self.x) + (self.y*self.y) + (self.z*self.z));
        Vector3::new(self.x/mag, self.y/mag, self.z/mag)
    }

    pub fn clone(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Mul<f32> for Vector3<f32> {
    type Output = Vector3<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Add for Vector3<f32> {
    type Output = Vector3<f32>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Point3<f32>> for Vector3<f32> {
    type Output = Vector3<f32>;

    fn add(self, rhs: Point3<f32>) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point3<f32>> for Vector3<f32> {
    type Output = Vector3<f32>;

    fn sub(self, rhs: Point3<f32>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector3<f32>> for Vector3<f32> {
    type Output = Vector3<f32>;

    fn sub(self, rhs: Vector3<f32>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl From<[f32; 3]> for Vector3<f32> {
    fn from(value: [f32; 3]) -> Self {
        Vector3::new(value[0], value[1], value[2])
    }
}

impl Into<[f32; 3]> for Vector3<f32> {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
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

    pub fn clone(&self) -> Vector4<f32>{Vector4::new(self.x, self.y, self.z, self.w)}
}

pub struct Matrix3<S> {
    pub x: Vector3<S>,
    pub y: Vector3<S>,
    pub z: Vector3<S>,
}

impl Matrix3<f32> {
    pub const fn new(x1: f32, x2: f32, x3: f32, y1: f32, y2: f32, y3: f32, z1: f32, z2: f32, z3: f32) -> Matrix3<f32> {
        Matrix3{x: Vector3::new(x1, x2, x3), y: Vector3::new(y1, y2, y3), z: Vector3::new(z1, z2, z3)}
    }
}

impl From<Quaternion<f32>> for Matrix3<f32>{
    fn from(quat: Quaternion<f32>) -> Matrix3<f32> {
        let x2 = quat.vector.x + quat.vector.x;
        let y2 = quat.vector.y + quat.vector.y;
        let z2 = quat.vector.z + quat.vector.z;

        let xx2 = x2 * quat.vector.x;
        let xy2 = x2 * quat.vector.y;
        let xz2 = x2 * quat.vector.z;

        let yy2 = y2 * quat.vector.y;
        let yz2 = y2 * quat.vector.z;
        let zz2 = z2 * quat.vector.z;

        let sy2 = y2 * quat.scalar;
        let sz2 = z2 * quat.scalar;
        let sx2 = x2 * quat.scalar;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            1.0 - yy2 - zz2, xy2 + sz2, xz2 - sy2,
            xy2 - sz2, 1.0 - xx2 - zz2, yz2 + sx2,
            xz2 + sy2, yz2 - sx2, 1.0 - xx2 - yy2,
        )
    }
}
impl From<Matrix3<f32>> for [[f32; 3]; 3] {
    fn from(value: Matrix3<f32>) -> [[f32; 3]; 3] {
        let mut output : [[f32; 3]; 3] = [[0.0; 3]; 3];

        output[0][0] = value.x.x;
        output[0][1] = value.x.y;
        output[0][2] = value.x.z;

        output[1][0] = value.y.x;
        output[1][1] = value.y.y;
        output[1][2] = value.y.z;

        output[2][0] = value.z.x;
        output[2][1] = value.z.y;
        output[2][2] = value.z.z;


        output
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
    pub const fn new_colum(x1: f32, x2: f32, x3: f32, x4: f32, y1: f32, y2: f32, y3: f32, y4: f32, z1: f32, z2: f32, z3: f32, z4: f32, w1: f32, w2: f32, w3: f32, w4: f32,) -> Matrix4<f32> {
        Matrix4{x: Vector4::new(x1, y1, z1, w1), y: Vector4::new(x2, y2, z2, w2), z: Vector4::new(x3, y3, z3, w3), w: Vector4::new(x4, y4, z4, w4)}
    }

    pub const fn new_from_vectors(x: Vector4<f32>, y: Vector4<f32>, z: Vector4<f32>, w: Vector4<f32>) -> Matrix4<f32>{
        Matrix4{x, y, z, w}
    }
    pub const fn new_identity() -> Matrix4<f32> {
        let x = Vector4::new(1.0,0.0,0.0,0.0);
        let y = Vector4::new(0.0,1.0,0.0,0.0);
        let z = Vector4::new(0.0,0.0,1.0,0.0);
        let w = Vector4::new(0.0,0.0,0.0,1.0);

        Matrix4::new_from_vectors(x, y, z, w)
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
            -eye.dot_with_vector(perpenduicular), -eye.dot_with_vector(perp_up), eye.dot_with_vector(dir_normal), 1.0,
        )
    }

    pub fn look_at_rh(eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) -> Matrix4<f32> {
        Matrix4::look_to_rh(eye.clone(), Vector3::new(center.x - eye.x, center.y - eye.y, center.z - eye.z), up)
    }

    pub fn clone(&self) -> Matrix4<f32> {
        Matrix4::new(self.x.x.clone(), self.x.y.clone(), self.x.z.clone(), self.x.w.clone(),
                     self.y.x.clone(), self.y.y.clone(), self.y.z.clone(), self.y.w.clone(),
                     self.z.x.clone(), self.z.y.clone(), self.z.z.clone(), self.z.w.clone(),
                     self.w.x.clone(), self.w.y.clone(), self.w.z.clone(), self.w.w.clone(),)
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
impl From<[[f32; 4]; 4]> for Matrix4<f32> {
    fn from(value: [[f32; 4]; 4]) -> Matrix4<f32> {
        let mut output = Matrix4::new_identity();
        output.x.x = value[0][0];
        output.x.y = value[0][1];
        output.x.z = value[0][2];
        output.x.w = value[0][3];

        output.y.x = value[1][0];
        output.y.y = value[1][1];
        output.y.z = value[1][2];
        output.y.w = value[1][3];

        output.z.x = value[2][0];
        output.z.y = value[2][1];
        output.z.z = value[2][2];
        output.z.w = value[2][3];

        output.w.x = value[3][0];
        output.w.y = value[3][1];
        output.w.z = value[3][2];
        output.w.w = value[3][3];

        output
    }
}

impl From<Matrix4<f32>> for [[f32; 4]; 4] {
    fn from(value: Matrix4<f32>) -> [[f32; 4]; 4] {
        let mut output : [[f32; 4]; 4] = [[0.0; 4]; 4];

        output[0][0] = value.x.x;
        output[0][1] = value.x.y;
        output[0][2] = value.x.z;
        output[0][3] = value.x.w;

        output[1][0] = value.y.x;
        output[1][1] = value.y.y;
        output[1][2] = value.y.z;
        output[1][3] = value.y.w;

        output[2][0] = value.z.x;
        output[2][1] = value.z.y;
        output[2][2] = value.z.z;
        output[2][3] = value.z.w;

        output[3][0] = value.w.x;
        output[3][1] = value.w.y;
        output[3][2] = value.w.z;
        output[3][3] = value.w.w;

        output
    }
}

impl Mul<Vector3<f32>> for Matrix4<f32> {
    type Output = Vector3<f32>;

    fn mul(self, rhs: Vector3<f32>) -> Self::Output {
        let hack = Vector4::new(rhs.x, rhs.y, rhs.z, 0.0);
        Vector3::new(self.x.dot(hack.clone()), self.y.dot(hack.clone()), self.z.dot(hack.clone()))
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
            c2r1, c2r2, c2r3, c2r4,
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

pub struct Quaternion<S> {
    pub vector: Vector3<S>,
    pub scalar: S,
}

impl Quaternion<f32> {
    pub fn new(vector: Vector3<f32>, scalar: f32) -> Quaternion<f32>{
        Quaternion {vector, scalar}
    }

    pub fn clone(&self) -> Quaternion<f32> {
        Quaternion::new(self.vector.clone(), self.scalar.clone())
    }

    pub fn from_axis_angle(axis: Vector3<f32>, angle: f32) -> Quaternion<f32> {
        let (s,c) = (angle * 0.5).to_radians().sin_cos();
        Quaternion::new(axis * s, c)
    }
}
impl From<Quaternion<f32>> for Matrix4<f32>{
    fn from(quat: Quaternion<f32>) -> Matrix4<f32> {
        let x2 = quat.vector.x + quat.vector.x;
        let y2 = quat.vector.y + quat.vector.y;
        let z2 = quat.vector.z + quat.vector.z;

        let xx2 = x2 * quat.vector.x;
        let xy2 = x2 * quat.vector.y;
        let xz2 = x2 * quat.vector.z;

        let yy2 = y2 * quat.vector.y;
        let yz2 = y2 * quat.vector.z;
        let zz2 = z2 * quat.vector.z;

        let sy2 = y2 * quat.scalar;
        let sz2 = z2 * quat.scalar;
        let sx2 = x2 * quat.scalar;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            1.0 - yy2 - zz2, xy2 + sz2, xz2 - sy2, 0.0,
            xy2 - sz2, 1.0 - xx2 - zz2, yz2 + sx2, 0.0,
            xz2 + sy2, yz2 - sx2, 1.0 - xx2 - yy2, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

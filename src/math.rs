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


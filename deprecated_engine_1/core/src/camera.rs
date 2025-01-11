use cgmath::{self, Matrix4, Point3, Vector3, vec3, Deg};

pub enum Camera {
    Perspective(PerspectiveCamera),
    Orthographic(OrthographicCamera),
}

impl Camera {
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        match self {
            Camera::Perspective(cam) => cam.get_view_matrix(),
            Camera::Orthographic(cam) => cam.get_view_matrix(),
        }
    }
}

pub struct PerspectiveCamera {
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub position: Point3<f32>,
    pub orientation: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl PerspectiveCamera {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32, position: Point3<f32>) -> Self {
        Self {
            fov,
            aspect_ratio,
            near,
            far,
            position,
            orientation: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.position, self.position + self.orientation, self.up);
        let proj = cgmath::perspective(Deg(self.fov), self.aspect_ratio, self.near, self.far);
        proj * view
    }
}

pub struct OrthographicCamera {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub position: Point3<f32>,
    pub orientation: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl OrthographicCamera {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32, position: Point3<f32>) -> Self {
        Self {
            left,
            right,
            bottom,
            top,
            near,
            far,
            position,
            orientation: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.position, self.position + self.orientation, self.up);
        let proj = cgmath::ortho(self.left, self.right, self.bottom, self.top, self.near, self.far);
        proj * view
    }
}

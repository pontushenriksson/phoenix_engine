use cgmath::*;

pub trait Camera {
  fn view(&self) -> cgmath::Matrix4<f32>;
  fn projection(&self) -> cgmath::Matrix4<f32>;
  fn position(&self) -> cgmath::Point3<f32>;
}

pub struct PerspectiveCamera {
  pub position: cgmath::Point3<f32>,
  pub target: cgmath::Point3<f32>,
  pub up: cgmath::Vector3<f32>,
  pub fov: f32,
  pub aspect_ratio: f32,
  pub near_plane: f32,
  pub far_plane: f32,
  pub view_matrix: cgmath::Matrix4<f32>,
  pub projection_matrix: cgmath::Matrix4<f32>
}

impl PerspectiveCamera {
  pub fn new(
    position: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect_ratio: f32,
    fov: f32,
    near_plane: f32,
    far_plane: f32,
  ) -> PerspectiveCamera {
    PerspectiveCamera {
      position,
      target,
      up,
      aspect_ratio,
      fov,
      near_plane,
      far_plane,
      view_matrix: cgmath::Matrix4::look_at_rh(position, target, up),
      projection_matrix: cgmath::perspective(Rad(fov.to_radians()), aspect_ratio, near_plane, far_plane),
    }
  }

  pub fn update_view_matrix(&mut self) {
    self.view_matrix = cgmath::Matrix4::look_at_rh(
      self.position,
      self.target,
      self.up
    )
  }

  pub fn update_projection_matrix(&mut self) {
    self.projection_matrix = cgmath::perspective(
      Deg(self.fov), 
      self.aspect_ratio,
      self.near_plane, 
      self.far_plane
    )
  }
}

impl Camera for PerspectiveCamera {
  fn view(&self) -> cgmath::Matrix4<f32> {
    self.view_matrix
  }

  fn projection(&self) -> cgmath::Matrix4<f32> {
    self.projection_matrix
  }

  fn position(&self) -> cgmath::Point3<f32> {
    self.position
  }
}

pub struct OrthographicCamera {
  position: cgmath::Point3<f32>,
  target: cgmath::Vector3<f32>,
  up: cgmath::Vector3<f32>,
  left: f32,
  right: f32,
  bottom: f32,
  top: f32,
  near_plane: f32,
  far_plane: f32,
}

impl OrthographicCamera {
  pub fn new(
    position: cgmath::Point3<f32>,
    target: cgmath::Vector3<f32>,
    up: cgmath::Vector3<f32>,
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near_plane: f32,
    far_plane: f32,
  ) -> OrthographicCamera {
    OrthographicCamera {
      position,
      target,
      up,
      left,
      right,
      bottom,
      top,
      near_plane,
      far_plane,
    }
  }
}

impl Camera for OrthographicCamera {
  fn view(&self) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::look_to_rh(
      /* eye */ self.position,
      /* dir */ self.target,
      /* up  */ self.up
    )
  }

  fn projection(&self) -> cgmath::Matrix4<f32> {
    cgmath::ortho(
      self.left,
      self.right,
      self.bottom,
      self.top,
      self.near_plane,
      self.far_plane
    )
  }

  fn position(&self) -> cgmath::Point3<f32> {
    self.position
  }
}

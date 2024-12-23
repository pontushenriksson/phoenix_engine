use cgmath::*;

use super::shader::ShaderProgram;

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
  pub projection_matrix: cgmath::Matrix4<f32>,
  pub first_click: bool,
  pub sensitivity: f32, // For mouse input
  pub speed: f32,       // For movement
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
    let view_matrix = cgmath::Matrix4::look_at_rh(position, target, up);
    let projection_matrix = cgmath::perspective(
      cgmath::Deg(fov),
      aspect_ratio,
      near_plane,
      far_plane,
    );

    PerspectiveCamera {
      position,
      target,
      up,
      fov,
      aspect_ratio,
      near_plane,
      far_plane,
      view_matrix,
      projection_matrix,
      first_click: true,
      sensitivity: 100.0,
      speed: 0.01,
    }
  }

  pub fn update_view_matrix(&mut self) {
    self.view_matrix = cgmath::Matrix4::look_at_rh(self.position, self.target, self.up);
  }

  pub fn update_projection_matrix(&mut self) {
    self.projection_matrix = cgmath::perspective(
      cgmath::Deg(self.fov),
      self.aspect_ratio,
      self.near_plane,
      self.far_plane,
    );
  }

  pub fn update_camera_matrix(&mut self) {
    self.update_view_matrix();
    self.update_projection_matrix();
  }

  pub fn matrix(&self, shader_program: &mut ShaderProgram, uniform_name: &str /* String later maybe*/) {
    shader_program.create_uniform(uniform_name);
    let matrix = self.projection_matrix * self.view_matrix;
    shader_program.set_uniform_matrix_4_f32_vec(uniform_name, &matrix);
  }

  pub fn inputs(&mut self, window: &mut glfw::Window) {
    // Keyboard input
    let direction = (self.target - self.position).normalize();
    let right = direction.cross(self.up).normalize();

    if window.get_key(glfw::Key::W) == glfw::Action::Press {
      self.position += self.speed * direction;
      self.target += self.speed * direction;
    }
    if window.get_key(glfw::Key::A) == glfw::Action::Press {
      self.position -= self.speed * right;
      self.target -= self.speed * right;
    }
    if window.get_key(glfw::Key::S) == glfw::Action::Press {
      self.position -= self.speed * direction;
      self.target -= self.speed * direction;
    }
    if window.get_key(glfw::Key::D) == glfw::Action::Press {
      self.position += self.speed * right;
      self.target += self.speed * right;
    }
    if window.get_key(glfw::Key::Space) == glfw::Action::Press {
      self.position += self.speed * self.up;
      self.target += self.speed * self.up;
    }
    if window.get_key(glfw::Key::LeftControl) == glfw::Action::Press {
      self.position -= self.speed * self.up;
      self.target -= self.speed * self.up;
    }
    if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
      self.speed = 0.04;
    }
    if window.get_key(glfw::Key::LeftShift) == glfw::Action::Release {
      self.speed = 0.01;
    }
    if window.get_key(glfw::Key::Backspace) == glfw::Action::Press {
      self.position = cgmath::point3(0.0, 0.0, 2.0);
      self.target = cgmath::point3(0.0, 0.0, -1.0);
    }

    // Mouse input
    if window.get_mouse_button(glfw::MouseButton::Button1) == glfw::Action::Press {
      window.set_cursor_mode(glfw::CursorMode::Hidden);

      if self.first_click {
        window.set_cursor_pos((self.aspect_ratio * 100.0) as f64, 100.0);
        self.first_click = false;
      }

      let (mouse_x, mouse_y): (f64, f64) = window.get_cursor_pos();

      let rot_x = self.sensitivity * ((mouse_y - (self.aspect_ratio * 100.0) as f64) as f32) / 100.0;
      let rot_y = self.sensitivity * ((mouse_x - 100.0) as f32) / 100.0;

      // Vertical rotation (pitch)
      let pitch_axis = direction.cross(self.up).normalize();
      let pitch_quat = cgmath::Quaternion::from_axis_angle(pitch_axis, cgmath::Deg(-rot_x));
      let new_direction = pitch_quat * direction;

      // Clamp pitch to avoid flipping
      let up_dot = new_direction.dot(self.up);
      if up_dot.abs() < 0.99 {
        self.target = self.position + new_direction;
      }

      // Horizontal rotation (yaw)
      let yaw_quat = cgmath::Quaternion::from_axis_angle(self.up, cgmath::Deg(-rot_y));
      self.target = self.position + yaw_quat * new_direction;

      // Reset cursor to the center
      window.set_cursor_pos((self.aspect_ratio * 100.0) as f64, 100.0);
    }

    if window.get_mouse_button(glfw::MouseButton::Button1) == glfw::Action::Release {
      window.set_cursor_mode(glfw::CursorMode::Normal);
      self.first_click = true;
    }

    // Update the view matrix
    self.view_matrix = cgmath::Matrix4::look_at_rh(self.position, self.target, self.up);
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

pub struct Camera3D { // Implement Perspective and Orthographic separetelly later
  pub matrix: cgmath::Matrix4<f32>,
  pub position: cgmath::Point3<f32>,
  pub orientation: cgmath::Vector3<f32>,
  pub up: cgmath::Vector3<f32>,
  pub window_width: i32,
  pub window_height: i32,
  pub speed: f32, // 0.1
  pub sensitivity: f32, // 100.0
  pub first_click: bool,
}

use std::sync::{Arc, Mutex};

use cgmath::{InnerSpace, Rotation3, SquareMatrix};

pub struct Camera {
  pub view: cgmath::Matrix4<f32>,
  pub projection: cgmath::Matrix4<f32>,
  position: cgmath::Point3<f32>,
  orientation: cgmath::Vector3<f32>,
  up: cgmath::Vector3<f32>,
  width: u32,
  height: u32,
  aspect_ratio: f32,
  fov: f32,
  near: f32,
  far: f32,
  /* Move this later */
  speed: f32,
  sensitivity: f32,
  first_click: bool,
}

impl Camera {
  pub fn new(
    window_width: u32,
    window_height: u32,
    aspect_ratio: f32,
    position: cgmath::Point3<f32>,
    fov: f32,
    near: f32,
    far: f32,
    speed: f32,
    sensitivity: f32
  ) -> Camera {
    Camera {
      view: cgmath::Matrix4::identity(),
      projection: cgmath::Matrix4::identity(),
      position,
      orientation: cgmath::vec3(0.0, 0.0, -1.0),
      up: cgmath::vec3(0.0, 1.0, 0.0),
      width: window_width,
      height: window_height,
      aspect_ratio,
      fov,
      near,
      far,
      speed,
      sensitivity,
      first_click: false,
    }
  }

  pub fn update_matrix(&mut self) {
    let view = cgmath::Matrix4::look_at_rh(self.position, self.position + self.orientation, self.up);
    self.view = view;
    let proj = cgmath::perspective(cgmath::Deg(self.fov), self.aspect_ratio, self.near, self.far);
    self.projection = proj;
  }

  pub fn inputs(&mut self, window: &Arc<Mutex<glfw::PWindow>>, delta_time: f64) {
  // Keyboard input
    if window.lock().unwrap().get_key(glfw::Key::W) == glfw::Action::Press {
      self.position += self.speed * self.orientation * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::A) == glfw::Action::Press {
      self.position += self.speed * -cgmath::Vector3::normalize(cgmath::Vector3::cross(self.orientation, self.up)) * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::S) == glfw::Action::Press {
      self.position += self.speed * -self.orientation * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::D) == glfw::Action::Press {
      self.position += self.speed * cgmath::Vector3::normalize(cgmath::Vector3::cross(self.orientation, self.up)) * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::Space) == glfw::Action::Press {
      self.position += self.speed * self.up * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::LeftControl) == glfw::Action::Press {
      self.position += self.speed * -self.up * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::LeftShift) == glfw::Action::Press {
      self.speed = 100.0 * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::LeftShift) == glfw::Action::Release {
      self.speed = 50.0 * delta_time as f32;
    }
    if window.lock().unwrap().get_key(glfw::Key::Backspace) == glfw::Action::Press {
      self.position = cgmath::point3(0.0, 0.0, 2.0);
      self.orientation = cgmath::vec3(0.0, 0.0, -1.0);
    }

    // Mouse input

    if window.lock().unwrap().get_mouse_button(glfw::MouseButton::Button1) == glfw::Action::Press {
      window.lock().unwrap().set_cursor_mode(glfw::CursorMode::Hidden);

      if self.first_click {
        window.lock().unwrap().set_cursor_pos((self.width / 2) as f64, (self.height / 2) as f64);
        self.first_click = false;
      }

      let (mouse_x, mouse_y): (f64, f64) = window.lock().unwrap().get_cursor_pos();

      let rot_x: f32 = self.sensitivity * ((mouse_y - (self.height / 2) as f64) as f32) / self.height as f32;
      let rot_y: f32 = self.sensitivity * ((mouse_x - (self.width / 2) as f64) as f32) / self.width as f32;

      // Calculate new orientation for vertical rotation (pitch)
      let right: cgmath::Vector3<f32> = self.orientation.cross(self.up).normalize();
      let pitch_quat = cgmath::Quaternion::from_axis_angle(right, cgmath::Deg(-rot_x));

      let new_orientation = pitch_quat * self.orientation;

      // Make sure the new orientation doesn't exceed the vertical limit
      let up_dot = new_orientation.dot(self.up);
      if up_dot.abs() < 0.99 {
        self.orientation = new_orientation;
      }

      // Apply horizontal rotation (yaw)
      let yaw_quat = cgmath::Quaternion::from_axis_angle(self.up, cgmath::Deg(-rot_y));
      self.orientation = yaw_quat * self.orientation;

      // Reset mouse position to the center of the screen
      window.lock().unwrap().set_cursor_pos((self.width / 2) as f64, (self.height / 2) as f64);
    }

    if window.lock().unwrap().get_mouse_button(glfw::MouseButton::Button1) == glfw::Action::Release {
      window.lock().unwrap().set_cursor_mode(glfw::CursorMode::Normal);
      self.first_click = true;
    }
  }
}

use cgmath::SquareMatrix;

use crate::graphics::camera::Camera;
use crate::graphics::data::{Attribute, VertexDescriptor};
use crate::graphics::material::Material;
use crate::graphics::mesh::{BufferType, Mesh};
use crate::graphics::texture::{Topography, Sampler2D};

use super::transform::Transform;

pub struct Quad {
  pub mesh: Mesh<gl::types::GLfloat, gl::types::GLuint>,
  pub matrix: cgmath::Matrix4<f32>,
}

impl Quad {
  pub fn new() -> Quad {
    let vertices: [gl::types::GLfloat; 32] = [
    // Positions              Color                   Texture Coords
      -0.5, -0.5,  0.0,       1.0, 1.0, 1.0,          0.0, 0.0,
       0.5, -0.5,  0.0,       1.0, 0.0, 0.0,          1.0, 0.0,
       0.5,  0.5,  0.0,       0.0, 1.0, 0.0,          1.0, 1.0,
      -0.5,  0.5,  0.0,       0.0, 0.0, 1.0,          0.0, 1.0,
    ];

    let indices: [gl::types::GLuint; 6] = [
      0, 1, 2,
      2, 3, 0
    ];

    let descriptor = VertexDescriptor {
      attributes: vec![
        Attribute::Vec3,
        Attribute::Vec3,
        Attribute::Vec2,
      ],
      stride: 8
    };

    let mesh = Mesh::new(
      BufferType::Static,
      vertices.to_vec(),
      Some(indices.to_vec()),
      descriptor,
    );

    Quad {
      mesh,
      matrix: cgmath::Matrix4::identity(),
    }
  }

  pub fn indices() -> u32 {
    6
  }
}

pub struct Plane {
  pub mesh: Mesh<gl::types::GLfloat, gl::types::GLuint>,
  pub matrix: cgmath::Matrix4<f32>,
}

impl Plane {
  /// Create a plane with `x` subdivisions along width and `y` subdivisions along height.
  pub fn new(x: u32, y: u32) -> Plane {
      let mut vertices: Vec<f32> = Vec::new();
      let mut indices: Vec<u32> = Vec::new();

      let width = x as f32;
      let height = y as f32;

      // Generate vertices
      for i in 0..=x {
          for j in 0..=y {
              let px = (i as f32 / width) - 0.5;  // Scale to range [-0.5, 0.5]
              let py = (j as f32 / height) - 0.5;

              // Position (x, y, z)
              vertices.push(px);
              vertices.push(0.0); // Flat plane (Y = 0)
              vertices.push(py);

              // Color (RGB)
              vertices.push(1.0);
              vertices.push(1.0);
              vertices.push(1.0);

              // Texture Coords (u, v)
              vertices.push(i as f32 / width);
              vertices.push(j as f32 / height);
          }
      }

      // Generate indices
      for i in 0..x {
          for j in 0..y {
              let top_left = i * (y + 1) + j;
              let top_right = (i + 1) * (y + 1) + j;
              let bottom_left = i * (y + 1) + (j + 1);
              let bottom_right = (i + 1) * (y + 1) + (j + 1);

              // First triangle
              indices.push(top_left);
              indices.push(bottom_left);
              indices.push(top_right);

              // Second triangle
              indices.push(top_right);
              indices.push(bottom_left);
              indices.push(bottom_right);
          }
      }

      let descriptor = VertexDescriptor {
          attributes: vec![
              Attribute::Vec3, // Position
              Attribute::Vec3, // Color
              Attribute::Vec2, // Texture Coordinates
          ],
          stride: 8,
      };

      let mesh = Mesh::new(BufferType::Static, vertices, Some(indices), descriptor);

      Plane {
          mesh,
          matrix: cgmath::Matrix4::identity(),
      }
  }
}

pub struct Ground {
  plane: Plane,
  material: Material,
  pub transform: Transform,
}

impl Ground {
  pub fn new(
    x: u32,
    y: u32,
    material: Material
  ) -> Ground {
    let plane = Plane::new(x, y);
    
    Ground {
      plane,
      material,
      transform: Transform::identity()
    }
  }

  pub fn with_transform(mut self, transform: Transform) -> Ground {
    self.transform = transform;

    let translation = cgmath::Matrix4::from_translation(self.transform.translation);
    let rotation = cgmath::Matrix4::from(self.transform.rotation);
    let scale = cgmath::Matrix4::from_nonuniform_scale(
      self.transform.scale.x,
      self.transform.scale.y,
      self.transform.scale.z,
    );

    self.plane.matrix = translation * rotation * scale; // Apply transformations in the correct order

    self
  }

  pub fn update_matrix(&mut self) {
    let translation = cgmath::Matrix4::from_translation(self.transform.translation);
    let rotation = cgmath::Matrix4::from(self.transform.rotation);
    let scale = cgmath::Matrix4::from_nonuniform_scale(
      self.transform.scale.x,
      self.transform.scale.y,
      self.transform.scale.z,
    );

    self.plane.matrix = translation * rotation * scale;
  }

  pub fn draw(&self, camera: &Camera) {
    self.material.bind();
    self.material.shader.set_matrix4_f32("uModel", &self.plane.matrix);
    self.material.shader.set_matrix4_f32("uView", &camera.view);
    self.material.shader.set_matrix4_f32("uProjection", &camera.projection);
    self.plane.mesh.draw();
  }
}

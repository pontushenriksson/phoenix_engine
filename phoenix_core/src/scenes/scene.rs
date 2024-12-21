use crate::assets::loader::RawVertexData;
use crate::ecs::manager::*;
use crate::graphics::mesh::*;

pub struct Transform {
  pub position: cgmath::Vector3<f32>,
  pub rotation: cgmath::Quaternion<f32>,
  pub scale: cgmath::Vector3<f32>,
}

impl Transform {
  pub fn new() -> Transform {
    Transform {
      position: cgmath::Vector3::new(0.0, 0.0, 0.0),
      rotation: cgmath::Quaternion::new(1.0, 1.0, 1.0, 1.0), // Identity
      scale: cgmath::Vector3::new(1.0, 1.0, 1.0)
    }
  }

  pub fn matrix(&self) -> cgmath::Matrix4<f32> {
    // Translation
    let translation = cgmath::Matrix4::from_translation(self.position);
    // Rotation
    let rotation = cgmath::Matrix4::from(self.rotation);
    // Scale
    let scale = cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

    // Transformation
    translation * rotation * scale
  }
}

pub struct StaticGameObject {
  mesh: Mesh,
  pub transform: Transform,
  pub material: Option<Material>
}

impl StaticGameObject {
  pub fn set_transform(&mut self, transform: Transform) {
    self.transform = transform;
  }

  pub fn new(
    vertices: RawVertexData,
    indices: Option<Vec<u32>>,
    material: Option<Material>
  ) -> StaticGameObject {
    let mesh = Mesh::new(vertices, indices);

    let mut mat = material.unwrap();

    mat.shader.bind();
    mat.shader.create_uniform("tex0");
    mat.shader.set_uniform_i32("tex0", 0);

    mat.shader.create_uniform("model");
    mat.shader.create_uniform("view");
    mat.shader.create_uniform("projection");

    StaticGameObject {
      mesh,
      transform: Transform::new(),
      material: Some(mat)
    }
  }

  pub fn mesh(&self) -> &Mesh {
    &self.mesh
  }

  pub fn material(&self) -> &Option<Material> {
    &self.material
  }
}

pub struct Scene {
  // ecs_manager: EcsManager,
  // Used for demo before ECS is implemented
  pub static_game_objects: Vec<StaticGameObject>,
}

impl Scene {
  pub fn new() -> Scene {
    Scene {
      static_game_objects: Vec::new()
    }
  }
}

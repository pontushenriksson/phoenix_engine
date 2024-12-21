use crate::debugger::debugger::Debugger;
use crate::gl_call;
use crate::graphics::camera::*;
use crate::graphics::mesh::*;
use crate::scenes::scene::*;
/// Renderer
pub struct Renderer;

impl Renderer {
  /// Render function
  pub fn render<T: Camera>(camera: &T, scene: &Scene) {
    // TODO:
  }
}

/*

  Old rendering:

  let view_matrix = camera.view();
  let projection_matrix = camera.projection();

  // Query<(Entity, Component)>

  for object in scene.static_game_objects.iter() {
    let mesh = object.mesh();
    let material = object.material().unwrap();
    let transform = object.transform;

    // Bind shader
    material.shader.bind();

    // Set the model, view, projection uniforms
    material.shader.set_uniform_matrix_4_f32_vec("model", &transform.matrix());
    material.shader.set_uniform_matrix_4_f32_vec("view", &view_matrix);
    material.shader.set_uniform_matrix_4_f32_vec("projection", &projection_matrix);

    // Set other material uniforms (e.g., textures)
    for (name, uniform) in material.uniforms.other.iter() {
      match uniform {
        UniformValue::Float(float) => {
          material.shader.set_uniform_f32(name, *float);
        },
        UniformValue::Vec3(vector3) => {
          material.shader.set_uniform_vector_3_f32(name, vector3);
        },
        UniformValue::Mat4(matrix4) => {
          material.shader.set_uniform_matrix_4_f32_vec(name, matrix4);
        }
      }
    }

    // Bind VAO and draw
    mesh.render_data.vao.bind();
    if let Some(ref indices) = mesh.indices {
      // Indexed drawing
      unsafe {
        gl_call!(
          gl::DrawElements(
            gl::TRIANGLES,
            indices.len() as i32,
            gl::UNSIGNED_INT,
            std::ptr::null(), // null = start of ebo
          )
        );
      }
    } else {
      // Non-indexed drawing
      let vertex_count = (mesh.vertices.data.len() / mesh.vertices.stride) as i32;
      unsafe {
        gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, vertex_count));
      }
    }
  }

  let texture = Texture::new("C:/dev/phoenix/phoenix_engine/assets/textures/goofy.jpg", TextureType::Diffuse);
  texture.activate(0);

  let vertices = RawVertexData {
    data: vec![
      -0.5, -0.5,  0.0,   0.0, 0.0,
      -0.5,  0.5,  0.0,   0.0, 1.0,
      0.5,  0.5,  0.0,   1.0, 1.0,
      0.5, -0.5,  0.0,   1.0, 0.0,
    ],
    stride: 5, // 3 for position + 2 for texcoord
  };
  
  let indices = Some(vec![0, 2, 1, 0, 3, 2]);
  
  let material = Material {
    r#type: MaterialType::Basic,
    shader: ShaderProgram::new("C:/dev/phoenix/phoenix_engine/shaders/vertex.glsl", "C:/dev/phoenix/phoenix_engine/shaders/fragment.glsl"),
    uniforms: UniformCollection {
      matrices: UniformMatrices {
        model: cgmath::Matrix4::identity(),
        view: cgmath::Matrix4::identity(),
        projection: cgmath::Matrix4::identity(),
      },
      other: std::collections::HashMap::new(),
    },
  };

  */

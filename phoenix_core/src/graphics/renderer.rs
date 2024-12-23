use crate::debugger::debugger::Debugger;
use crate::gl_call;
use crate::graphics::camera::*;
use crate::graphics::data::VertexArrayObject;
use crate::graphics::mesh::*;
use crate::scenes::scene::*;
/// Renderer
pub struct Renderer {
  polygon_mode: (gl::types::GLenum, gl::types::GLenum),
  clear_color: [f32; 4]
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {
      polygon_mode: (gl::FRONT_AND_BACK, gl::FILL),
      clear_color: [0.0, 0.0, 0.0, 1.0],
    }
  }

  pub fn from(
    mode: (gl::types::GLenum, gl::types::GLenum),
    clear: [f32; 4]
  ) -> Renderer {
    Renderer {
      polygon_mode: mode,
      clear_color: clear,
    }
  }

  pub fn clear(&self) {
    unsafe {
      gl_call!(gl::ClearColor(self.clear_color[0], self.clear_color[1], self.clear_color[2], self.clear_color[3]));
    }
  }

  pub fn set_clear(&mut self, clear: [f32; 4]) {
    self.clear_color = clear;
  }

  pub fn polygon_mode(&self) {
    unsafe {
      gl_call!(gl::PolygonMode(self.polygon_mode.0, self.polygon_mode.1));
    }
  }

  pub fn set_polygon_mode(&mut self, mode: (gl::types::GLenum, gl::types::GLenum)) {
    self.polygon_mode = mode
  }

  /// Render function
  pub fn render(&self, scene: &Box<Scene>) {
    // TODO:'
    unsafe {
      gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));

      // Move later
      scene.get_shader_program(0).unwrap().bind();
      
      for static_object in scene.static_objects() {
        static_object.mesh.render_data.vao.bind();

        if let Some(ind) = &static_object.mesh.indices {
          gl_call!(gl::DrawElements(gl::TRIANGLES, ind.len() as i32, gl::UNSIGNED_INT, std::ptr::null()));
        } else {
          gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, static_object.mesh.vertices.data.len() as i32));
        }
        
        VertexArrayObject::unbind();
      }
    }
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

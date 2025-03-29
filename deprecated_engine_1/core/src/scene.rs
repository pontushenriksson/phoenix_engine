use gl;
use gl::types;
use image::GenericImageView;
use std::ffi::c_void;

use crate::camera;
use crate::vertices;
use crate::indices;
use crate::shaders;

pub enum ObjectType {
  Obj,
  Glb,
}

pub enum SpriteType {
  Jpg,
  Png,
}

pub enum MeshType {
  Object(ObjectType),
  Sprite(SpriteType),
}

pub enum TextureType {
  Diffuse,
  SpecularMap,
  NormalMap,
  ParallaxMap,
}

pub struct Texture {
  pub id: gl::types::GLuint,
  pub r#type: TextureType,
}

impl Texture {
  pub fn new(file_path: &str, r#type: TextureType) -> Texture {
      let img = image::open(file_path).expect("Failed to load texture");
      let (width, height) = img.dimensions();
      let img_data = img.to_rgba8(); // Convert to RGBA format

      // Flip the image vertically
      let row_size = (width * 4) as usize; // 4 bytes per pixel (RGBA)
      let mut flipped_data = vec![0u8; (width * height * 4) as usize];
      
      for y in 0..height {
          let src_offset = (y * width * 4) as usize;
          let dst_offset = ((height - 1 - y) * width * 4) as usize; // Flipping vertically

          // Correctly copy the row from img_data to flipped_data
          flipped_data[dst_offset..dst_offset + row_size]
              .copy_from_slice(&img_data.as_raw()[src_offset..src_offset + row_size]);
      }

      // Generate and bind a texture in OpenGL
      let mut texture_id: gl::types::GLuint = 0;
      unsafe {
          gl::GenTextures(1, &mut texture_id);
          gl::BindTexture(gl::TEXTURE_2D, texture_id);

          // Set texture parameters
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
          gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

          // Upload flipped texture data to OpenGL
          gl::TexImage2D(
              gl::TEXTURE_2D,
              0,
              gl::RGBA as i32,
              width as i32,
              height as i32,
              0,
              gl::RGBA,
              gl::UNSIGNED_BYTE,
              flipped_data.as_ptr() as *const c_void,
          );
          gl::GenerateMipmap(gl::TEXTURE_2D); // Generate mipmaps
      }

      Texture { id: texture_id,  r#type: r#type }
  }
}

pub struct Mesh {
  pub vbo: Option<vertices::VertexBufferObject>,
  pub indices: Option<indices::IndexBufferObject>,
  pub vao: Option<vertices::VertexArrayObject>,
  pub vertex_attributes: Vec<vertices::VertexAttribute>,
  pub mesh_type: MeshType,
  pub textures: Vec<Texture>,
}

pub struct Transform {
  pub position: cgmath::Vector3<f32>,
  pub rotation: cgmath::Quaternion<f32>,
  pub scale: cgmath::Vector3<f32>,
}

pub struct GameObject {
  pub transform: Transform,
  pub mesh: Option<Mesh>,
  pub shader_program: shaders::ShaderProgram, // Associate a shader with an object
  // pub engine: AnimationEngine,
}

pub struct Character {
  pub model: GameObject,
  pub camera: Option<camera::Camera>,
}

impl GameObject {
  pub fn new(transform: Transform, mesh: Option<Mesh>, shader_program: &shaders::ShaderProgram) -> GameObject {
    // load in files like .glb, .obj, etc.

    GameObject {
      transform: transform,
      mesh: mesh,
      shader_program: &shader_program,
    }
  }

  pub fn attach_script(path: &str) {
    // Implement much later
  }

  pub fn update(&self) {
    // Game logic
  }
}

pub struct Scene {
  objects: Vec<GameObject>,
  cameras: Vec<camera::Camera>, // Can be orthographic or perspective
  // lights:  Vec<light::Light>,
}

/*

impl Scene {
  pub fn new() -> Scene {
    Scene {
      objects: Vec::new(),
      // cameras: Vec![];
      // camera: Camera::orthographic() or ::perspective(),
      // lighting data
    }
  }

  pub fn update(&mut self) {
    // Update all objects in the scene
    for object in &mut self.objects {
      object.update();
    }
  }
}

*/
